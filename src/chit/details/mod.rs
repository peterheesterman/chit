use colored::*;

use super::extract;
use super::format;
use super::sources::crates;
use super::sources::get;

mod alternatives;

pub fn print_details(crate_name: String) {
    let mut width = format::get_width();
    println!("{} {}...", "Searching for".magenta(), &crate_name.blue());

    match get(crates::url(&crate_name)) {
        Some(crate_json) => {
            if let Some(fields) = extract::package::fields(crate_json) {
                // Assume repository is the longest field
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

                // Print the retrieved crate name
                println!("{}", format::title_bar(width, &fields.name));

                // Description
                format::bounded_print(width, &fields.description);

                println!();
                // Rating
                let rating = extract::package::calculate_rating(&fields);
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
                    &fields.name
                ));

                format::print(format!("License: {}", recent_version.license));

                if let Some(size) = recent_version.size_in_bytes {
                    format::print(format!(
                        "Crate size: {} kB",
                        (size as f64 / 1000_f64).round()
                    ));
                }

                // Does crate contain unsafe code?
                if super::unsafe_check::contains_unsafe(&fields.name, &recent_version.semver)
                    .expect("Failed to download crate")
                {
                    format::print("This crate contains ⚔️unsafe⚔️ code!".to_owned());
                } else {
                    // Not quite true... there might be unsafe code within macros.
                    format::print("This crate contains no ✔️unsafe✔️ code!".to_owned());
                }

                // IDEA: Clean this up by making it less imperative and into another file
                let mut found_alternative = false;
                let alternatives = alternatives::get_alternatives();

                match alternatives {
                    Ok(alternatives) => {
                        for i in 0..alternatives.sets.len() {
                            let set = &alternatives.sets[i];

                            if !set.alternatives.iter().any(|x| x == &crate_name || x == &fields.name) {
                                continue;
                            }

                            let mut alternatives = set.alternatives.clone();
                            alternatives.retain(|x| *x != crate_name && x != &fields.name);
                            let list_line = format!("Alternatives: {}", alternatives.join(", "));
                            format::bounded_print(width, &list_line);
                            found_alternative = true;
                        }

                        if !found_alternative {
                            format::print(
                                "Alternatives: None listed - Know one? Make a PR!".to_string(),
                            );
                        }
                    }
                    Err(err) => {
                        format::print(format!("Error retrieving alternatives because: {}", err));
                    }
                }

                format::print(keywords);
                format::print(categories);
            }
        }
        None => {
            println!("{} {}", "Failed".red(), "to find that crate".magenta());
            return;
        }
    }

    match get(crates::owners_url(&crate_name)) {
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
