use colored::*;

use super::crates;
use super::extract;
use super::format;

pub fn print_versions(crate_name: String) {
    let width = format::get_width();
    let search_message = format::get_crate_search_message(&crate_name);
    println!("{}", search_message);
    match crates::get(crates::url(&crate_name)) {
        Some(crate_json) => {
            if let Some(fields) = extract::crate_fields(crate_json) {
                let lines = describe_versions(width, fields);
                for line in lines {
                    println!("{}", line);
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

fn describe_versions(width: usize, fields: extract::Crate) -> Vec<String> {
    let mut lines = Vec::new();

    lines.push(format!("{}", format::title_bar(width, &fields.name)));
    lines.push(format!("{}", "Versions:".blue()));

    for version in fields.versions {
        if let Some(size) = version.size_in_bytes {
            lines.push(format!(
                    "    {}  {} | {} | {}",
                    version.semver.blue(),
                    format!("({})", version.license).blue(),
                    version.date.blue(),
                    format!("{} kB", (size as f64 / 1000_f64).round()).blue(),
                    ));
        } else {
            lines.push(format!("    {} | {}", version.semver.blue(), version.date.blue()));
        }
    }
    
    lines
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn describe_versions_should_have_consistent_output() {
        let versions: Vec<extract::Version> = vec![
            extract::Version {
                date: String::from("some date"),
                semver: String::from("0.1.3"),
                downloads: Some(220), 
                size_in_bytes: Some(20),
                license: String::from("MIT"),
            }, 
            extract::Version {
                date: String::from("some date"),
                semver: String::from("0.1.1"),
                downloads: Some(120), 
                size_in_bytes: Some(10),
                license: String::from("MIT"),
            }
        ];

        let crate_details = extract::Crate {
            name: String::from("name"),
            description: String::from("name"),
            downloads: Some(100),
            versions,
            recent_downloads: Some(20),
            repository: String::from("name"),
            documentation: String::from("name"),
            keywords: vec![String::from("keyword")],
            categories: vec![String::from("keyword")], 
        };

        let lines = describe_versions(40, crate_details);
        assert_eq!(lines.len(), 4);
        assert_eq!(lines[0].len(), 141);
        assert_eq!(lines[1].len(), 18);
        assert_eq!(lines[2].len(), 71);
        assert_eq!(lines[3].len(), 71);
    }
}
