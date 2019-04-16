use colored::*;

use super::crates;
use super::format;

pub fn print_owners(crate_name: String) {
    let width = format::get_width();
    println!("{} {}...", "Searching for".magenta(), &crate_name.blue());

    match crates::get(crates::owners_url(&crate_name)) {
        Some(crate_owners_json) => {
            println!("{}", format::title_bar(width, &crate_name));

            for user_json in crate_owners_json["users"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|json| json["kind"] == "user")
            {
                // Owner
                format::print(format!(
                    "Crates by {}:",
                    format::remove_quotes(user_json["name"].to_string())
                ));

                // Crates by owner
                if let Some(user_id) = user_json["id"].as_u64() {
                    if let Some(user_json) = crates::get(crates::user_url(user_id)) {
                        if let Some(array) = user_json["crates"].as_array() {
                            for thing in array {
                                format::print(format!(
                                    "    {}",
                                    format::remove_quotes(thing["id"].to_string())
                                ));
                            }
                        }
                    }
                }
            }
        }
        None => println!("Failed to get crate owner details"),
    }

    println!("{}", format::end_bar(width));
}
