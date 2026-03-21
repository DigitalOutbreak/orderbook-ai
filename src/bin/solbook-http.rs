use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use async_stream::stream;
use axum::extract::State;
use axum::http::{HeaderValue, Method, StatusCode};
use axum::response::IntoResponse;
use axum::response::sse::{Event, KeepAlive, Sse};
use axum::routing::{get, post};
use axum::{Json, Router};
use futures_util::stream::Stream;
use serde::{Deserialize, Serialize};
use solbook_core::{
    BookEvent, MarketConfig, NewOrderRequest, OrderBook, Price, Quantity, Side, Trade,
};
use tokio::sync::broadcast;
use tower_http::cors::CorsLayer;

#[derive(Clone)]
struct AppState {
    inner: Arc<Mutex<EngineState>>,
    broadcaster: broadcast::Sender<EngineStreamPayload>,
}

struct EngineState {
    book: OrderBook,
    trades: Vec<ApiTrade>,
    simulation: ApiSimulationState,
    sim_reference_price: f64,
    rng_state: u64,
    regime_ticks_remaining: u32,
    aggression_imbalance: f64,
    active_impulse: Option<SimulationImpulse>,
}

#[derive(Debug, Clone, Copy)]
enum SimulationScenario {
    Balanced,
    TrendUp,
    TrendDown,
    Range,
    BidWall,
    AskWall,
    ThinLiquidity,
}

#[derive(Debug, Clone, Copy)]
enum SimulationRegime {
    Balanced,
    BuyPressure,
    SellPressure,
    TrendUp,
    TrendDown,
    Pullback,
    MeanRevert,
    Thin,
    Refill,
}

#[derive(Debug, Clone, Copy)]
struct SimulationImpulse {
    direction: f64,
    strength: f64,
    decay: f64,
    remaining_ticks: u32,
}

#[derive(Debug, Clone, Copy)]
enum SimulationAction {
    CrossBuy,
    CrossSell,
    RestBid,
    RestAsk,
}

#[derive(Debug, Serialize, Clone)]
struct ApiOrderbookEvent {
    id: String,
    time: String,
    tone: String,
    title: String,
    detail: String,
}

#[derive(Debug, Serialize, Clone)]
struct ApiOrderbookLevel {
    id: String,
    side: String,
    price: f64,
    size: f64,
    total: f64,
}

#[derive(Debug, Serialize, Clone)]
struct ApiOrderbookTrade {
    id: String,
    side: String,
    price: f64,
    size: f64,
    time: String,
    venue: String,
}

#[derive(Debug, Serialize, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
enum ApiSimulationSpeed {
    Slow,
    Normal,
    Fast,
    Burst,
}

#[derive(Debug, Serialize, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
enum ApiSimulationScenario {
    Balanced,
    TrendUp,
    TrendDown,
    Range,
    BidWall,
    AskWall,
    ThinLiquidity,
}

#[derive(Debug, Serialize, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
enum ApiSimulationRegime {
    Balanced,
    BuyPressure,
    SellPressure,
    TrendUp,
    TrendDown,
    Pullback,
    MeanRevert,
    Thin,
    Refill,
}

#[derive(Debug, Serialize, Clone)]
struct ApiSimulationState {
    active: bool,
    speed: ApiSimulationSpeed,
    scenario: ApiSimulationScenario,
    regime: ApiSimulationRegime,
    reference_price: f64,
    aggression_imbalance: f64,
    volatility_state: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Clone)]
struct ApiOrderbookStats {
    symbol: String,
    bestBid: f64,
    bestAsk: f64,
    spread: f64,
    midPrice: f64,
    updatedAt: String,
    mode: String,
}

#[derive(Debug, Serialize, Clone)]
struct ApiOrderbookSnapshot {
    stats: ApiOrderbookStats,
    simulation: ApiSimulationState,
    bids: Vec<ApiOrderbookLevel>,
    asks: Vec<ApiOrderbookLevel>,
    trades: Vec<ApiOrderbookTrade>,
}

#[derive(Debug, Serialize)]
struct SubmitOrderResponse {
    snapshot: ApiOrderbookSnapshot,
    event: ApiOrderbookEvent,
}

#[derive(Debug, Serialize)]
struct ScenarioResponse {
    snapshot: ApiOrderbookSnapshot,
    event: ApiOrderbookEvent,
}

