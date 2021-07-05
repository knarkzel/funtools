use funtools::Tribonacci;

#[test]
fn tribonacci_100() {
    let numbers = Tribonacci::new()
        .take_while(|x| x < &100)
        .collect::<Vec<_>>();
    assert_eq!(numbers, vec![0, 0, 1, 1, 2, 4, 7, 13, 24, 44, 81]);
}
