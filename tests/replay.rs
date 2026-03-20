mod support;

use support::{ScenarioOutcome, generated_scenario, reference_scenario, replay};

#[test]
fn replayed_scenario_is_deterministic_across_runs() {
    let scenario = reference_scenario();

    let first = replay(&scenario);
    let second = replay(&scenario);

    assert_eq!(first, second);
}

#[test]
fn replay_fixture_produces_expected_event_shapes() {
    let scenario = reference_scenario();
    let (outcomes, snapshot) = replay(&scenario);

    assert_eq!(snapshot.asks.len(), 1);
    assert_eq!(snapshot.bids.len(), 0);
    assert!(matches!(
        &outcomes[3],
        ScenarioOutcome::Submission(result) if result.fully_filled && result.events.len() >= 3
    ));
    assert!(matches!(
        &outcomes[4],
        ScenarioOutcome::Cancellation(result) if result.cancelled
    ));
}

#[test]
fn generated_scenario_is_deterministic_for_seeded_mixed_flow() {
    let scenario = generated_scenario(42, 64);

    let first = replay(&scenario);
    let second = replay(&scenario);

    assert_eq!(first, second);
}
