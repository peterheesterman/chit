extern crate reqwest;

use colored::*;
use serde_json::Value;
use std::env;

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
    let width = format::get_width(&crate_name);
    println!("{}", format::title_bar(width, &crate_name));

    let crate_json: Value = crates::get(crates::url(&crate_name));
    let crate_owners_json: Value = crates::get(crates::owners_url(&crate_name));

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
