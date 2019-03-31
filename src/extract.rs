#[derive(Debug, Clone)]
pub struct Version {
    pub semver: String,
    pub date: String,
    pub downloads: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct Crate {
    pub name: String,
    pub downloads: Option<i64>,
    pub versions: Vec<Version>,
    pub recent_downloads: Option<i64>,
}

pub fn crate_fields(json: serde_json::value::Value) -> Option<Crate> {
    let name = json["crate"]["name"].to_string();
    let downloads = json["crate"]["downloads"].as_i64();
    let recent_downloads = json["crate"]["recent_downloads"].as_i64();

    if let Some(versions) = json["versions"].as_array() {
        let versions: Vec<Version> = versions
            .into_iter()
            .map(|version| {
                let mut date = super::format::remove_quotes(version["updated_at"].to_string());
                date.truncate(10);
                return Version {
                    date,
                    semver: version["num"].to_string(),
                    downloads: version["downloads"].as_i64(),
                };
            })
            .collect();

        Some(Crate {
            name,
            downloads,
            versions,
            recent_downloads,
        })
    } else {
        None
    }
}

pub fn calculate_rating(crate_info: Crate) -> usize {
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
