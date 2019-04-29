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

/* Ideal state:
 * get data -> extract -> format -> print
*/

// pub fn print_owners(crate_name: String) {
//     let width = format::get_width();
//     println!("{}", format::get_crate_search_message(&crate_name));

//     match crates::get(crates::owners_url(&crate_name)) {
//         Some(crate_owners_json) => {
//             if let Some(owners) = extract::owners::fields(crate_owners_json) {
//                 for owner in owners.filter(|owner| owner.kind == "user") {
//                     if let Some(user_json) = crates::get(crates::user_url(owner.id)) {

//                     }
//                 }
//             }

//         }
//         None => println!("Failed to get crate owner details"),
//     }

//     println!("{}", format::end_bar(width));
// }

fn format_owners_details(
    width: usize,
    crate_name: &str,
    owners: Vec<extract::owners::UserWithAuthoredCrates>,
) -> Vec<String> {
    let mut lines = Vec::new();

    lines.push(format!("{}", format::title_bar(width, &crate_name)));

    for owner in owners {
        lines.extend(format_owner_details(owner));
    }

    lines
}

fn format_owner_details(owner: extract::owners::UserWithAuthoredCrates) -> Vec<String> {
    let mut lines = Vec::new();

    lines.push(format!("Crates by {}:", owner.name));

    for crate_name in owner.authored_crates {
        lines.push(format!("    {}", crate_name));
    }

    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn describe_owners_should_have_consistent_output() {
        let owners = vec![
            extract::owners::UserWithAuthoredCrates {
                name: String::from("ownerName1"),
                id: Some(1),
                kind: String::from("kind1"),
                authored_crates: vec![String::from("name"), String::from("other")],
            },
            extract::owners::UserWithAuthoredCrates {
                name: String::from("ownerName2"),
                id: Some(2),
                kind: String::from("kind2"),
                authored_crates: vec![String::from("name"), String::from("other")],
            },
        ];

        let lines = format_owners_details(40, "test", owners);
        println!("{:?}", lines);
        assert_eq!(lines[0].len(), 141);
        // TODO: assert the total number of lines in the output
    }

    #[test]
    fn describing_a_single_owner() {
        let owner = extract::owners::UserWithAuthoredCrates {
            name: String::from("ownerName1"),
            id: Some(1),
            kind: String::from("kind1"),
            authored_crates: vec![String::from("name"), String::from("other")],
        };

        let lines = format_owner_details(owner);
        println!("{:?}", lines);
        assert_eq!(lines[0].len(), 21);
        assert_eq!(lines[1].len(), 8);
        assert_eq!(lines[2].len(), 9);
    }
}