#[derive(Debug, Serialize, Clone)]
struct EngineStreamPayload {
    snapshot: ApiOrderbookSnapshot,
    event: Option<ApiOrderbookEvent>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
struct SubmitOrderRequest {
    side: String,
    orderType: String,
    quantity: String,
    price: Option<String>,
}

#[derive(Debug, Serialize)]
struct ApiError {
    error: String,
}

type ApiTrade = ApiOrderbookTrade;

#[tokio::main]
async fn main() {
    let market = MarketConfig::sol_usdc();
    let (broadcaster, _) = broadcast::channel(64);
    let state = AppState {
        inner: Arc::new(Mutex::new(build_engine_state(
            market,
            SimulationScenario::Balanced,
        ))),
        broadcaster,
    };

    let cors = CorsLayer::new()
        .allow_origin([
            HeaderValue::from_static("http://localhost:3000"),
            HeaderValue::from_static("http://127.0.0.1:3000"),
        ])
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(tower_http::cors::Any);

    let app = Router::new()
        .route("/health", get(health))
        .route("/snapshot", get(get_snapshot))
        .route("/stream", get(stream_updates))
        .route("/orders", post(submit_order))
        .route("/reset", post(reset_book))
        .route("/scenarios/{scenario_id}", post(seed_scenario))
        .route("/simulate/cross-buy", post(simulate_cross_buy))
        .route("/simulate/cross-sell", post(simulate_cross_sell))
        .route("/simulation/start", post(start_simulation))
        .route("/simulation/stop", post(stop_simulation))
        .route("/simulation/speed/{speed}", post(set_simulation_speed))
        .with_state(state.clone())
        .layer(cors);

    tokio::spawn(run_market_simulation_loop(state.clone()));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("solbook-http listening on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("bind api listener");
    axum::serve(listener, app).await.expect("run api server");
}

async fn health() -> impl IntoResponse {
    StatusCode::NO_CONTENT
}

async fn get_snapshot(
    State(state): State<AppState>,
) -> Result<Json<ApiOrderbookSnapshot>, ApiResponseError> {
    let engine = state.inner.lock().expect("engine state lock poisoned");
    Ok(Json(build_snapshot_response(
        &engine.book,
        &engine.trades,
        &engine.simulation,
    )))
}

async fn stream_updates(
    State(state): State<AppState>,
) -> Sse<impl Stream<Item = Result<Event, std::convert::Infallible>>> {
    let mut rx = state.broadcaster.subscribe();

    let stream = stream! {
        loop {
            match rx.recv().await {
                Ok(payload) => {
                    if let Ok(json) = serde_json::to_string(&payload) {
                        yield Ok(Event::default().event("engine-update").data(json));
                    }
                }
                Err(broadcast::error::RecvError::Lagged(_)) => continue,
                Err(broadcast::error::RecvError::Closed) => break,
            }
        }
    };

    Sse::new(stream).keep_alive(KeepAlive::default())
}

async fn submit_order(
    State(state): State<AppState>,
    Json(payload): Json<SubmitOrderRequest>,
) -> Result<Json<SubmitOrderResponse>, ApiResponseError> {
    let mut engine = state.inner.lock().expect("engine state lock poisoned");
    let request = build_new_order_request(engine.book.market_config(), payload)?;
    let submitted_price = request.price;
    let submitted_qty = request.quantity;
    let submitted_side = request.side;
    let result = engine.book.submit_order(request);

    for trade in result.events.iter().filter_map(extract_trade) {
        engine.trades.insert(0, map_trade(trade));
    }
    engine.trades.truncate(24);

    let snapshot = build_snapshot_response(&engine.book, &engine.trades, &engine.simulation);
    let event = build_event_response(
        &result.events,
        submitted_side,
        submitted_price,
        submitted_qty,
    );
    let _ = state.broadcaster.send(EngineStreamPayload {
        snapshot: snapshot.clone(),
        event: Some(event.clone()),
    });

    Ok(Json(SubmitOrderResponse { snapshot, event }))
}

async fn reset_book(
    State(state): State<AppState>,
) -> Result<Json<ScenarioResponse>, ApiResponseError> {
    let mut engine = state.inner.lock().expect("engine state lock poisoned");
    *engine = build_engine_state(
        engine.book.market_config().clone(),
        SimulationScenario::Balanced,
    );
    let snapshot = build_snapshot_response(&engine.book, &engine.trades, &engine.simulation);
    let event = build_scenario_event(
        "Book reset",
        "Balanced ladder restored with seeded bid and ask liquidity.",
    );
    let _ = state.broadcaster.send(EngineStreamPayload {
        snapshot: snapshot.clone(),
        event: Some(event.clone()),
    });

    Ok(Json(ScenarioResponse { snapshot, event }))
}

async fn seed_scenario(
    State(state): State<AppState>,
    axum::extract::Path(scenario_id): axum::extract::Path<String>,
) -> Result<Json<ScenarioResponse>, ApiResponseError> {
    let scenario = match scenario_id.as_str() {
        "balanced" => SimulationScenario::Balanced,
        "trend-up" => SimulationScenario::TrendUp,
        "trend-down" => SimulationScenario::TrendDown,
        "range" => SimulationScenario::Range,
        "bid-wall" => SimulationScenario::BidWall,
        "ask-wall" => SimulationScenario::AskWall,
        "thin-liquidity" => SimulationScenario::ThinLiquidity,
        _ => {
            return Err(ApiResponseError::bad_request(format!(
                "unknown scenario '{}'",
                scenario_id
            )));
        }
    };

    let mut engine = state.inner.lock().expect("engine state lock poisoned");
    let detail = if engine.simulation.active {
        apply_live_scenario_switch(&mut engine, scenario);
        match scenario {
            SimulationScenario::Balanced => {
                "Simulation behavior switched to a balanced two-sided market."
            }
            SimulationScenario::TrendUp => {
                "Simulation behavior switched to an upward trend without resetting the book."
            }
            SimulationScenario::TrendDown => {
                "Simulation behavior switched to a downward trend without resetting the book."
            }
            SimulationScenario::Range => {
                "Simulation behavior switched to a range-bound market without resetting the book."
            }
            SimulationScenario::BidWall => {
                "Simulation behavior switched to bid-wall support without resetting the book."
            }
            SimulationScenario::AskWall => {
                "Simulation behavior switched to ask-wall pressure without resetting the book."
            }
            SimulationScenario::ThinLiquidity => {
                "Simulation behavior switched to thin liquidity without resetting the book."
            }
        }
    } else {
        let market = engine.book.market_config().clone();
        let was_active = engine.simulation.active;
        let current_speed = engine.simulation.speed;

        *engine = build_engine_state(market, scenario);
        engine.simulation.active = was_active;
        engine.simulation.speed = current_speed;

        match scenario {
            SimulationScenario::Balanced => "Balanced ladder restored.",
            SimulationScenario::TrendUp => "Uptrend regime seeded with stronger bid support.",
            SimulationScenario::TrendDown => "Downtrend regime seeded with heavier ask pressure.",
            SimulationScenario::Range => "Range-bound regime seeded around a stable midpoint.",
            SimulationScenario::BidWall => "Heavy bid wall seeded beneath the touch.",
            SimulationScenario::AskWall => "Heavy ask wall seeded above the touch.",
            SimulationScenario::ThinLiquidity => {
                "Thin liquidity seeded with wider gaps and lighter depth."
            }
        }
    };

    let snapshot = build_snapshot_response(&engine.book, &engine.trades, &engine.simulation);
    let event = build_scenario_event("Scenario loaded", detail);
    let _ = state.broadcaster.send(EngineStreamPayload {
        snapshot: snapshot.clone(),
        event: Some(event.clone()),
    });

    Ok(Json(ScenarioResponse { snapshot, event }))
}

async fn simulate_cross_buy(
    State(state): State<AppState>,
) -> Result<Json<ScenarioResponse>, ApiResponseError> {
    simulate_crossing_order(state, Side::Buy)
}

async fn simulate_cross_sell(
    State(state): State<AppState>,
) -> Result<Json<ScenarioResponse>, ApiResponseError> {
    simulate_crossing_order(state, Side::Sell)
}

fn simulate_crossing_order(
    state: AppState,
    side: Side,
) -> Result<Json<ScenarioResponse>, ApiResponseError> {
    let mut engine = state.inner.lock().expect("engine state lock poisoned");
    let market_id = engine.book.market_config().market_id.clone();
    let target_price = match side {
        Side::Buy => engine
            .book
            .best_ask()
            .map(|level| level.price)
            .ok_or_else(|| {
                ApiResponseError::bad_request("no ask liquidity available".to_string())
            })?,
        Side::Sell => engine
            .book
            .best_bid()
            .map(|level| level.price)
            .ok_or_else(|| {
                ApiResponseError::bad_request("no bid liquidity available".to_string())
            })?,
    };
    let target_quantity = parse_seed_quantity("8.00");

    let result = engine.book.submit_order(NewOrderRequest::limit(
        market_id,
        side,
        target_quantity,
        target_price,
    ));

    for trade in result.events.iter().filter_map(extract_trade) {
        engine.trades.insert(0, map_trade(trade));
    }
    engine.trades.truncate(24);

    let snapshot = build_snapshot_response(&engine.book, &engine.trades, &engine.simulation);
    let event = build_simulation_event(side, target_quantity, target_price, &result.events);
    let _ = state.broadcaster.send(EngineStreamPayload {
        snapshot: snapshot.clone(),
        event: Some(event.clone()),
    });

    Ok(Json(ScenarioResponse { snapshot, event }))
}

async fn start_simulation(
    State(state): State<AppState>,
) -> Result<Json<ScenarioResponse>, ApiResponseError> {
    let mut engine = state.inner.lock().expect("engine state lock poisoned");
    engine.simulation.active = true;
    let snapshot = build_snapshot_response(&engine.book, &engine.trades, &engine.simulation);
    let event = build_scenario_event(
        "Market simulation on",
        "Rust-driven synthetic order flow is now updating the book and chart.",
    );
    let _ = state.broadcaster.send(EngineStreamPayload {
        snapshot: snapshot.clone(),
        event: Some(event.clone()),
    });

    Ok(Json(ScenarioResponse { snapshot, event }))
}

async fn stop_simulation(
    State(state): State<AppState>,
) -> Result<Json<ScenarioResponse>, ApiResponseError> {
    let mut engine = state.inner.lock().expect("engine state lock poisoned");
    engine.simulation.active = false;
    let snapshot = build_snapshot_response(&engine.book, &engine.trades, &engine.simulation);
    let event = build_scenario_event(
        "Market simulation off",
        "Synthetic order flow is paused. Manual orders still work normally.",
    );
    let _ = state.broadcaster.send(EngineStreamPayload {
        snapshot: snapshot.clone(),
        event: Some(event.clone()),
    });

    Ok(Json(ScenarioResponse { snapshot, event }))
}

async fn set_simulation_speed(
    State(state): State<AppState>,
    axum::extract::Path(speed): axum::extract::Path<String>,
) -> Result<Json<ScenarioResponse>, ApiResponseError> {
    let next_speed = match speed.as_str() {
        "slow" => ApiSimulationSpeed::Slow,
        "normal" => ApiSimulationSpeed::Normal,
        "fast" => ApiSimulationSpeed::Fast,
        "burst" => ApiSimulationSpeed::Burst,
        _ => {
            return Err(ApiResponseError::bad_request(format!(
                "unknown simulation speed '{}'",
                speed
            )));
        }
    };

    let mut engine = state.inner.lock().expect("engine state lock poisoned");
    engine.simulation.speed = next_speed;
    let snapshot = build_snapshot_response(&engine.book, &engine.trades, &engine.simulation);
    let event = build_scenario_event(
        "Simulation speed changed",
        match next_speed {
            ApiSimulationSpeed::Slow => "Market simulation is now running at a slow study pace.",
            ApiSimulationSpeed::Normal => "Market simulation is now running at a normal pace.",
            ApiSimulationSpeed::Fast => "Market simulation is now running at a fast pace.",
            ApiSimulationSpeed::Burst => "Market simulation is now running at a burst pace.",
        },
    );
    let _ = state.broadcaster.send(EngineStreamPayload {
        snapshot: snapshot.clone(),
        event: Some(event.clone()),
    });

    Ok(Json(ScenarioResponse { snapshot, event }))
}

fn build_new_order_request(
    market: &MarketConfig,
    payload: SubmitOrderRequest,
) -> Result<NewOrderRequest, ApiResponseError> {
    let side = match payload.side.as_str() {
        "buy" => Side::Buy,
        "sell" => Side::Sell,
        _ => {
            return Err(ApiResponseError::bad_request(
                "side must be 'buy' or 'sell'".to_string(),
            ));
        }
    };

    if payload.orderType != "limit" {
        return Err(ApiResponseError::bad_request(
            "only limit orders are currently supported".to_string(),
        ));
    }

    let quantity = parse_quantity(&payload.quantity)?;
    let price =
        parse_price(payload.price.as_deref().ok_or_else(|| {
            ApiResponseError::bad_request("limit price is required".to_string())
        })?)?;

    Ok(NewOrderRequest::limit(
        market.market_id.clone(),
        side,
        quantity,
        price,
    ))
}

fn parse_price(value: &str) -> Result<Price, ApiResponseError> {
    let parsed = value
        .parse()
        .map_err(|_| ApiResponseError::bad_request("invalid price".to_string()))?;
    Ok(Price::new(parsed))
}

fn parse_quantity(value: &str) -> Result<Quantity, ApiResponseError> {
    let parsed = value
        .parse()
        .map_err(|_| ApiResponseError::bad_request("invalid quantity".to_string()))?;
    Ok(Quantity::new(parsed))
}

fn build_snapshot_response(
    book: &OrderBook,
    trades: &[ApiTrade],
    simulation: &ApiSimulationState,
) -> ApiOrderbookSnapshot {
    let snapshot = book.snapshot(32);
    let bids = build_levels(snapshot.bids, "bid");
    let asks = build_levels(snapshot.asks, "ask");
    let best_bid = bids.first().map(|level| level.price).unwrap_or(0.0);
    let best_ask = asks.first().map(|level| level.price).unwrap_or(0.0);
    let spread = if best_bid > 0.0 && best_ask > 0.0 {
        round2(best_ask - best_bid)
    } else {
        0.0
    };
    let mid_price = if best_bid > 0.0 && best_ask > 0.0 {
        round2((best_bid + best_ask) / 2.0)
    } else {
        0.0
    };

    ApiOrderbookSnapshot {
        stats: ApiOrderbookStats {
            symbol: book.market_config().market_id.as_str().to_string(),
            bestBid: best_bid,
            bestAsk: best_ask,
            spread,
            midPrice: mid_price,
            updatedAt: format_time_label(),
            mode: "live".to_string(),
        },
        simulation: simulation.clone(),
        bids,
        asks,
        trades: trades.to_vec(),
    }
}

fn build_engine_state(market: MarketConfig, scenario: SimulationScenario) -> EngineState {
    let mut book = OrderBook::new(market.clone());

    seed_scenario_book(&mut book, scenario);
    let snapshot = book.snapshot(1);
    let best_bid = snapshot
        .bids
        .first()
        .map(|level| decimal_to_f64(level.price))
        .unwrap_or(171.98);
    let best_ask = snapshot
        .asks
        .first()
        .map(|level| decimal_to_f64(level.price))
        .unwrap_or(172.02);
    let seed = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos() as u64;

    EngineState {
        book,
        trades: Vec::new(),
        simulation: ApiSimulationState {
            active: false,
            speed: ApiSimulationSpeed::Normal,
            scenario: map_scenario(scenario),
            regime: map_regime(initial_regime_for_scenario(scenario)),
            reference_price: round2((best_bid + best_ask) / 2.0),
            aggression_imbalance: 0.0,
            volatility_state: initial_volatility_label(scenario).to_string(),
        },
        sim_reference_price: round2((best_bid + best_ask) / 2.0),
        rng_state: seed.max(1),
        regime_ticks_remaining: 0,
        aggression_imbalance: 0.0,
        active_impulse: None,
    }
}

fn apply_live_scenario_switch(engine: &mut EngineState, scenario: SimulationScenario) {
    engine.simulation.scenario = map_scenario(scenario);
    engine.simulation.regime = map_regime(initial_regime_for_scenario(scenario));
    engine.simulation.volatility_state = initial_volatility_label(scenario).to_string();
    engine.regime_ticks_remaining = regime_duration_ticks(engine);
    engine.active_impulse = None;
    engine.aggression_imbalance = 0.0;
    engine.simulation.aggression_imbalance = 0.0;

    let snapshot = engine.book.snapshot(1);
    let best_bid = snapshot
        .bids
        .first()
        .map(|level| decimal_to_f64(level.price));
    let best_ask = snapshot
        .asks
        .first()
        .map(|level| decimal_to_f64(level.price));

    if let (Some(bid), Some(ask)) = (best_bid, best_ask) {
        let reference_price = round2((bid + ask) / 2.0);
        engine.sim_reference_price = reference_price;
        engine.simulation.reference_price = reference_price;
    }
}

fn seed_scenario_book(book: &mut OrderBook, scenario: SimulationScenario) {
    match scenario {
        SimulationScenario::Balanced => {
            seed_levels(
                book,
                Side::Buy,
                &[
                    ("171.98", "42.18"),
                    ("171.95", "38.72"),
                    ("171.92", "34.61"),
                    ("171.90", "28.44"),
                    ("171.87", "25.91"),
                    ("171.84", "21.67"),
                ],
            );
            seed_levels(
                book,
                Side::Sell,
                &[
                    ("172.02", "31.11"),
                    ("172.05", "33.48"),
                    ("172.08", "36.92"),
                    ("172.10", "28.84"),
                    ("172.13", "24.51"),
                    ("172.16", "22.70"),
                ],
            );
        }
        SimulationScenario::TrendUp => {
            seed_levels(
                book,
                Side::Buy,
                &[
                    ("171.98", "28.00"),
                    ("171.95", "34.00"),
                    ("171.92", "41.00"),
                    ("171.88", "46.00"),
                    ("171.84", "38.00"),
                ],
            );
            seed_levels(
                book,
                Side::Sell,
                &[
                    ("172.02", "18.00"),
                    ("172.05", "17.00"),
                    ("172.09", "15.00"),
                    ("172.13", "13.00"),
                ],
            );
        }
        SimulationScenario::TrendDown => {
            seed_levels(
                book,
                Side::Buy,
                &[
                    ("171.98", "17.00"),
                    ("171.95", "16.00"),
                    ("171.91", "14.00"),
                    ("171.87", "12.00"),
                ],
            );
            seed_levels(
                book,
                Side::Sell,
                &[
                    ("172.02", "29.00"),
                    ("172.05", "36.00"),
                    ("172.08", "42.00"),
                    ("172.12", "49.00"),
                    ("172.16", "38.00"),
                ],
            );
        }
        SimulationScenario::Range => {
            seed_levels(
                book,
                Side::Buy,
                &[
                    ("171.98", "22.00"),
                    ("171.95", "24.00"),
                    ("171.92", "26.00"),
                    ("171.89", "24.00"),
                    ("171.86", "22.00"),
                ],
            );
            seed_levels(
                book,
                Side::Sell,
                &[
                    ("172.02", "22.00"),
                    ("172.05", "24.00"),
                    ("172.08", "26.00"),
                    ("172.11", "24.00"),
                    ("172.14", "22.00"),
                ],
            );
        }
        SimulationScenario::BidWall => {
            seed_levels(
                book,
                Side::Buy,
                &[
                    ("171.99", "18.25"),
                    ("171.97", "42.00"),
                    ("171.95", "105.00"),
                    ("171.92", "210.00"),
                    ("171.89", "88.00"),
                    ("171.85", "41.50"),
                ],
            );
            seed_levels(
                book,
                Side::Sell,
                &[
                    ("172.01", "14.80"),
                    ("172.04", "16.40"),
                    ("172.07", "19.20"),
                    ("172.11", "21.00"),
                    ("172.15", "18.30"),
                ],
            );
        }
        SimulationScenario::AskWall => {
            seed_levels(
                book,
                Side::Buy,
                &[
                    ("171.99", "15.10"),
                    ("171.96", "17.80"),
                    ("171.93", "18.50"),
                    ("171.89", "16.25"),
                    ("171.86", "13.90"),
                ],
            );
            seed_levels(
                book,
                Side::Sell,
                &[
                    ("172.01", "22.50"),
                    ("172.03", "48.00"),
                    ("172.05", "116.00"),
                    ("172.08", "230.00"),
                    ("172.11", "94.00"),
                    ("172.14", "44.75"),
                ],
            );
        }
        SimulationScenario::ThinLiquidity => {
            seed_levels(
                book,
                Side::Buy,
                &[
                    ("171.99", "5.50"),
                    ("171.95", "4.20"),
                    ("171.90", "3.40"),
                    ("171.82", "2.80"),
                ],
            );
            seed_levels(
                book,
                Side::Sell,
                &[
                    ("172.01", "5.20"),
                    ("172.05", "4.10"),
                    ("172.10", "3.10"),
                    ("172.18", "2.60"),
                ],
            );
        }
    }
}

fn seed_levels(book: &mut OrderBook, side: Side, levels: &[(&str, &str)]) {
    let market_id = book.market_config().market_id.clone();

    for (price, quantity) in levels {
        let request = NewOrderRequest::limit(
            market_id.clone(),
            side,
            parse_seed_quantity(quantity),
            parse_seed_price(price),
        );
        let _ = book.submit_order(request);
    }
}

fn parse_seed_price(value: &str) -> Price {
    Price::new(value.parse().expect("seed price should parse"))
}

fn parse_seed_quantity(value: &str) -> Quantity {
    Quantity::new(value.parse().expect("seed quantity should parse"))
}

fn build_levels(levels: Vec<solbook_core::BookLevelView>, side: &str) -> Vec<ApiOrderbookLevel> {
    let mut running_total = 0.0;

    levels
        .into_iter()
        .enumerate()
        .map(|(index, level)| {
            let size = decimal_to_f64(level.total_quantity);
            running_total += size;

            ApiOrderbookLevel {
                id: format!("{}-{}", side, index),
                side: side.to_string(),
                price: decimal_to_f64(level.price),
                size,
                total: round2(running_total),
            }
        })
        .collect()
}

fn build_event_response(
    events: &[BookEvent],
    side: Side,
    submitted_price: Option<Price>,
    submitted_qty: Quantity,
) -> ApiOrderbookEvent {
    let title_side = match side {
        Side::Buy => "Buy",
        Side::Sell => "Sell",
    };
    let tone = match side {
        Side::Buy => "buy",
        Side::Sell => "sell",
    };

    if let Some(error) = events.iter().find_map(|event| match event {
        BookEvent::OrderRejected { error } => Some(error.to_string()),
        _ => None,
    }) {
        return ApiOrderbookEvent {
            id: format!("event-{}", format_time_id()),
            time: format_clock_time(),
            tone: "neutral".to_string(),
            title: "Order rejected".to_string(),
            detail: error,
        };
    }

    let trades: Vec<&Trade> = events.iter().filter_map(extract_trade).collect();
    if !trades.is_empty() {
        let executed_qty = round2(
            trades
                .iter()
                .map(|trade| decimal_to_f64(trade.quantity))
                .sum::<f64>(),
        );
        let last_price = trades
            .last()
            .map(|trade| decimal_to_f64(trade.price))
            .unwrap_or_else(|| submitted_price.map(decimal_to_f64).unwrap_or(0.0));
        let remaining = round2(decimal_to_f64(submitted_qty) - executed_qty);
        let suffix = if remaining > 0.0 {
            match submitted_price {
                Some(price) => format!(
                    ", {} rested at {}",
                    format_number(remaining),
                    format_number(decimal_to_f64(price))
                ),
                None => String::new(),
            }
        } else {
            String::new()
        };

        return ApiOrderbookEvent {
            id: format!("event-{}", format_time_id()),
            time: format_clock_time(),
            tone: tone.to_string(),
            title: format!("{} crossed the spread", title_side),
            detail: format!(
                "Executed {} @ {}{}.",
                format_number(executed_qty),
                format_number(last_price),
                suffix
            ),
        };
    }

    let price_text = submitted_price
        .map(|price| format_number(decimal_to_f64(price)))
        .unwrap_or_else(|| "--".to_string());

    ApiOrderbookEvent {
        id: format!("event-{}", format_time_id()),
        time: format_clock_time(),
        tone: tone.to_string(),
        title: format!("{} rested on the book", title_side),
        detail: format!(
            "Added {} @ {} to the {} side.",
            format_number(decimal_to_f64(submitted_qty)),
            price_text,
            if side == Side::Buy { "bid" } else { "ask" }
        ),
    }
}

fn build_scenario_event(title: &str, detail: &str) -> ApiOrderbookEvent {
    ApiOrderbookEvent {
        id: format!("event-{}", format_time_id()),
        time: format_clock_time(),
        tone: "neutral".to_string(),
        title: title.to_string(),
        detail: detail.to_string(),
    }
}

fn build_simulation_event(
    side: Side,
    quantity: Quantity,
    price: Price,
    events: &[BookEvent],
) -> ApiOrderbookEvent {
    let (title, tone) = match side {
        Side::Buy => ("Cross Buy", "buy"),
        Side::Sell => ("Cross Sell", "sell"),
    };
    let traded = events
        .iter()
        .any(|event| matches!(event, BookEvent::TradeExecuted { .. }));

    ApiOrderbookEvent {
        id: format!("event-{}", format_time_id()),
        time: format_clock_time(),
        tone: tone.to_string(),
        title: title.to_string(),
        detail: if traded {
            format!(
                "{} executed {} at {} and removed resting liquidity.",
                title,
                format_number(decimal_to_f64(quantity)),
                format_number(decimal_to_f64(price))
            )
        } else {
            format!(
                "{} posted {} at {} without finding opposite liquidity.",
                title,
                format_number(decimal_to_f64(quantity)),
                format_number(decimal_to_f64(price))
            )
        },
    }
}

async fn run_market_simulation_loop(state: AppState) {
    loop {
        let maybe_payload = {
            let mut engine = state.inner.lock().expect("engine state lock poisoned");

            if !engine.simulation.active {
                None
            } else {
                let event = apply_simulation_tick(&mut engine);
                let snapshot =
                    build_snapshot_response(&engine.book, &engine.trades, &engine.simulation);
                Some(EngineStreamPayload {
                    snapshot,
                    event: Some(event),
                })
            }
        };

        if let Some(payload) = maybe_payload {
            let _ = state.broadcaster.send(payload);
        }

        tokio::time::sleep(simulation_interval(&state)).await;
    }
}

fn simulation_interval(state: &AppState) -> std::time::Duration {
    let engine = state.inner.lock().expect("engine state lock poisoned");

    if !engine.simulation.active {
        return std::time::Duration::from_millis(250);
    }

    match engine.simulation.speed {
        ApiSimulationSpeed::Slow => std::time::Duration::from_millis(1500),
        ApiSimulationSpeed::Normal => std::time::Duration::from_millis(900),
        ApiSimulationSpeed::Fast => std::time::Duration::from_millis(180),
        ApiSimulationSpeed::Burst => std::time::Duration::from_millis(70),
    }
}

fn apply_simulation_tick(engine: &mut EngineState) -> ApiOrderbookEvent {
    refresh_regime(engine);
    maybe_start_impulse(engine);
    evolve_reference_price(engine);
    ensure_two_sided_liquidity(engine);
    let event = match choose_simulation_action(engine) {
        SimulationAction::CrossBuy => simulate_market_cross(engine, Side::Buy),
        SimulationAction::CrossSell => simulate_market_cross(engine, Side::Sell),
        SimulationAction::RestBid => simulate_resting_order(engine, Side::Buy),
        SimulationAction::RestAsk => simulate_resting_order(engine, Side::Sell),
    };

    ensure_two_sided_liquidity(engine);
    engine.aggression_imbalance = (engine.aggression_imbalance * 0.86).clamp(-1.0, 1.0);
    engine.simulation.regime = map_regime(current_regime(engine));
    engine.simulation.reference_price = round2(engine.sim_reference_price);
    engine.simulation.aggression_imbalance = round2(engine.aggression_imbalance);
    engine.simulation.volatility_state = current_volatility_label(engine).to_string();
    event
}

fn simulate_market_cross(engine: &mut EngineState, side: Side) -> ApiOrderbookEvent {
    let market_id = engine.book.market_config().market_id.clone();
    let target_price = match side {
        Side::Buy => engine
            .book
            .best_ask()
            .map(|level| level.price)
            .unwrap_or_else(|| parse_seed_price("172.02")),
        Side::Sell => engine
            .book
            .best_bid()
            .map(|level| level.price)
            .unwrap_or_else(|| parse_seed_price("171.98")),
    };
    let target_quantity = parse_seed_quantity(match engine.simulation.speed {
        ApiSimulationSpeed::Slow => "1.25",
        ApiSimulationSpeed::Normal => "2.50",
        ApiSimulationSpeed::Fast => "4.75",
        ApiSimulationSpeed::Burst => "7.50",
    });
    let result = engine.book.submit_order(NewOrderRequest::limit(
        market_id,
        side,
        target_quantity,
        target_price,
    ));

    for trade in result.events.iter().filter_map(extract_trade) {
        engine.trades.insert(0, map_trade(trade));
    }
    engine.trades.truncate(24);
    let impulse_side = match side {
        Side::Buy => 1.0,
        Side::Sell => -1.0,
    };
    engine.aggression_imbalance =
        (engine.aggression_imbalance + impulse_side * 0.22).clamp(-1.0, 1.0);

    build_simulation_event(side, target_quantity, target_price, &result.events)
}

fn simulate_resting_order(engine: &mut EngineState, side: Side) -> ApiOrderbookEvent {
    let spread_ticks = match engine.simulation.speed {
        ApiSimulationSpeed::Slow => 0.02,
        ApiSimulationSpeed::Normal => 0.02,
        ApiSimulationSpeed::Fast => 0.03,
        ApiSimulationSpeed::Burst => 0.04,
    };
    let book_offset = random_range(engine, 0.0, 0.08);
    let base_price = match side {
        Side::Buy => (engine.sim_reference_price - spread_ticks - book_offset).max(0.01),
        Side::Sell => engine.sim_reference_price + spread_ticks + book_offset,
    };
    let target_price = price_from_f64(base_price);
    let target_quantity = quantity_from_f64(match engine.simulation.speed {
        ApiSimulationSpeed::Slow => random_range(engine, 1.0, 2.4),
        ApiSimulationSpeed::Normal => random_range(engine, 2.0, 5.5),
        ApiSimulationSpeed::Fast => random_range(engine, 3.5, 8.5),
        ApiSimulationSpeed::Burst => random_range(engine, 6.0, 13.0),
    });
    let market_id = engine.book.market_config().market_id.clone();
    let result = engine.book.submit_order(NewOrderRequest::limit(
        market_id,
        side,
        target_quantity,
        target_price,
    ));

    for trade in result.events.iter().filter_map(extract_trade) {
        engine.trades.insert(0, map_trade(trade));
    }
    engine.trades.truncate(24);
    let resting_side = match side {
        Side::Buy => 1.0,
        Side::Sell => -1.0,
    };
    engine.aggression_imbalance =
        (engine.aggression_imbalance + resting_side * 0.08).clamp(-1.0, 1.0);

    ApiOrderbookEvent {
        id: format!("event-{}", format_time_id()),
        time: format_clock_time(),
        tone: match side {
            Side::Buy => "buy".to_string(),
            Side::Sell => "sell".to_string(),
        },
        title: match side {
            Side::Buy => "Sim bid added".to_string(),
            Side::Sell => "Sim ask added".to_string(),
        },
        detail: format!(
            "Simulator added {} @ {} on the {} side.",
            format_number(decimal_to_f64(target_quantity)),
            format_number(decimal_to_f64(target_price)),
            if side == Side::Buy { "bid" } else { "ask" }
        ),
    }
}

fn ensure_two_sided_liquidity(engine: &mut EngineState) {
    let best_bid = engine.book.best_bid().map(|level| level.price);
    let best_ask = engine.book.best_ask().map(|level| level.price);

    if best_bid.is_none() {
        seed_emergency_levels(
            engine,
            Side::Buy,
            (engine.sim_reference_price - 0.03).max(0.01),
        );
    }

    if best_ask.is_none() {
        seed_emergency_levels(engine, Side::Sell, engine.sim_reference_price + 0.03);
    }
}

fn seed_emergency_levels(engine: &mut EngineState, side: Side, start_price: f64) {
    let market_id = engine.book.market_config().market_id.clone();
    let offsets = match side {
        Side::Buy => [0.00, -0.03, -0.06],
        Side::Sell => [0.00, 0.03, 0.06],
    };

    for (index, offset) in offsets.into_iter().enumerate() {
        let price_value = (start_price + offset).max(0.01);
        let quantity = match index {
            0 => "9.00",
            1 => "6.00",
            _ => "4.00",
        };
        let _ = engine.book.submit_order(NewOrderRequest::limit(
            market_id.clone(),
            side,
            parse_seed_quantity(quantity),
            price_from_f64(price_value),
        ));
    }
}

fn evolve_reference_price(engine: &mut EngineState) {
    let noise = random_signed(engine) * noise_scale(engine);
    let regime_bias = regime_bias(engine);
    let continuation = engine.aggression_imbalance * continuation_scale(engine);
    let impulse = current_impulse_effect(engine);
    let mean_reversion = scenario_mean_reversion(engine);

    engine.sim_reference_price = round2(
        (engine.sim_reference_price
            + noise
            + regime_bias
            + continuation
            + impulse
            + mean_reversion)
            .max(0.05),
    );
}

fn refresh_regime(engine: &mut EngineState) {
    if engine.regime_ticks_remaining > 0 {
        engine.regime_ticks_remaining -= 1;
        return;
    }

    let next_regime = match current_scenario(engine) {
        SimulationScenario::Balanced => {
            let roll = random_unit(engine);
            if roll < 0.45 {
                SimulationRegime::Balanced
            } else if roll < 0.65 {
                SimulationRegime::BuyPressure
            } else if roll < 0.85 {
                SimulationRegime::SellPressure
            } else {
                SimulationRegime::MeanRevert
            }
        }
        SimulationScenario::TrendUp => {
            let roll = random_unit(engine);
            if roll < 0.55 {
                SimulationRegime::TrendUp
            } else if roll < 0.8 {
                SimulationRegime::BuyPressure
            } else {
                SimulationRegime::Pullback
            }
        }
        SimulationScenario::TrendDown => {
            let roll = random_unit(engine);
            if roll < 0.55 {
                SimulationRegime::TrendDown
            } else if roll < 0.8 {
                SimulationRegime::SellPressure
            } else {
                SimulationRegime::Pullback
            }
        }
        SimulationScenario::Range => {
            let roll = random_unit(engine);
            if roll < 0.6 {
                SimulationRegime::MeanRevert
            } else if roll < 0.85 {
                SimulationRegime::Balanced
            } else {
                SimulationRegime::Pullback
            }
        }
        SimulationScenario::BidWall => {
            let roll = random_unit(engine);
            if roll < 0.55 {
                SimulationRegime::BuyPressure
            } else if roll < 0.82 {
                SimulationRegime::Refill
            } else {
                SimulationRegime::Balanced
            }
        }
        SimulationScenario::AskWall => {
            let roll = random_unit(engine);
            if roll < 0.55 {
                SimulationRegime::SellPressure
            } else if roll < 0.82 {
                SimulationRegime::Refill
            } else {
                SimulationRegime::Balanced
            }
        }
        SimulationScenario::ThinLiquidity => {
            let roll = random_unit(engine);
            if roll < 0.45 {
                SimulationRegime::Thin
            } else if roll < 0.75 {
                SimulationRegime::Balanced
            } else {
                SimulationRegime::Refill
            }
        }
    };

    engine.simulation.regime = map_regime(next_regime);
    engine.regime_ticks_remaining = regime_duration_ticks(engine);
}

fn maybe_start_impulse(engine: &mut EngineState) {
    if let Some(impulse) = engine.active_impulse.as_mut() {
        if impulse.remaining_ticks > 0 {
            impulse.remaining_ticks -= 1;
            impulse.strength *= impulse.decay;
        }
        if impulse.remaining_ticks == 0 || impulse.strength.abs() < 0.001 {
            engine.active_impulse = None;
        }
        return;
    }

    let chance = match current_scenario(engine) {
        SimulationScenario::Balanced => 0.01,
        SimulationScenario::TrendUp | SimulationScenario::TrendDown => 0.018,
        SimulationScenario::Range => 0.007,
        SimulationScenario::BidWall | SimulationScenario::AskWall => 0.012,
        SimulationScenario::ThinLiquidity => 0.03,
    };

    if random_unit(engine) >= chance {
        return;
    }

    let direction = match current_scenario(engine) {
        SimulationScenario::TrendUp => 1.0,
        SimulationScenario::TrendDown => -1.0,
        SimulationScenario::BidWall => {
            if random_unit(engine) < 0.72 {
                1.0
            } else {
                -1.0
            }
        }
        SimulationScenario::AskWall => {
            if random_unit(engine) < 0.72 {
                -1.0
            } else {
                1.0
            }
        }
        _ => {
            if random_unit(engine) < 0.5 {
                -1.0
            } else {
                1.0
            }
        }
    };

    engine.active_impulse = Some(SimulationImpulse {
        direction,
        strength: match current_scenario(engine) {
            SimulationScenario::Balanced => random_range(engine, 0.02, 0.06),
            SimulationScenario::TrendUp | SimulationScenario::TrendDown => {
                random_range(engine, 0.03, 0.08)
            }
            SimulationScenario::Range => random_range(engine, 0.015, 0.04),
            SimulationScenario::BidWall | SimulationScenario::AskWall => {
                random_range(engine, 0.025, 0.07)
            }
            SimulationScenario::ThinLiquidity => random_range(engine, 0.05, 0.14),
        },
        decay: random_range(engine, 0.72, 0.88),
        remaining_ticks: match engine.simulation.speed {
            ApiSimulationSpeed::Slow => 4,
            ApiSimulationSpeed::Normal => 5,
            ApiSimulationSpeed::Fast => 6,
            ApiSimulationSpeed::Burst => 8,
        },
    });
}

fn choose_simulation_action(engine: &mut EngineState) -> SimulationAction {
    let (cross_buy, cross_sell, rest_bid, _rest_ask) = match current_regime(engine) {
        SimulationRegime::Balanced => (0.18, 0.18, 0.32, 0.32),
        SimulationRegime::BuyPressure => (0.28, 0.10, 0.42, 0.20),
        SimulationRegime::SellPressure => (0.10, 0.28, 0.20, 0.42),
        SimulationRegime::TrendUp => (0.24, 0.12, 0.42, 0.22),
        SimulationRegime::TrendDown => (0.12, 0.24, 0.22, 0.42),
        SimulationRegime::Pullback => (0.16, 0.16, 0.34, 0.34),
        SimulationRegime::MeanRevert => (0.16, 0.16, 0.34, 0.34),
        SimulationRegime::Thin => (0.24, 0.24, 0.26, 0.26),
        SimulationRegime::Refill => (0.08, 0.08, 0.42, 0.42),
    };
    let roll = random_unit(engine);

    if roll < cross_buy {
        SimulationAction::CrossBuy
    } else if roll < cross_buy + cross_sell {
        SimulationAction::CrossSell
    } else if roll < cross_buy + cross_sell + rest_bid {
        SimulationAction::RestBid
    } else {
        SimulationAction::RestAsk
    }
}

fn noise_scale(engine: &mut EngineState) -> f64 {
    match current_scenario(engine) {
        SimulationScenario::Balanced => 0.004,
        SimulationScenario::TrendUp | SimulationScenario::TrendDown => 0.006,
        SimulationScenario::Range => 0.003,
        SimulationScenario::BidWall | SimulationScenario::AskWall => 0.005,
        SimulationScenario::ThinLiquidity => 0.012,
    }
}

fn regime_bias(engine: &EngineState) -> f64 {
    match current_regime(engine) {
        SimulationRegime::Balanced => 0.0,
        SimulationRegime::BuyPressure => 0.01,
        SimulationRegime::SellPressure => -0.01,
        SimulationRegime::TrendUp => 0.018,
        SimulationRegime::TrendDown => -0.018,
        SimulationRegime::Pullback => {
            if matches!(current_scenario(engine), SimulationScenario::TrendUp) {
                -0.01
            } else if matches!(current_scenario(engine), SimulationScenario::TrendDown) {
                0.01
            } else {
                0.0
            }
        }
        SimulationRegime::MeanRevert => 0.0,
        SimulationRegime::Thin => 0.0,
        SimulationRegime::Refill => 0.0,
    }
}

fn continuation_scale(engine: &EngineState) -> f64 {
    match engine.simulation.speed {
        ApiSimulationSpeed::Slow => 0.004,
        ApiSimulationSpeed::Normal => 0.006,
        ApiSimulationSpeed::Fast => 0.008,
        ApiSimulationSpeed::Burst => 0.01,
    }
}

fn current_impulse_effect(engine: &EngineState) -> f64 {
    engine
        .active_impulse
        .map(|impulse| impulse.direction * impulse.strength)
        .unwrap_or(0.0)
}

fn scenario_mean_reversion(engine: &EngineState) -> f64 {
    let anchor = 172.0;
    let gap = anchor - engine.sim_reference_price;

    match current_scenario(engine) {
        SimulationScenario::Balanced => gap * 0.012,
        SimulationScenario::Range => gap * 0.03,
        SimulationScenario::BidWall | SimulationScenario::AskWall => gap * 0.016,
        SimulationScenario::ThinLiquidity => gap * 0.004,
        SimulationScenario::TrendUp | SimulationScenario::TrendDown => gap * 0.002,
    }
}

fn current_volatility_label(engine: &EngineState) -> &'static str {
    if matches!(current_scenario(engine), SimulationScenario::ThinLiquidity) {
        return "high";
    }
    if engine.active_impulse.is_some() {
        return "elevated";
    }

    match engine.simulation.speed {
        ApiSimulationSpeed::Slow => "low",
        ApiSimulationSpeed::Normal => "medium",
        ApiSimulationSpeed::Fast => "high",
        ApiSimulationSpeed::Burst => "extreme",
    }
}

fn regime_duration_ticks(engine: &mut EngineState) -> u32 {
    match current_scenario(engine) {
        SimulationScenario::Balanced => random_range(engine, 18.0, 34.0).round() as u32,
        SimulationScenario::TrendUp | SimulationScenario::TrendDown => {
            random_range(engine, 24.0, 48.0).round() as u32
        }
        SimulationScenario::Range => random_range(engine, 20.0, 40.0).round() as u32,
        SimulationScenario::BidWall | SimulationScenario::AskWall => {
            random_range(engine, 16.0, 30.0).round() as u32
        }
        SimulationScenario::ThinLiquidity => random_range(engine, 12.0, 24.0).round() as u32,
    }
}

fn initial_regime_for_scenario(scenario: SimulationScenario) -> SimulationRegime {
    match scenario {
        SimulationScenario::Balanced => SimulationRegime::Balanced,
        SimulationScenario::TrendUp => SimulationRegime::TrendUp,
        SimulationScenario::TrendDown => SimulationRegime::TrendDown,
        SimulationScenario::Range => SimulationRegime::MeanRevert,
        SimulationScenario::BidWall => SimulationRegime::BuyPressure,
        SimulationScenario::AskWall => SimulationRegime::SellPressure,
        SimulationScenario::ThinLiquidity => SimulationRegime::Thin,
    }
}

fn initial_volatility_label(scenario: SimulationScenario) -> &'static str {
    match scenario {
        SimulationScenario::ThinLiquidity => "high",
        SimulationScenario::TrendUp | SimulationScenario::TrendDown => "medium",
        _ => "low",
    }
}

