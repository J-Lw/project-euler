// [2](https://projecteuler.net/problem=2).
//
// # p:
// - By considering the terms in the fibonacci sequence whose values do not exceed four million, find the sum of the even-valued terms.
pub fn find_sum() -> i32 {
    let mut current_fn: i32 = 0;
    let mut n1: i32 = 1;
    let mut n2: i32 = 2;
    let mut et_sum: i32 = 2;

    while current_fn < 4000000 {
        current_fn = n1 + n2;

        if current_fn % 2 == 0 {
            et_sum = et_sum + current_fn;   
        }
        
        n1 = n2;
        n2 = current_fn;
    }
    et_sum
}

#[cfg(test)]
mod tests {
    use super::find_sum;

    #[test]
    fn test_find_sum() {
        println!("{}", find_sum());
    }
}
