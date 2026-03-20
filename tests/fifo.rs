use rust_decimal_macros::dec;
use solbook_core::{BookEvent, MarketConfig, NewOrderRequest, OrderBook, Price, Quantity, Side};

#[test]
fn same_price_orders_execute_fifo() {
    let config = MarketConfig::sol_usdc();
    let mut book = OrderBook::new(config.clone());

    let first = book.submit_order(NewOrderRequest::limit(
        config.market_id.clone(),
        Side::Sell,
        Quantity::new(dec!(1.000)),
        Price::new(dec!(101.00)),
    ));
    let second = book.submit_order(NewOrderRequest::limit(
        config.market_id.clone(),
        Side::Sell,
        Quantity::new(dec!(1.000)),
        Price::new(dec!(101.00)),
    ));
    let taker = book.submit_order(NewOrderRequest::market(
        config.market_id.clone(),
        Side::Buy,
        Quantity::new(dec!(2.000)),
    ));

    let trade_order_ids: Vec<_> = taker
        .events
        .iter()
        .filter_map(|event| match event {
            BookEvent::TradeExecuted { trade } => Some(trade.maker_order_id),
            _ => None,
        })
        .collect();

    assert_eq!(
        trade_order_ids,
        vec![first.order_id.unwrap(), second.order_id.unwrap()]
    );
}
