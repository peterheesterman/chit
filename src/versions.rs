use colored::*;

use super::crates;
use super::format;
use super::extract;

pub fn print_versions(crate_name: String) {
    let width = format::get_width();
    println!("{} {}...", "Searching for".magenta(), &crate_name.blue());

    match crates::get(crates::url(&crate_name)) {
        Some(crate_json) => {
            if let Some(fields) = extract::crate_fields(crate_json) {
                println!("{}", format::title_bar(width, &crate_name));

                format::print(String::from("Versions:"));
                for version in fields.versions {
                    if let Some(size) = version.size_in_bytes {
                        format::print(
                            format!(
                                "    {}  ({}) | {} | {} kB",
                                format::remove_quotes(version.semver),
                                format::remove_quotes(version.license),
                                version.date,
                                (size as f64 / 1000_f64).round(),
                            )
                        );
                    } else {
                        format::print(
                            format!(
                                "    {} | {}",
                                format::remove_quotes(version.semver),
                                version.date
                            )
                        );
                    }
                }
            }
        }
        None => {
            println!("{} {}", "Failed".red(), "to find that crate".magenta());
            return;
        }
    }

    println!("{}", format::end_bar(width));
}
