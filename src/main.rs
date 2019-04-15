use clap::{App, Arg, SubCommand};
use colored::*;
use std::env;

mod meta;
mod chit;

fn main() {
    // subcommands
    let help = "help"; // from clap
    let versions = "versions";
    let owners = "owners";

    // Keep it simple
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let crate_name = args[1].clone();
        let reserved = vec![help, versions, owners, "-h", "--help", "-V", "--version"];
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

    // Actions
    if let Some(owners) = matches.subcommand_matches("owners") {
        if let Some(crate_name) = owners.value_of(crate_arg_name) {
           chit::owners::print_owners(crate_name.to_string())
        } else {
            println!("{}", "No crate supplied".yellow());
        };
    } else if let Some(versions) = matches.subcommand_matches("versions") {
        if let Some(crate_name) = versions.value_of(crate_arg_name) {
            chit::versions::print_versions(crate_name.to_string())
        } else {
            println!("{}", "No crate supplied".yellow());
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

