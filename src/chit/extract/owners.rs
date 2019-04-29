use super::super::format;
use serde_json::json;

#[derive(Debug, Clone)]
pub struct User {
    pub name: String,
    pub id: Option<i64>,
    pub kind: String,
}

#[derive(Debug, Clone)]
pub struct UserWithAuthoredCrates {
    pub name: String,
    pub id: Option<i64>,
    pub kind: String,
    pub authored_crates: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct CrateOwners {
    pub users: Vec<User>,
}

pub fn fields(json: serde_json::value::Value) -> Option<CrateOwners> {
    if let Some(users) = json["users"].as_array() {
        let users = users
            .into_iter()
            .map(|user| {
                return User {
                    name: format::remove_quotes(user["name"].to_string()),
                    kind: format::remove_quotes(user["kind"].to_string()),
                    id: user["id"].as_i64(),
                };
            })
            .collect();

        Some(CrateOwners { users })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_get_owners_fields() {
        let json = json!(  {
            "users": [{
                "avatar": "https://avatars2.githubusercontent.com/u/16065728?v=4",
                "id": 28804,
                "kind": "user",
                "name": "Peter Heesterman",
            }]
        });

        assert!(match fields(json) {
            Some(_) => true,
            None => false,
        });
    }
}