fn current_scenario(engine: &EngineState) -> SimulationScenario {
    match engine.simulation.scenario {
        ApiSimulationScenario::Balanced => SimulationScenario::Balanced,
        ApiSimulationScenario::TrendUp => SimulationScenario::TrendUp,
        ApiSimulationScenario::TrendDown => SimulationScenario::TrendDown,
        ApiSimulationScenario::Range => SimulationScenario::Range,
        ApiSimulationScenario::BidWall => SimulationScenario::BidWall,
        ApiSimulationScenario::AskWall => SimulationScenario::AskWall,
        ApiSimulationScenario::ThinLiquidity => SimulationScenario::ThinLiquidity,
    }
}

fn current_regime(engine: &EngineState) -> SimulationRegime {
    match engine.simulation.regime {
        ApiSimulationRegime::Balanced => SimulationRegime::Balanced,
        ApiSimulationRegime::BuyPressure => SimulationRegime::BuyPressure,
        ApiSimulationRegime::SellPressure => SimulationRegime::SellPressure,
        ApiSimulationRegime::TrendUp => SimulationRegime::TrendUp,
        ApiSimulationRegime::TrendDown => SimulationRegime::TrendDown,
        ApiSimulationRegime::Pullback => SimulationRegime::Pullback,
        ApiSimulationRegime::MeanRevert => SimulationRegime::MeanRevert,
        ApiSimulationRegime::Thin => SimulationRegime::Thin,
        ApiSimulationRegime::Refill => SimulationRegime::Refill,
    }
}

