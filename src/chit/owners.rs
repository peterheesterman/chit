use colored::*;

use super::crates;
use super::extract;
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
                let name = format::remove_quotes(user_json["name"].to_string());

                if let Some(user_id) = user_json["id"].as_u64() {
                    if let Some(user_json) = crates::get(crates::user_url(user_id)) {
                        if let Some(set) = extract::owners::fields(user_json) {
                            let lines = format_owner_details(&name, set);
                            for line in lines {
                                println!("{}", line.blue());
                            }
                        }
                    }
                }
            }
            println!("{}", format::end_bar(width));
        }
        None => println!("Failed to get crate owner details"),
    }
}

fn format_owner_details(owner_name: &str, owner: extract::owners::AuthoredCrates) -> Vec<String> {
    let mut lines = Vec::new();

    lines.push(format!("Crates by {}:", owner_name));

    for crate_name in owner.authored_crates {
        lines.push(format!("    {}", crate_name));
    }

    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn describing_a_single_owner_achievements() {
        let owner = extract::owners::AuthoredCrates {
            authored_crates: vec![String::from("name"), String::from("other")],
        };

        let lines = format_owner_details("ownerName1", owner);
        println!("{:?}", lines);
        assert_eq!(lines[0].len(), 21);
        assert_eq!(lines[1].len(), 8);
        assert_eq!(lines[2].len(), 9);
        assert_eq!(lines.len(), 3);
    }
}
