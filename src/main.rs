extern crate reqwest;

use colored::*;
use std::env;

mod crates;
mod extract;
mod format;

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
            if let Some(fields) = extract::crate_fields(crate_json) {
                println!("{}", format::title_bar(width, &crate_name));

                let rating = extract::calculate_rating(fields.clone());
                format::print_rating(width, rating);

                // Download count
                if let Some(download_count) = fields.downloads {
                    padded_print(width, format!("Total downloads: {}", download_count));
                }

                if let Some(recent_downloads) = fields.recent_downloads {
                    padded_print(width, format!("Recent downloads: {}", recent_downloads));
                }

                padded_print(width, String::from("Versions:"));
                for version in fields.versions {
                    padded_print(
                        width,
                        format!(
                            "    {} ({})",
                            format::remove_quotes(version.semver),
                            version.date
                        ),
                    );
                }
            }
        }
        None => {
            println!("{} {}", "Failed".red(), "to find that crate".magenta());
            return;
        }
    }

    match crates::get(crates::owners_url(&crate_name)) {
        Some(crate_owners_json) => {
            // Owners
            padded_print(
                width,
                format!(
                    "Owner: {}",
                    format::remove_quotes(crate_owners_json["users"][0]["name"].to_string())
                ),
            );

            padded_print(width, String::new());
            padded_print(width, String::from("Crates by owner:"));

            // Crates by owner
            if let Some(user_id) = crate_owners_json["users"][0]["id"].as_u64() {
                if let Some(user_json) = crates::get(crates::user_url(user_id)) {
                    if let Some(array) = user_json["crates"].as_array() {
                        for thing in array {
                            padded_print(
                                width,
                                format!("    {}", format::remove_quotes(thing["id"].to_string())),
                            );
                        }
                    }
                }
            }
        }
        None => println!("Failed to get crate owner details"),
    }

    println!("{}", format::end_bar(width));
}
