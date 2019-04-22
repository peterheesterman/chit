use std::fs::File;
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct Crate {
    pub name: String,
    pub alternatives: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Alternatives {
    pub description: String,
    pub crates: Vec<Crate>,
}

pub fn get_alternatives() -> Alternatives {
    let json_file_path = Path::new("./alternatives.json");
    let json_file = File::open(json_file_path).expect("file not found");
    let alternatives: Alternatives =
        serde_json::from_reader(json_file).expect("error while reading json");
    alternatives
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_alternatives() {
        let alternatives = get_alternatives();
        assert!(alternatives.crates.len() != 0)
    }
}
