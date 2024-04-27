use std::collections::HashMap;

pub fn replace_vars(file: String, vars: &HashMap<String, String>) -> String {
    file.lines()
        .map(|line| {
            let re = regex::Regex::new(r"\{\{(\w+)\}\}").unwrap();
            re.replace_all(line, |captures: &regex::Captures| {
                // Replace the variable with its value
                match vars.get(&captures[1]) {
                    Some(value) => value.to_string(),
                    None => captures[0].to_string(),
                }
                .to_string()
            })
            .to_string()
        })
        .collect::<Vec<String>>()
        .join("\n")
}
