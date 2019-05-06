use super::super::format;

#[derive(Debug, Clone)]
pub struct AuthoredCrates {
    pub authored_crates: Vec<String>,
}

pub fn fields(json: serde_json::value::Value) -> Option<AuthoredCrates> {
    if let Some(crates) = json["crates"].as_array() {
        let authored_crates = crates
            .into_iter()
            .map(|package| return format::remove_quotes(package["id"].to_string()))
            .collect();

        Some(AuthoredCrates { authored_crates })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn can_get_owners_fields() {
        let json = json!({
            "crates": [{
                "id": "chit",
                "name": "chit",
                "links": {
                    "owner_user": "/api/v1/crates/chit/owner_user",
                    "reverse_dependencies": "/api/v1/crates/chit/reverse_dependencies"
                }
            }, {
                "id": "lsb_png_steganography",
                "name": "lsb_png_steganography",
                "links": {
                    "owner_user": "/api/v1/crates/lsb_png_steganography/owner_user",
                    "reverse_dependencies": "/api/v1/crates/lsb_png_steganography/reverse_dependencies"
                }
            }]
        });

        assert!(match fields(json) {
            Some(set) => {
                if set.authored_crates.len() == 2 {
                    true
                } else {
                    false
                }
            }
            None => false,
        });
    }
}
