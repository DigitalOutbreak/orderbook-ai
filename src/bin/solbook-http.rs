use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use axum::extract::State;
use axum::http::{HeaderValue, Method, StatusCode};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use solbook_core::{
    BookEvent, MarketConfig, NewOrderRequest, OrderBook, Price, Quantity, Side, Trade,
};
use tower_http::cors::CorsLayer;

#[derive(Clone)]
struct AppState {
    inner: Arc<Mutex<EngineState>>,
}

struct EngineState {
    book: OrderBook,
    trades: Vec<ApiTrade>,
}

#[derive(Debug, Serialize)]
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

#[allow(non_snake_case)]
#[derive(Debug, Serialize)]
struct ApiOrderbookStats {
    symbol: String,
    bestBid: f64,
    bestAsk: f64,
    spread: f64,
    midPrice: f64,
    updatedAt: String,
    mode: String,
}

#[derive(Debug, Serialize)]
struct ApiOrderbookSnapshot {
    stats: ApiOrderbookStats,
    bids: Vec<ApiOrderbookLevel>,
    asks: Vec<ApiOrderbookLevel>,
    trades: Vec<ApiOrderbookTrade>,
}

#[derive(Debug, Serialize)]
struct SubmitOrderResponse {
    snapshot: ApiOrderbookSnapshot,
    event: ApiOrderbookEvent,
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
    let state = AppState {
        inner: Arc::new(Mutex::new(EngineState {
            book: OrderBook::new(market),
            trades: Vec::new(),
        })),
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
        .route("/orders", post(submit_order))
        .with_state(state)
        .layer(cors);

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
    Ok(Json(build_snapshot_response(&engine.book, &engine.trades)))
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

    let snapshot = build_snapshot_response(&engine.book, &engine.trades);
    let event = build_event_response(
        &result.events,
        submitted_side,
        submitted_price,
        submitted_qty,
    );

    Ok(Json(SubmitOrderResponse { snapshot, event }))
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

fn build_snapshot_response(book: &OrderBook, trades: &[ApiTrade]) -> ApiOrderbookSnapshot {
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
        bids,
        asks,
        trades: trades.to_vec(),
    }
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
