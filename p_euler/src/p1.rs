/// [1](https://projecteuler.net/problem=1).
///
/// # p:
/// - Find the sum of all multiples of 3 or 5 below 1000.
use std::collections::HashSet;

pub fn find_sum() -> i32 {
    let mut multiples = HashSet::new();
    let mut n1: i32 = 3;
    let mut n2: i32 = 5;

    while n1 < 1000 {
        multiples.insert(n1);
        n1 += 3;  
    }

    while n2 < 1000 {
        multiples.insert(n2);
        n2 += 5;
    }

    multiples.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::find_sum;

    #[test]
    fn test_find_sum() {
        println!("{}", find_sum());
    }
}
