use rust_decimal_macros::dec;
use solbook_core::{BookEvent, MarketConfig, NewOrderRequest, OrderBook, Price, Quantity, Side};

fn main() {
    let config = MarketConfig::sol_usdc();
    let mut book = OrderBook::new(config.clone());

    let resting_ask = book.submit_order(NewOrderRequest::limit(
        config.market_id.clone(),
        Side::Sell,
        Quantity::new(dec!(3.000)),
        Price::new(dec!(101.00)),
    ));

    let incoming_buy = book.submit_order(NewOrderRequest::limit(
        config.market_id.clone(),
        Side::Buy,
        Quantity::new(dec!(2.000)),
        Price::new(dec!(102.00)),
    ));

    println!("Resting ask accepted: {}", resting_ask.accepted);
    println!("Incoming buy fully filled: {}", incoming_buy.fully_filled);

    for event in incoming_buy.events {
        match event {
            BookEvent::TradeExecuted { trade } => {
                println!(
                    "trade: qty={} price={} maker={} taker={}",
                    trade.quantity, trade.price, trade.maker_order_id, trade.taker_order_id
                );
            }
            other => println!("{other:?}"),
        }
    }

    println!("top of book: {:?}", book.top_of_book());
    println!("snapshot: {:?}", book.snapshot(5));
}
