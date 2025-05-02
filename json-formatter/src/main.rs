const INDENT_LENGTH: usize = 2;

fn format(json: &str) -> String {
    let mut result = String::new();
    let mut indent = 0;
    let mut in_string = false;
    let mut after_colon = false;

    for c in json.chars() {
        match c {
            '{' | '[' if !in_string => {
                result.push(c);
                indent += 1;
                result.push('\n');
                result.push_str(&" ".repeat(indent * INDENT_LENGTH));
            }
            '}' | ']' if !in_string => {
                indent -= 1;
                result.push('\n');
                result.push_str(&" ".repeat(indent * INDENT_LENGTH));
                result.push(c);
            }
            ',' if !in_string => {
                result.push(c);
                result.push('\n');
                result.push_str(&" ".repeat(indent * INDENT_LENGTH));
            }
            '"' => {
                in_string = !in_string;
                result.push(c);
            }
            ':' if !in_string => {
                result.push(c);
                result.push(' ');
                after_colon = true;
            }
            ' ' if after_colon => {
                // Skip spaces after colon
                continue;
            }
            _ => {
                result.push(c);
            }
        }
    }

    result
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <json_string>", args[0]);
        std::process::exit(1);
    }

    let json = &args[1];
    let formatted_json = format(json);
    println!("{}", formatted_json);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format() {
        let json = r#"{"key1":"value1","key2":{"key3":"value3","key4":[1,2,3]}}"#;
        let expected = r#"{
  "key1": "value1",
  "key2": {
    "key3": "value3",
    "key4": [
      1,
      2,
      3
    ]
  }
}"#;
        assert_eq!(format(json), expected);
    }
}