fn map_scenario(scenario: SimulationScenario) -> ApiSimulationScenario {
    match scenario {
        SimulationScenario::Balanced => ApiSimulationScenario::Balanced,
        SimulationScenario::TrendUp => ApiSimulationScenario::TrendUp,
        SimulationScenario::TrendDown => ApiSimulationScenario::TrendDown,
        SimulationScenario::Range => ApiSimulationScenario::Range,
        SimulationScenario::BidWall => ApiSimulationScenario::BidWall,
        SimulationScenario::AskWall => ApiSimulationScenario::AskWall,
        SimulationScenario::ThinLiquidity => ApiSimulationScenario::ThinLiquidity,
    }
}

fn map_regime(regime: SimulationRegime) -> ApiSimulationRegime {
    match regime {
        SimulationRegime::Balanced => ApiSimulationRegime::Balanced,
        SimulationRegime::BuyPressure => ApiSimulationRegime::BuyPressure,
        SimulationRegime::SellPressure => ApiSimulationRegime::SellPressure,
        SimulationRegime::TrendUp => ApiSimulationRegime::TrendUp,
        SimulationRegime::TrendDown => ApiSimulationRegime::TrendDown,
        SimulationRegime::Pullback => ApiSimulationRegime::Pullback,
        SimulationRegime::MeanRevert => ApiSimulationRegime::MeanRevert,
        SimulationRegime::Thin => ApiSimulationRegime::Thin,
        SimulationRegime::Refill => ApiSimulationRegime::Refill,
    }
}

