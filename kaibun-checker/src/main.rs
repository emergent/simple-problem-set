fn kaibun_checker(s: &str) -> bool {
    let s = s.to_lowercase();
    let len = s.chars().count(); // Count the number of characters
    if len <= 1 {
        return true;
    }
    // Check if the string is a palindrome
    for i in 0..len / 2 {
        let c1 = s.chars().nth(i).unwrap();
        let c2 = s.chars().nth(len - 1 - i).unwrap();
        if c1 != c2 {
            return false;
        }
    }
    true
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        eprintln!("Usage: {} <string>", args[0]);
        std::process::exit(1);
    }

    let input = &args[1];
    if kaibun_checker(input) {
        println!("The string \"{}\" is a palindrome.", input);
    } else {
        println!("The string \"{}\" is not a palindrome.", input);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kaibun_checker() {
        assert_eq!(kaibun_checker(""), true);
        assert_eq!(kaibun_checker("a"), true);
        assert_eq!(kaibun_checker("aa"), true);
        assert_eq!(kaibun_checker("ab"), false);
        assert_eq!(kaibun_checker("aba"), true);
        assert_eq!(kaibun_checker("abc"), false);
        assert_eq!(kaibun_checker("Aba"), true);
        assert_eq!(kaibun_checker("AbaCaba"), true);
        assert_eq!(kaibun_checker("AbaCabaDabaCaba"), true);
    }

    #[test]
    fn test_kaibun_checker_japanese() {
        assert_eq!(kaibun_checker(""), true);
        assert_eq!(kaibun_checker("あ"), true);
        assert_eq!(kaibun_checker("ああ"), true);
        assert_eq!(kaibun_checker("あああ"), true);
        assert_eq!(kaibun_checker("あいうえお"), false);

        assert_eq!(kaibun_checker("しんぶんし"), true);
        assert_eq!(kaibun_checker("しんぶんしゃ"), false);
    }
}
