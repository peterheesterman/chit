use colored::*;

use super::crates;
use super::extract;
use super::format;

mod alternatives;

pub fn print_details(crate_name: String) {
    
    let mut width = format::get_width();
    println!("{} {}...", "Searching for".magenta(), &crate_name.blue());

    match crates::get(crates::url(&crate_name)) {
        Some(crate_json) => {
            if let Some(fields) = extract::crate_fields(crate_json) {
                // Asume repository is the longest field
                let repository_details = format!("Repository: {}", &fields.repository);

                let keywords = format!("Keywords: {}", fields.keywords.join(", "));
                let categories = format!("Categories: {}", fields.categories.join(", "));

                let large_widths: Vec<usize> = vec![
                    width,
                    repository_details.len(),
                    keywords.len(),
                    categories.len(),
                ];

                width = *large_widths.iter().max().unwrap();

                println!("{}", format::title_bar(width, &crate_name));

                // Description
                let chars: Vec<char> = fields.description.chars().collect();
                let split = &chars
                    .chunks(width)
                    .map(|chunk| chunk.iter().collect::<String>())
                    .collect::<Vec<_>>();

                for bit in split.iter() {
                    format::print(bit.to_string());
                }

                println!("");
                // Rating
                let rating = extract::calculate_rating(&fields);
                format::print_rating(rating);

                // Download count
                if let Some(download_count) = fields.downloads {
                    format::print(format!("Total downloads: {}", download_count));
                }

                if let Some(recent_downloads) = fields.recent_downloads {
                    format::print(format!("Recent downloads: {}", recent_downloads));
                }

                let recent_version = &fields.versions[0];
                format::print(format!(
                    "Latest version: {} ({})",
                    recent_version.semver, recent_version.date
                ));

                format::print(format!("Docs: {}", fields.documentation));

                format::print(repository_details);

                format::print(format!(
                    "Crates.io: https://crates.io/crates/{}",
                    &crate_name
                ));

                format::print(format!("License: {}", recent_version.license));

                if let Some(size) = recent_version.size_in_bytes {
                    format::print(format!(
                        "Crate size: {} kB",
                        (size as f64 / 1000_f64).round()
                    ));
                }

                // TODO: move to another file
                // ---------------------------------------------------------
                // TODO: Clean this up by making it less imparative
                let mut found_alternative = false;
                let alternatives = alternatives::get_alternatives();
                'find: for i in 0..alternatives.crates.len() {
                    if alternatives.crates[i].name == crate_name {
                        format::print(format!(
                            "Alternatives: {}",
                            alternatives.crates[i].alternatives.join(", ")
                        ));
                        found_alternative = true;
                        break 'find;
                    }
                }
               
                if !found_alternative {
                    format::print("Alternatives: None listed - Know one? Make a PR!".to_string());
                }
                // ---------------------------------------------------------

                format::print(keywords);
                format::print(categories);
            }
        }
        None => {
            println!("{} {}", "Failed".red(), "to find that crate".magenta());
            return;
        }
    }

    match crates::get(crates::owners_url(&crate_name)) {
        Some(crate_owners_json) => {
            let owners_names: Vec<String> = crate_owners_json["users"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|json| json["kind"] == "user")
                .map(|json| format::remove_quotes(json["name"].to_string()))
                .collect();
            let multiple = owners_names.len() > 1;

            let owners_names = owners_names.join(", ");

            // Owners
            format::print(format!(
                "Owner{}: {}",
                if multiple { "s" } else { "" },
                owners_names
            ));
        }
        None => println!("Failed to get crate owner details"),
    }

    println!("{}", format::end_bar(width));
}
