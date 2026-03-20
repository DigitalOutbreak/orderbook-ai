use criterion::{Criterion, criterion_group, criterion_main};
use rust_decimal_macros::dec;
use solbook_core::{
    InvariantPolicy, MarketConfig, NewOrderRequest, OrderBook, Price, Quantity, Side,
};
use std::hint::black_box;

fn seed_resting_buy_orders(
    book: &mut OrderBook,
    config: &MarketConfig,
    count: u64,
    single_price: Option<Price>,
) -> Vec<solbook_core::OrderId> {
    let mut order_ids = Vec::with_capacity(count as usize);

    for offset in 0..count {
        let price = single_price.unwrap_or_else(|| {
            Price::new(
                dec!(99.00)
                    + rust_decimal::Decimal::from(offset % 10) / rust_decimal::Decimal::from(100),
            )
        });
        let submission = book.submit_order_minimal(NewOrderRequest::limit(
            config.market_id.clone(),
            Side::Buy,
            Quantity::new(dec!(1.000)),
            price,
        ));
        order_ids.push(submission.order_id.expect("accepted order id"));
    }

    order_ids
}

fn submit_resting_limit_orders(c: &mut Criterion) {
    c.bench_function("submit_resting_limit_orders", |b| {
        b.iter(|| {
            let config = MarketConfig::sol_usdc();
            let mut book = OrderBook::new(config.clone());

            for index in 0..1_000_u64 {
                let price = if index % 2 == 0 {
                    Price::new(dec!(99.00))
                } else {
                    Price::new(dec!(101.00))
                };
                let side = if index % 2 == 0 {
                    Side::Buy
                } else {
                    Side::Sell
                };

                let request = NewOrderRequest::limit(
                    config.market_id.clone(),
                    side,
                    Quantity::new(dec!(1.000)),
                    price,
                );
                black_box(book.submit_order(request));
            }
        });
    });
}

fn submit_resting_limit_orders_minimal(c: &mut Criterion) {
    c.bench_function("submit_resting_limit_orders_minimal", |b| {
        b.iter(|| {
            let config = MarketConfig::sol_usdc();
            let mut book = OrderBook::new(config.clone());

            for index in 0..1_000_u64 {
                let price = if index % 2 == 0 {
                    Price::new(dec!(99.00))
                } else {
                    Price::new(dec!(101.00))
                };
                let side = if index % 2 == 0 {
                    Side::Buy
                } else {
                    Side::Sell
                };

                let request = NewOrderRequest::limit(
                    config.market_id.clone(),
                    side,
                    Quantity::new(dec!(1.000)),
                    price,
                );
                black_box(book.submit_order_minimal(request));
            }
        });
    });
}

fn submit_resting_limit_orders_minimal_no_invariants(c: &mut Criterion) {
    c.bench_function("submit_resting_limit_orders_minimal_no_invariants", |b| {
        b.iter(|| {
            let config = MarketConfig::sol_usdc();
            let mut book = OrderBook::with_invariant_policy(config.clone(), InvariantPolicy::Never);

            for index in 0..1_000_u64 {
                let price = if index % 2 == 0 {
                    Price::new(dec!(99.00))
                } else {
                    Price::new(dec!(101.00))
                };
                let side = if index % 2 == 0 {
                    Side::Buy
                } else {
                    Side::Sell
                };

                let request = NewOrderRequest::limit(
                    config.market_id.clone(),
                    side,
                    Quantity::new(dec!(1.000)),
                    price,
                );
                black_box(book.submit_order_minimal(request));
            }
        });
    });
}

fn cross_seeded_book(c: &mut Criterion) {
    c.bench_function("cross_seeded_book", |b| {
        b.iter(|| {
            let config = MarketConfig::sol_usdc();
            let mut book = OrderBook::new(config.clone());

            for offset in 0..100_u64 {
                let ask_price = Price::new(
                    dec!(100.00)
                        + rust_decimal::Decimal::from(offset) / rust_decimal::Decimal::from(100),
                );
                let bid_price = Price::new(
                    dec!(99.00)
                        - rust_decimal::Decimal::from(offset) / rust_decimal::Decimal::from(100),
                );

                black_box(book.submit_order(NewOrderRequest::limit(
                    config.market_id.clone(),
                    Side::Sell,
                    Quantity::new(dec!(1.000)),
                    ask_price,
                )));
                black_box(book.submit_order(NewOrderRequest::limit(
                    config.market_id.clone(),
                    Side::Buy,
                    Quantity::new(dec!(1.000)),
                    bid_price,
                )));
            }

            black_box(book.submit_order(NewOrderRequest::market(
                config.market_id.clone(),
                Side::Buy,
                Quantity::new(dec!(25.000)),
            )));
        });
    });
}

