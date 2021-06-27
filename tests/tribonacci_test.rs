use funtools::Tribonacci;

#[test]
fn tribonacci_100() {
    let value = Tribonacci::new().take_while(|x| x < &100).last();
    assert_eq!(value, Some(81));
}
