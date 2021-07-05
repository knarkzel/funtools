use funtools::Lucas;

#[test]
fn lucas_to_max_100() {
    let numbers = Lucas::new().take_while(|x| x < &100).collect::<Vec<_>>();
    assert_eq!(numbers, vec![3, 4, 7, 11, 18, 29, 47, 76]);
}
