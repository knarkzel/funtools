use funtools::Prime;

#[test]
fn primes_to_100() {
    let valid = vec![
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
        97,
    ];
    let primes = Prime::new().take_while(|x| x < &100).collect::<Vec<_>>();
    assert_eq!(valid, primes);
}

#[test]
fn last_prime_number_10000() {
    let valid = Some(9973); 
    let prime = Prime::new().take_while(|x| x < &10000).last();
    assert_eq!(valid, prime);
}

#[test]
fn ten_primes_after_9000() {
    let valid = vec![9001, 9007, 9011, 9013, 9029, 9041, 9043, 9049, 9059, 9067];
    let primes = Prime::new().skip_while(|x| x < &9000).take(10).collect::<Vec<_>>();
    assert_eq!(valid, primes);
}
