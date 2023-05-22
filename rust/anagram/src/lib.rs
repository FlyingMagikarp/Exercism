use std::collections::HashSet;
use std::char;

const PRIMES: [i64; 26] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71,
    73, 79, 83, 89, 97, 101,
];

fn map_string_to_prime_product(input: &str) -> i64 {
    input.chars()
        .map(char::to_lowercase)
        .map(|c| (c as u8) - b'a')
        .map(|cb| PRIMES[cb as usize])
    .product();


    return 1;
}


pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&str]) -> HashSet<&'a str> {
    let mut result: HashSet<&'a str> = HashSet::new();

    let tmp = PrimeMapper::A.value().to_string();
    let tmpa = "A";
    tmp = PrimeMapper::tmpa

    return result;
}