fn cross_seeded_book_minimal(c: &mut Criterion) {
    c.bench_function("cross_seeded_book_minimal", |b| {
        b.iter(|| {
            let config = MarketConfig::sol_usdc();
            let mut book = OrderBook::new(config.clone());

            for offset in 0..100_u64 {
                let ask_price = Price::new(
                    dec!(100.00)
                        + rust_decimal::Decimal::from(offset) / rust_decimal::Decimal::from(100),
                );
                let bid_price = Price::new(
                    dec!(99.00)
                        - rust_decimal::Decimal::from(offset) / rust_decimal::Decimal::from(100),
                );

                black_box(book.submit_order_minimal(NewOrderRequest::limit(
                    config.market_id.clone(),
                    Side::Sell,
                    Quantity::new(dec!(1.000)),
                    ask_price,
                )));
                black_box(book.submit_order_minimal(NewOrderRequest::limit(
                    config.market_id.clone(),
                    Side::Buy,
                    Quantity::new(dec!(1.000)),
                    bid_price,
                )));
            }

            black_box(book.submit_order_minimal(NewOrderRequest::market(
                config.market_id.clone(),
                Side::Buy,
                Quantity::new(dec!(25.000)),
            )));
        });
    });
}

fn cross_seeded_book_minimal_no_invariants(c: &mut Criterion) {
    c.bench_function("cross_seeded_book_minimal_no_invariants", |b| {
        b.iter(|| {
            let config = MarketConfig::sol_usdc();
            let mut book = OrderBook::with_invariant_policy(config.clone(), InvariantPolicy::Never);

            for offset in 0..100_u64 {
                let ask_price = Price::new(
                    dec!(100.00)
                        + rust_decimal::Decimal::from(offset) / rust_decimal::Decimal::from(100),
                );
                let bid_price = Price::new(
                    dec!(99.00)
                        - rust_decimal::Decimal::from(offset) / rust_decimal::Decimal::from(100),
                );

                black_box(book.submit_order_minimal(NewOrderRequest::limit(
                    config.market_id.clone(),
                    Side::Sell,
                    Quantity::new(dec!(1.000)),
                    ask_price,
                )));
                black_box(book.submit_order_minimal(NewOrderRequest::limit(
                    config.market_id.clone(),
                    Side::Buy,
                    Quantity::new(dec!(1.000)),
                    bid_price,
                )));
            }

            black_box(book.submit_order_minimal(NewOrderRequest::market(
                config.market_id.clone(),
                Side::Buy,
                Quantity::new(dec!(25.000)),
            )));
        });
    });
}

fn cancel_populated_book(c: &mut Criterion) {
    c.bench_function("cancel_populated_book", |b| {
        b.iter(|| {
            let config = MarketConfig::sol_usdc();
            let mut book = OrderBook::new(config.clone());
            let order_ids = seed_resting_buy_orders(&mut book, &config, 500, None);

            for order_id in order_ids.into_iter().rev() {
                black_box(book.cancel_order(order_id));
            }
        });
    });
}

fn cancel_populated_book_minimal(c: &mut Criterion) {
    c.bench_function("cancel_populated_book_minimal", |b| {
        b.iter(|| {
            let config = MarketConfig::sol_usdc();
            let mut book = OrderBook::new(config.clone());
            let order_ids = seed_resting_buy_orders(&mut book, &config, 500, None);

            for order_id in order_ids.into_iter().rev() {
                black_box(book.cancel_order_minimal(order_id));
            }
        });
    });
}

fn cancel_populated_book_minimal_no_invariants(c: &mut Criterion) {
    c.bench_function("cancel_populated_book_minimal_no_invariants", |b| {
        b.iter(|| {
            let config = MarketConfig::sol_usdc();
            let mut book = OrderBook::with_invariant_policy(config.clone(), InvariantPolicy::Never);
            let order_ids = seed_resting_buy_orders(&mut book, &config, 500, None);

            for order_id in order_ids.into_iter().rev() {
                black_box(book.cancel_order_minimal(order_id));
            }
        });
    });
}

