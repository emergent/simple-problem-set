fn fizzbuzz(i: u32) -> String {
    if i % 3 == 0 && i % 5 == 0 {
        "FizzBuzz".into()
    } else if i % 3 == 0 {
        "Fizz".into()
    } else if i % 5 == 0 {
        "Buzz".into()
    } else if i % 7 == 0 {
        "Pop".into()
    } else {
        format!("{i}")
    }
}

fn main() {
    let range = 1..=100;

    for i in range {
        let result = fizzbuzz(i);
        println!("{result}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fizzbuzz() {
        assert_eq!(fizzbuzz(1), "1".to_string());
        assert_eq!(fizzbuzz(2), "2".to_string());
        assert_eq!(fizzbuzz(3), "Fizz".to_string());
        assert_eq!(fizzbuzz(4), "4".to_string());
        assert_eq!(fizzbuzz(5), "Buzz".to_string());
        assert_eq!(fizzbuzz(6), "Fizz".to_string());
        assert_eq!(fizzbuzz(7), "Pop".to_string());
        assert_eq!(fizzbuzz(8), "8".to_string());
        assert_eq!(fizzbuzz(9), "Fizz".to_string());
        assert_eq!(fizzbuzz(10), "Buzz".to_string());
        assert_eq!(fizzbuzz(15), "FizzBuzz".to_string());
        assert_eq!(fizzbuzz(21), "Fizz".to_string());
        assert_eq!(fizzbuzz(35), "Buzz".to_string());
        assert_eq!(fizzbuzz(42), "Fizz".to_string());
        assert_eq!(fizzbuzz(49), "Pop".to_string());
        assert_eq!(fizzbuzz(50), "Buzz".to_string());
        assert_eq!(fizzbuzz(105), "FizzBuzz".to_string());
    }
}
