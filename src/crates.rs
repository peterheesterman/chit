extern crate reqwest;

use std::io::Read;

const ENDPOIONT: &str = "https://crates.io/api/v1/crates";

pub fn url(crate_name: &str) -> String {
  format!("{}/{}", ENDPOIONT, crate_name)
}

pub fn owners_url(crate_name: &str) -> String {
  format!("{}/{}/owners", ENDPOIONT, crate_name)
}

pub fn get(url: String) -> serde_json::value::Value {
    let mut res = reqwest::get(&url)
                    .expect("fail to get crate");

    if res.status() != 200 {
        panic!("fail")
    }

    // Crate
    let mut res_body = String::new();
    res.read_to_string(&mut res_body)
        .expect("fail to read crate res_body");

    serde_json::from_str(&res_body).expect("fail to serde parse res_body")
}
