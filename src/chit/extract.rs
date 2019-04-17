use super::format;

#[derive(Debug, Clone)]
pub struct Version {
    pub semver: String,
    pub date: String,
    pub downloads: Option<i64>,
    pub size_in_bytes: Option<i64>,
    pub license: String,
}

#[derive(Debug, Clone)]
pub struct Crate {
    pub name: String,
    pub downloads: Option<i64>,
    pub versions: Vec<Version>,
    pub recent_downloads: Option<i64>,
    pub repository: String,
    pub documentation: String,
    pub keywords: Vec<String>,
    pub categories: Vec<String>,
}

fn get_collect(key: &'static str, json: &serde_json::value::Value) -> Vec<String> {
    json[key]
        .as_array()
        .unwrap()
        .iter()
        .map(|word| format::remove_quotes(word.to_string()))
        .collect()
}

pub fn crate_fields(json: serde_json::value::Value) -> Option<Crate> {
    let name = format::remove_quotes(json["crate"]["name"].to_string());
    let downloads = json["crate"]["downloads"].as_i64();
    let recent_downloads = json["crate"]["recent_downloads"].as_i64();
    let repository = format::remove_quotes(json["crate"]["repository"].to_string());
    let documentation = json["crate"]["documentation"].to_string();
    let keywords = get_collect("keywords", &json["crate"]);
    let categories = get_collect("categories", &json["crate"]);

    let documentation = if documentation == "null" {
        format!("None specified in Cargo.toml")
    } else {
        format::remove_quotes(documentation)
    };

    if let Some(versions) = json["versions"].as_array() {
        let versions: Vec<Version> = versions
            .into_iter()
            .map(|version| {
                let mut date = super::format::remove_quotes(version["updated_at"].to_string());
                date.truncate(10);
                return Version {
                    date,
                    semver: format::remove_quotes(version["num"].to_string()),
                    downloads: version["downloads"].as_i64(),
                    size_in_bytes: version["crate_size"].as_i64(),
                    license: format::remove_quotes(version["license"].to_string()),
                };
            })
            .collect();

        Some(Crate {
            name,
            downloads,
            versions,
            recent_downloads,
            repository,
            documentation,
            keywords,
            categories,
        })
    } else {
        None
    }
}

pub fn calculate_rating(crate_info: &Crate) -> usize {
    let five = 20000;
    let four = 5000;
    let three = 1000;
    let two = 100;

    if let Some(recent_downloads) = crate_info.recent_downloads {
        if recent_downloads > five {
            5
        } else if recent_downloads > four {
            4
        } else if recent_downloads > three {
            3
        } else if recent_downloads > two {
            2
        } else {
            1
        }
    } else {
        0
    }
}
