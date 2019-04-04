use clap::{App, Arg, SubCommand};
use colored::*;
use std::env;

mod crates;
mod extract;
mod format;

fn main() {
    // subcommands
    let help = "help"; // from clap
    let versions = "versions";
    let owners = "owners";

    let reserved = vec![help, versions, owners, "-h", "--help", "-V", "--version"];

    // Keep it simple
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        let crate_name = args[1].clone();
        if !reserved.contains(&&crate_name.as_str()) {
            chit(crate_name.to_string());
            return;
        }
    }

    // Next level
    let semver = "0.1.2";
    let author = "Peter Heesterman <heesterman.peter@gmail.com>";

    // Args
    let crate_arg_name = "crate";
    let crate_arg = Arg::with_name(&crate_arg_name)
        .short("c")
        .long("crate")
        .help("The name of the crate")
        .required(false)
        .takes_value(true);

    let matches = App::new("chit")
        .version(semver)
        .author(author)
        .about("Easily looking up details about rust crates")
        .arg(crate_arg.clone())
        .subcommand(
            SubCommand::with_name(owners)
                .about("Find out details about a crate owners")
                .version(semver)
                .author(author)
                .arg(crate_arg.clone()),
        )
        .subcommand(
            SubCommand::with_name(versions)
                .about("Find out details about a crate versions")
                .version(semver)
                .author(author)
                .arg(crate_arg),
        )
        .get_matches();

    if let Some(owners) = matches.subcommand_matches("owners") {
        if let Some(crate_name) = owners.value_of(crate_arg_name) {
            chit_owners(crate_name.to_string())
        } else {
            println!("{}", "No crate supplied".yellow());
        };
    } else if let Some(versions) = matches.subcommand_matches("versions") {
        if let Some(crate_name) = versions.value_of(crate_arg_name) {
            chit_versions(crate_name.to_string())
        } else {
            println!("{}", "No crate supplied".yellow());
        };
    } else if let Some(crate_name) = matches.value_of(crate_arg_name) {
        chit(crate_name.to_string())
    } else {
        println!(
            "{} {}",
            "It looks like there was an issue, try: ".yellow(),
            "chit --help".blue()
        );
    }
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
                    format::padded_print(width, format!("Total downloads: {}", download_count));
                }

                if let Some(recent_downloads) = fields.recent_downloads {
                    format::padded_print(width, format!("Recent downloads: {}", recent_downloads));
                }

                let recent_version = fields.versions[0].clone();
                format::padded_print(width, 
                    format!("Latest version: {} ({})",
                        format::remove_quotes(recent_version.semver),
                        recent_version.date
                    )
                );

                if let Some(size) = recent_version.size_in_bytes {
                    format::padded_print(width, format!("Crate size: {} kB", (size as f64 / 1000_f64).round()));
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
            format::padded_print(
                width,
                format!("Owner{}: {}", if multiple { "s" } else { "" }, owners_names),
            );
        }
        None => println!("Failed to get crate owner details"),
    }

    println!("{}", format::end_bar(width));
}

fn chit_owners(crate_name: String) {
    let width = format::get_width(&crate_name);
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
                format::padded_print(
                    width,
                    format!(
                        "Crates by {}:",
                            format::remove_quotes(user_json["name"].to_string())
                    ),
                );

                // Crates by owner
                if let Some(user_id) = user_json["id"].as_u64() {
                    if let Some(user_json) = crates::get(crates::user_url(user_id)) {
                        if let Some(array) = user_json["crates"].as_array() {
                            for thing in array {
                                format::padded_print(
                                    width,
                                    format!(
                                        "    {}",
                                        format::remove_quotes(thing["id"].to_string())
                                    ),
                                );
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

fn chit_versions(crate_name: String) {
    let width = format::get_width(&crate_name);
    println!("{} {}...", "Searching for".magenta(), &crate_name.blue());

    match crates::get(crates::url(&crate_name)) {
        Some(crate_json) => {
            if let Some(fields) = extract::crate_fields(crate_json) {
                println!("{}", format::title_bar(width, &crate_name));

                format::padded_print(width, String::from("Versions:"));
                for version in fields.versions {
                    if let Some(size) = version.size_in_bytes {
                        format::padded_print(
                            width,
                            format!(
                                "    {} | {} | {} kB",
                                format::remove_quotes(version.semver),
                                version.date,
                                (size as f64 / 1000_f64).round()
                            )
                        );
                    } else {
                        format::padded_print(
                            width,
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
