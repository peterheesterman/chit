use std::io::Read;

#[derive(Serialize, Deserialize)]
pub struct AlternativesSet {
    pub name: String,
    pub alternatives: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Alternatives {
    pub description: String,
    pub sets: Vec<AlternativesSet>,
}

const ALTERNATIVES_URL: &str =
    "https://raw.githubusercontent.com/peterheesterman/chit/master/alternatives.json";

pub fn get_alternatives() -> Alternatives {
    let mut res = reqwest::get(ALTERNATIVES_URL)
        .expect("Failed to connect to github to retrieve alternatives");

    let mut res_body = String::new();

    //TODO: add a match expression here
    res.read_to_string(&mut res_body)
        .expect("Failed to fetch alternatives");

    let alternatives: Alternatives =
        serde_json::from_str(&res_body).expect("error while reading json");

    alternatives
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_alternatives() {
        let alternatives = get_alternatives();
        assert!(alternatives.sets.len() != 0)
    }
}