fn cancel_deep_single_price_level(c: &mut Criterion) {
    c.bench_function("cancel_deep_single_price_level", |b| {
        b.iter(|| {
            let config = MarketConfig::sol_usdc();
            let mut book = OrderBook::new(config.clone());
            let order_ids =
                seed_resting_buy_orders(&mut book, &config, 1_000, Some(Price::new(dec!(99.00))));

            for order_id in order_ids.into_iter().rev() {
                black_box(book.cancel_order(order_id));
            }
        });
    });
}

fn cancel_deep_single_price_level_minimal(c: &mut Criterion) {
    c.bench_function("cancel_deep_single_price_level_minimal", |b| {
        b.iter(|| {
            let config = MarketConfig::sol_usdc();
            let mut book = OrderBook::new(config.clone());
            let order_ids =
                seed_resting_buy_orders(&mut book, &config, 1_000, Some(Price::new(dec!(99.00))));

            for order_id in order_ids.into_iter().rev() {
                black_box(book.cancel_order_minimal(order_id));
            }
        });
    });
}

fn cancel_deep_single_price_level_minimal_no_invariants(c: &mut Criterion) {
    c.bench_function(
        "cancel_deep_single_price_level_minimal_no_invariants",
        |b| {
            b.iter(|| {
                let config = MarketConfig::sol_usdc();
                let mut book =
                    OrderBook::with_invariant_policy(config.clone(), InvariantPolicy::Never);
                let order_ids = seed_resting_buy_orders(
                    &mut book,
                    &config,
                    1_000,
                    Some(Price::new(dec!(99.00))),
                );

                for order_id in order_ids.into_iter().rev() {
                    black_box(book.cancel_order_minimal(order_id));
                }
            });
        },
    );
}

fn sweep_single_price_fifo(c: &mut Criterion) {
    c.bench_function("sweep_single_price_fifo", |b| {
        b.iter(|| {
            let config = MarketConfig::sol_usdc();
            let mut book = OrderBook::new(config.clone());

            for _ in 0..1_000_u64 {
                black_box(book.submit_order(NewOrderRequest::limit(
                    config.market_id.clone(),
                    Side::Sell,
                    Quantity::new(dec!(1.000)),
                    Price::new(dec!(101.00)),
                )));
            }

            black_box(book.submit_order(NewOrderRequest::market(
                config.market_id.clone(),
                Side::Buy,
                Quantity::new(dec!(1_000.000)),
            )));
        });
    });
}

fn mixed_insert_and_cross_churn(c: &mut Criterion) {
    c.bench_function("mixed_insert_and_cross_churn", |b| {
        b.iter(|| {
            let config = MarketConfig::sol_usdc();
            let mut book = OrderBook::new(config.clone());

            for offset in 0..500_u64 {
                let ask_price = Price::new(
                    dec!(100.00)
                        + rust_decimal::Decimal::from(offset % 20)
                            / rust_decimal::Decimal::from(100),
                );

                black_box(book.submit_order(NewOrderRequest::limit(
                    config.market_id.clone(),
                    Side::Sell,
                    Quantity::new(dec!(1.000)),
                    ask_price,
                )));

                if offset % 2 == 0 {
                    black_box(book.submit_order(NewOrderRequest::limit(
                        config.market_id.clone(),
                        Side::Buy,
                        Quantity::new(dec!(1.000)),
                        Price::new(dec!(100.25)),
                    )));
                }
            }
        });
    });
}

criterion_group!(
    benches,
    submit_resting_limit_orders,
    submit_resting_limit_orders_minimal,
    submit_resting_limit_orders_minimal_no_invariants,
    cross_seeded_book,
    cross_seeded_book_minimal,
    cross_seeded_book_minimal_no_invariants,
    cancel_populated_book,
    cancel_populated_book_minimal,
    cancel_populated_book_minimal_no_invariants,
    cancel_deep_single_price_level,
    cancel_deep_single_price_level_minimal,
    cancel_deep_single_price_level_minimal_no_invariants,
    sweep_single_price_fifo,
    mixed_insert_and_cross_churn
);
criterion_main!(benches);
