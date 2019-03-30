extern crate reqwest;

use colored::*;
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
    println!("{} {}...", "Searching for".magenta(), &crate_name.blue());

    match crates::get(crates::url(&crate_name)) {
        Some(crate_json) => {
            // We found the crate
            println!("{}", format::title_bar(width, &crate_name));

            let mut date = format::remove_quotes(crate_json["crate"]["updated_at"].to_string());
            date.truncate(10);
            // Version
            padded_print(width, format!(
                "Lastest version: {} ({})",
                format::remove_quotes(crate_json["crate"]["max_version"].to_string()),
                date
            ));

            // Download count
            if let Some(download_count) = crate_json["crate"]["downloads"].as_i64() {
                padded_print(width, format!("Download count: {:?}", download_count));
            }
        },
        None => {
            println!("{} {}", "Failed".red(), "to find that crate".magenta());
            return
        },
    }
    
    match crates::get(crates::owners_url(&crate_name)) {
        Some(crate_owners_json) => {
            // Owners
            padded_print(width, format!(
                "Owner: {}",
                format::remove_quotes(crate_owners_json["users"][0]["name"].to_string())
            ));

            padded_print(width, String::new());
            padded_print(width, String::from("Crates by owner:"));

            // Crates by owner
            if let Some(user_id) = crate_owners_json["users"][0]["id"].as_u64() {
                if let Some(user_json) = crates::get(crates::user_url(user_id)) {
                    if let Some(array) = user_json["crates"].as_array() {
                        for thing in array {
                            padded_print(width, format!(
                                "    {}",
                                format::remove_quotes(thing["id"].to_string())
                            ));
                        }
                    }
                }
            }
        },
        None => {
            println!("Failed to get crate owner details")
        },
    }

    println!("{}", format::end_bar(width));
}
