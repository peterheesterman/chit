use colored::*;

use super::sources::{ get };
use super::sources::crates;
use super::sources::github;
use super::extract;
use super::format;

pub fn print_repo(crate_name: String) {
    let width = format::get_width();
    println!("{}", format::get_crate_search_message(&crate_name));
    match get(crates::url(&crate_name)) {
        Some(crate_json) => {
            if let Some(fields) = extract::package::fields(crate_json) {
                if github::check(fields.repository.as_str()) {
                    let url = github::api_url(fields.repository);
                    match get(url) {
                        Some(repository_json) => {
                            let repo_fields = extract::repo::fields(repository_json);
                            let lines = describe_repository(width, &crate_name, repo_fields);
                            for line in lines {
                                println!("{}", line);
                            }
                        },
                        None => {
                            println!("Failed to get from repo");
                        }
                    }
                } else {
                    // We don't handle other providers yet
                    println!("This repository is not on github, make a PR!");
                }
            }
        },
        None => {
            println!("{} {}", "Failed".red(), "to find that crate".magenta());
            return;
        }
    }
    println!("{}", format::end_bar(width));
}

fn describe_repository(width: usize, crate_name: &str, fields: extract::repo::RepositoryInfo) -> Vec<String> {
    let mut lines = Vec::new();

    lines.push(format!("{}", format::title_bar(width, &crate_name)));
    lines.push(format!("{}", format!("{} {}", "Last commit date:", fields.last_commit_date).blue()));
   
    if let Some(stars) = fields.stars {
        lines.push(format!("{}", format!("{} {}", "Github Stars:", stars).blue()));
    }

    if let Some(issues) = fields.issues {
        lines.push(format!("{}", format!("{} {}", "Github Issues:", issues).blue()));
    }

    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn describe_versions_should_have_consistent_output() {

        let repository_details = extract::repo::RepositoryInfo {
            last_commit_date: String::from("fake date"),
            stars: Some(100),
            issues: Some(2),
        };

        let lines = describe_repository(40, "testBane", repository_details);

        assert_eq!(lines[0].len(), 133);
        assert_eq!(lines[1].len(), 36);
        assert_eq!(lines[2].len(), 26);
        assert_eq!(lines[3].len(), 25);
        assert_eq!(lines.len(), 4);
    }
}
