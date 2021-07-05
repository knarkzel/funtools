use funtools::Lucas;

#[test]
fn lucas_100() {
    let numbers = Lucas::new().take_while(|x| x < &100).collect::<Vec<_>>();
    assert_eq!(numbers, vec![2, 1, 3, 4, 7, 11, 18, 29, 47, 76]);
}
