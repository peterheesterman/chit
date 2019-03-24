
const ENDPOIONT: &str = "https://crates.io/api/v1/crates";

pub fn url(crate_name: &str) -> String {
  format!("{}/{}", ENDPOIONT, crate_name)
}

pub fn owners_url(crate_name: &str) -> String {
  format!("{}/{}/owners", ENDPOIONT, crate_name)
}
