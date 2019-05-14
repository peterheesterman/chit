use colored::*;

use super::extract;
use super::format;
use super::sources::crates;
use super::sources::get;
use super::sources::github;

pub fn print_repo(crate_name: String) {
    println!("{}", format::get_crate_search_message(&crate_name));
    match get(crates::url(&crate_name)) {
        Some(crate_json) => {
            if let Some(fields) = extract::package::fields(crate_json) {
                if github::check(fields.repository.as_str()) {
                    let url = github::api_url(&fields.repository);
                    match get(url) {
                        Some(repository_json) => {
                            let repo_fields = extract::repo::fields(repository_json);
                            let lines = describe_repository(
                                &crate_name,
                                fields.repository.as_str(),
                                repo_fields,
                            );
                            for line in lines {
                                println!("{}", line);
                            }
                        }
                        None => {
                            println!("Failed to get from repo");
                        }
                    }
                } else {
                    // We don't handle other providers yet
                    println!("This repository is not on github, make a PR!");
                }
            }
        }
        None => {
            println!("{} {}", "Failed".red(), "to find that crate".magenta());
            return;
        }
    }
}

fn describe_repository(
    crate_name: &str,
    repository_url: &str,
    fields: extract::repo::RepositoryInfo,
) -> Vec<String> {
    let mut lines = Vec::new();

    let mut width = format::get_width();
    let repo = format!("{} {}", "Repository: ", repository_url).blue();

    let large_widths: Vec<usize> = vec![width, repo.len()];

    width = *large_widths.iter().max().unwrap();

    lines.push(format!("{}", format::title_bar(width, &crate_name)));
    lines.push(format!("{}", repo));
    lines.push(format!(
        "{}",
        format!("{} {}", "Last commit date:", fields.last_commit_date).blue()
    ));

    if let Some(stars) = fields.stars {
        lines.push(format!(
            "{}",
            format!("{} {}", "Github Stars:", stars).blue()
        ));
    }

    if let Some(issues) = fields.issues {
        lines.push(format!(
            "{}",
            format!("{} {}", "Github Issues:", issues).blue()
        ));
    }

    lines.push(format!("{}", format::end_bar(width)));

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

        let lines = describe_repository("testBane", "urlfortest", repository_details);

        assert_eq!(lines[0].len(), 148);
        assert_eq!(lines[1].len(), 32);
        assert_eq!(lines[2].len(), 36);
        assert_eq!(lines[3].len(), 26);
        assert_eq!(lines[4].len(), 25);
        assert_eq!(lines[5].len(), 144);
        assert_eq!(lines.len(), 6);
    }
}
