const ENDPOINT: &str = "https://crates.io/api/v1/crates";

pub fn url(crate_name: &str) -> String {
    format!("{}/{}", ENDPOINT, crate_name)
}

pub fn owners_url(crate_name: &str) -> String {
    format!("{}/{}/owners", ENDPOINT, crate_name)
}

pub fn user_url(user_id: u64) -> String {
    let per_page = 100;
    format!(
        "{}?page=1&per_page={}&sort=alpha&user_id={}",
        ENDPOINT, per_page, user_id
    )
}
