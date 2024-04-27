use std::collections::HashMap;

pub fn parse_vars(file: &str) -> HashMap<String, String> {
    let mut map: HashMap<String, String> = HashMap::new();

    for line in file.lines() {
        if !line.starts_with("@") {
            continue;
        }

        let Some((lhs, rhs)) = line.split_once("=") else {
            println!(
                "a variable declaration line doesn't follow the @name = value format, skipping it"
            );
            continue;
        };

        let mut lhs = lhs.trim().to_owned();
        lhs.remove(0);
        let rhs = rhs.trim().to_owned();

        map.insert(lhs, rhs);
    }

    return map;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vars_are_parsed_correctly() {
        assert_eq!(
            parse_vars("@a = example.com"),
            HashMap::from([("a".to_string(), "example.com".to_string())])
        )
    }
}
