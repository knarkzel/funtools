use funtools::Fibonacci;

#[test]
fn fibonacci_100() {
    let numbers = Fibonacci::new()
        .take_while(|x| x < &100)
        .collect::<Vec<_>>();
    assert_eq!(numbers, vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89]);
}
