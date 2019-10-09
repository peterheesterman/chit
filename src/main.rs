#[macro_use]
extern crate serde_derive;

use clap::{App, Arg, SubCommand};
use colored::*;
use std::env;

mod chit;
mod meta;

fn main() {
    // subcommands
    let help = "help"; // from clap
    let versions = "versions";
    let owners = "owners";
    let repo = "repo";

    // Keep it simple
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let crate_name = args[1].clone();
        let reserved = vec![
            help,
            versions,
            owners,
            repo,
            "-h",
            "--help",
            "-V",
            "--version",
        ];
        if !reserved.contains(&&crate_name.as_str()) {
            chit::details::print_details(crate_name.to_string());
            return;
        }
    }

    // Meta
    let semver = meta::get_version();
    let author = meta::get_author();

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
                .about("Find out details about crate owners")
                .version(semver)
                .author(author)
                .arg(crate_arg.clone()),
        )
        .subcommand(
            SubCommand::with_name(versions)
                .about("Find out details about a crates versions")
                .version(semver)
                .author(author)
                .arg(crate_arg.clone()),
        )
        .subcommand(
            SubCommand::with_name(repo)
                .about("Find out details about a crates repository")
                .version(semver)
                .author(author)
                .arg(crate_arg),
        )
        .get_matches();

    let no_crate_message = format!("{}", "No crate supplied".yellow());

    // Actions
    if let Some(owners) = matches.subcommand_matches("owners") {
        if let Some(crate_name) = owners.value_of(crate_arg_name) {
            chit::owners::print_owners(crate_name.to_string())
        } else {
            println!("{}", no_crate_message);
        };
    } else if let Some(versions) = matches.subcommand_matches("versions") {
        if let Some(crate_name) = versions.value_of(crate_arg_name) {
            chit::versions::print_versions(crate_name.to_string())
        } else {
            println!("{}", no_crate_message);
        };
    } else if let Some(repo) = matches.subcommand_matches("repo") {
        if let Some(crate_name) = repo.value_of(crate_arg_name) {
            chit::repo::print_repo(crate_name.to_string())
        } else {
            println!("{}", no_crate_message);
        };
    } else if let Some(crate_name) = matches.value_of(crate_arg_name) {
        chit::details::print_details(crate_name.to_string())
    } else {
        println!(
            "{} {}",
            "It looks like there was an issue, try: ".yellow(),
            "chit --help".blue()
        );
    }
}
