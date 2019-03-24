extern crate reqwest;

use colored::*;
use serde_json::Value;
use std::env;
use std::io::Read;

mod format;
mod crates;

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



fn chit(crate_name: String) {
    let width = 40;
    println!("{}", format::title_bar(width, &crate_name));

    let mut res = reqwest::get(&crates::url(&crate_name))
                    .expect("fail to get crate");

    let mut owners_res = reqwest::get(&crates::owners_url(&crate_name))
                            .expect("fail to get owners");

    if res.status() != 200 && owners_res.status() != 200 {
        return println!(
            "Error: Got back a bad response from crates.io. crate({}) crate_owners({})",
            res.status(),
            owners_res.status()
        );
    }

    // Crate
    let mut body = String::new();
    res.read_to_string(&mut body)
        .expect("fail to read crate body");
    let crate_json: Value = serde_json::from_str(&body).expect("fail to serde parse for crate");

    // Crate owners
    let mut owners_body = String::new();
    owners_res
        .read_to_string(&mut owners_body)
        .expect("fail to read crate owners body");
    let crate_owners_json: Value =
        serde_json::from_str(&owners_body).expect("fail to serde parse for crate owners");

    // Owners
    println!(
        "{}",
        format::pad(
            width,
            &format!(
                "Current owner: {}",
                crate_owners_json["users"][0]["name"].to_string()
            )
        )
    );

    // Version
    println!(
        "{}",
        format::pad(
            width,
            &format!(
                "Current version: {}",
                crate_json["crate"]["max_version"].to_string()
            )
        )
    );

    // Download count
    if let Some(download_count) = crate_json["crate"]["downloads"].as_i64() {
        println!(
            "{}",
            format::pad(width, &format!("Download count: {:?}", download_count))
        );
    }

    println!("{}", format::end_bar(width));
}
