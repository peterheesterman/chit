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

// This should use a closure
fn padded_print(width: usize, message: String) {
    println!("{}", format::pad(width, &message));
}

fn chit(crate_name: String) {
    let width = format::get_width(&crate_name);
    println!("{}", format::title_bar(width, &crate_name));

    let crate_json: Value = crates::get(crates::url(&crate_name));
    let crate_owners_json: Value = crates::get(crates::owners_url(&crate_name));
    
    // Version
    padded_print(width, format!(
        "Lastest version: {} ({})",
        crate_json["crate"]["max_version"].to_string(),
        crate_json["crate"]["updated_at"].to_string()
    ));

    // Download count
    if let Some(download_count) = crate_json["crate"]["downloads"].as_i64() {
        padded_print(width, format!("Download count: {:?}", download_count));
    }

    // Owners
    padded_print(width, String::new());
    padded_print(width, format!(
        "Written by {}, Other crates by the same owner:",
        crate_owners_json["users"][0]["name"].to_string()
    ));

    // Other crates
    if let Some(user_id) = crate_owners_json["users"][0]["id"].as_u64() {
        let user_json: Value = crates::get(crates::user_url(user_id));
        if let Some(array) = user_json["crates"].as_array() {
            for thing in array {
                padded_print(width, format!(
                    "    {}",
                    thing["id"].to_string()
                ));
            }
        }
    }

    println!("{}", format::end_bar(width));
}
