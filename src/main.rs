extern crate reqwest;

use colored::*;
use serde_json::Value;
use std::env;
use std::io::Read;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        chit(args[1].to_string());
    } else {
        println!(
            "{}{}",
            "Chit currently only takes the name of a crate. e.g. ".yellow(),
            "chit steg".blue(),
        );
    }
}

fn n_dash(count: usize) -> String {
    format!("{}", (0..count).map(|_| "-").collect::<String>())
}

fn chit(crate_name: String) {
    let line = n_dash(crate_name.len()).red();
    println!("{}", line);
    println!("{}", crate_name.blue());
    println!("{}", line);

    let crate_endpoint = "https://crates.io/api/v1/crates";
    let crate_url = format!("{}/{}", crate_endpoint, crate_name);
    let crate_owners_url = format!("{}/{}", crate_url, "owners");

    let mut res = reqwest::get(&crate_url).expect("fail to get crate");
    let mut owners_res = reqwest::get(&crate_owners_url).expect("fail to get owners");

    if res.status() != 200 && owners_res.status() != 200 {
        return println!(
            "Error: Got back a bad response from crates.io. crate({}) crate_owners({})",
            res.status(),
            owners_res.status()
        );
    }

    // Crate
    let mut body = String::new();
    res.read_to_string(&mut body).expect("fail to read crate body");
    let crate_json: Value = serde_json::from_str(&body).expect("fail to serde parse for crate");

    // Crate owners
    let mut owners_body = String::new();
    owners_res.read_to_string(&mut owners_body).expect("fail to read crate owners body");
    let crate_owners_json: Value = serde_json::from_str(&owners_body).expect("fail to serde parse for crate owners");

    // Owners
    println!("Current owner: {}", crate_owners_json["users"][0]["name"].to_string());

    // Version
    println!("Current version: {}", crate_json["crate"]["max_version"].to_string());

    // Download count
    if let Some(download_count) = crate_json["crate"]["downloads"].as_i64() {
        println!("Download count: {:?}", download_count);
    }
}
