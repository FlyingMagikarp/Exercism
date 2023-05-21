use clock::Clock;

#[test]
fn test_negative_minutes() {
    assert_eq!(Clock::new(1, -100).to_string(), "00:20");
}