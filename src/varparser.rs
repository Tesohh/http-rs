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

        map.insert(lhs.trim().to_owned(), rhs.trim().to_owned());
    }

    return map;
}