fn random_unit(engine: &mut EngineState) -> f64 {
    let mut x = engine.rng_state;
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    engine.rng_state = x.max(1);
    ((engine.rng_state % 10_000) as f64) / 10_000.0
}

fn random_signed(engine: &mut EngineState) -> f64 {
    random_unit(engine) * 2.0 - 1.0
}

fn random_range(engine: &mut EngineState, min: f64, max: f64) -> f64 {
    min + (max - min) * random_unit(engine)
}

fn price_from_f64(value: f64) -> Price {
    Price::new(
        round2(value)
            .to_string()
            .parse()
            .expect("price should parse"),
    )
}

fn quantity_from_f64(value: f64) -> Quantity {
    Quantity::new(
        round2(value)
            .to_string()
            .parse()
            .expect("quantity should parse"),
    )
}

fn extract_trade(event: &BookEvent) -> Option<&Trade> {
    match event {
        BookEvent::TradeExecuted { trade } => Some(trade),
        _ => None,
    }
}

fn map_trade(trade: &Trade) -> ApiTrade {
    ApiOrderbookTrade {
        id: format!(
            "trade-{}-{}",
            trade.maker_order_id.value(),
            trade.taker_order_id.value()
        ),
        side: match trade.taker_side {
            Side::Buy => "buy".to_string(),
            Side::Sell => "sell".to_string(),
        },
        price: decimal_to_f64(trade.price),
        size: decimal_to_f64(trade.quantity),
        time: format_clock_time(),
        venue: "ENGINE".to_string(),
    }
}

fn decimal_to_f64<T: std::fmt::Display>(value: T) -> f64 {
    value.to_string().parse::<f64>().unwrap_or(0.0)
}

fn round2(value: f64) -> f64 {
    (value * 100.0).round() / 100.0
}

fn format_number(value: f64) -> String {
    format!("{value:.2}")
}

fn format_clock_time() -> String {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    let millis = now.as_millis() % 1_000;
    let secs = now.as_secs() % 86_400;
    let hour = secs / 3_600;
    let minute = (secs % 3_600) / 60;
    let second = secs % 60;
    format!("{hour:02}:{minute:02}:{second:02}.{millis:03}")
}

fn format_time_label() -> String {
    format!("{} UTC", format_clock_time())
}

fn format_time_id() -> String {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
        .to_string()
}

struct ApiResponseError {
    status: StatusCode,
    message: String,
}

impl ApiResponseError {
    fn bad_request(message: String) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            message,
        }
    }
}

impl IntoResponse for ApiResponseError {
    fn into_response(self) -> axum::response::Response {
        (
            self.status,
            Json(ApiError {
                error: self.message,
            }),
        )
            .into_response()
    }
}
