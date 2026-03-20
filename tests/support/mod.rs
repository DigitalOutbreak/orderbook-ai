use rust_decimal_macros::dec;
use solbook_core::{
    CancelResult, MarketConfig, NewOrderRequest, OrderBook, Price, Quantity, Side, SubmissionResult,
};

#[derive(Debug, Clone)]
pub enum ScenarioStep {
    Submit(NewOrderRequest),
    CancelLastAccepted(usize),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScenarioOutcome {
    Submission(SubmissionResult),
    Cancellation(CancelResult),
}

pub fn replay(steps: &[ScenarioStep]) -> (Vec<ScenarioOutcome>, solbook_core::BookSnapshot) {
    let config = MarketConfig::sol_usdc();
    let mut book = OrderBook::new(config);
    let mut accepted_ids = Vec::new();
    let mut outcomes = Vec::new();

    for step in steps {
        match step {
            ScenarioStep::Submit(request) => {
                let result = book.submit_order(request.clone());
                if let Some(order_id) = result.order_id {
                    accepted_ids.push(order_id);
                }
                outcomes.push(ScenarioOutcome::Submission(result));
            }
            ScenarioStep::CancelLastAccepted(index_from_end) => {
                let target = accepted_ids[accepted_ids.len() - 1 - index_from_end];
                outcomes.push(ScenarioOutcome::Cancellation(book.cancel_order(target)));
            }
        }
    }

    (outcomes, book.snapshot(10))
}

pub fn reference_scenario() -> Vec<ScenarioStep> {
    let market_id = MarketConfig::sol_usdc().market_id;

    vec![
        ScenarioStep::Submit(NewOrderRequest::limit(
            market_id.clone(),
            Side::Sell,
            Quantity::new(dec!(2.000)),
            Price::new(dec!(101.00)),
        )),
        ScenarioStep::Submit(NewOrderRequest::limit(
            market_id.clone(),
            Side::Sell,
            Quantity::new(dec!(1.500)),
            Price::new(dec!(102.00)),
        )),
        ScenarioStep::Submit(NewOrderRequest::limit(
            market_id.clone(),
            Side::Buy,
            Quantity::new(dec!(1.000)),
            Price::new(dec!(100.00)),
        )),
        ScenarioStep::Submit(NewOrderRequest::market(
            market_id.clone(),
            Side::Buy,
            Quantity::new(dec!(2.250)),
        )),
        ScenarioStep::CancelLastAccepted(1),
    ]
}

pub fn generated_scenario(seed: u64, steps: usize) -> Vec<ScenarioStep> {
    let market_id = MarketConfig::sol_usdc().market_id;
    let mut generator = Lcg::new(seed);
    let mut accepted_submissions = 0usize;
    let mut scenario = Vec::with_capacity(steps);

    for _ in 0..steps {
        let choose_cancel = accepted_submissions > 0 && generator.next_u64().is_multiple_of(5);
        if choose_cancel {
            let index_from_end = (generator.next_u64() as usize) % accepted_submissions;
            scenario.push(ScenarioStep::CancelLastAccepted(index_from_end));
            continue;
        }

        let side = if generator.next_u64().is_multiple_of(2) {
            Side::Buy
        } else {
            Side::Sell
        };
        let order_type = generator.next_u64() % 4;
        let quantity_steps = 1 + (generator.next_u64() % 5);
        let quantity = Quantity::new(
            dec!(1.000) + dec!(0.500) * rust_decimal::Decimal::from(quantity_steps - 1),
        );

        let request = if order_type == 0 {
            NewOrderRequest::market(market_id.clone(), side, quantity)
        } else {
            let price_offset = rust_decimal::Decimal::from(generator.next_u64() % 8)
                / rust_decimal::Decimal::from(100);
            let base_price = match side {
                Side::Buy => dec!(99.50),
                Side::Sell => dec!(100.50),
            };
            NewOrderRequest::limit(
                market_id.clone(),
                side,
                quantity,
                Price::new(base_price + price_offset),
            )
        };

        accepted_submissions += 1;
        scenario.push(ScenarioStep::Submit(request));
    }

    scenario
}

#[derive(Debug, Clone)]
struct Lcg {
    state: u64,
}

impl Lcg {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next_u64(&mut self) -> u64 {
        self.state = self
            .state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.state
    }
}
