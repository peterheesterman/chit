use super::super::format;

#[derive(Debug, Clone)]
pub struct RepositoryInfo {
    pub last_commit_date: String,
    pub stars: Option<i64>,
    pub issues: Option<i64>,
}

pub fn fields(json: serde_json::value::Value) -> RepositoryInfo {
    let mut last_commit_date = format::remove_quotes(json["updated_at"].to_string());
    last_commit_date.truncate(10);
    let stars = json["stargazers_count"].as_i64();
    let issues = json["open_issues_count"].as_i64();

    RepositoryInfo { last_commit_date, stars, issues }
}
