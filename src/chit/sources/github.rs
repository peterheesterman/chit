pub fn check(url: &str) -> bool {
    url.contains("github.com")
}

pub fn api_url(url: &str) -> String {
    format!(
        "{}{}",
        "https://api.github.com/repos/",
        crop_letters(url, 19)
    )
}
// https://github.com/peterheesterman/chit
// https://api.github.com/repos/peterheesterman/chit
// IDEA: add the number of pull requests open on the repository as well.
// /peterheesterman/chit/pulls?state=open
fn crop_letters(s: &str, pos: usize) -> &str {
    match s.char_indices().skip(pos).next() {
        Some((pos, _)) => &s[pos..],
        None => "",
    }
}
