use funtools::Leonardo;

#[test]
fn leonardo_100() {
    let numbers = Leonardo::new().take_while(|x| x < &100).collect::<Vec<_>>();
    assert_eq!(numbers, vec![1, 1, 3, 5, 9, 15, 25, 41, 67]);
}
