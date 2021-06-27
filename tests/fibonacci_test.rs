use funtools::Fibonacci;

#[test]
fn fibonacci_to_100() {
    let count = Fibonacci::new().take_while(|x| x < &100).count();
    assert_eq!(count, 10);
}
