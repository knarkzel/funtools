# funtools

`funtools` contain a collection of fun iterators to tinker with.
For examples, see the [tests directory](https://git.sr.ht/~knarkzel/funtools/tree/master/item/tests).

```rust
use funtools::Prime;

let first_hundred = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97];
let primes = Prime::new().take_while(|x| x < &100).collect::<Vec<_>>();
assert_eq!(first_hundred, primes);
```

```rust
use funtools::Prime;

let valid = Some(9973); 
let prime = Prime::new().take_while(|x| x < &10000).last();
assert_eq!(valid, prime);
```
