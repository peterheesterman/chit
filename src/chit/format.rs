use colored::*;

const HORIZONTAL: &str = "─";

pub fn title_bar(width: usize, title: &str) -> String {
    let fill = (width - title.len()) - 2;
    format!(
        "{}{}{}{}",
        HORIZONTAL.magenta(),
        title.blue().bold(),
        n_character(fill, HORIZONTAL).magenta(),
        HORIZONTAL.magenta()
    )
}

pub fn print(message: String) {
    println!("{}",  &message.to_string().blue());
}

pub fn end_bar(width: usize) -> String {
    let fill = width - 2;
    format!(
        "{}{}{}",
        HORIZONTAL.magenta(),
        n_character(fill, HORIZONTAL).magenta(),
        HORIZONTAL.magenta()
    )
}

pub fn get_width() -> usize {
    45
}

pub fn remove_quotes(value: String) -> String {
    let mut string = value.clone();
    string.remove(0);
    string.pop();
    string
}

pub fn print_rating(rating: usize) {
    let stars = n_character(rating, "🌟 ");
    let star_rating = format!("Rating: {}", stars);

    if rating > 0 && rating < 6 {
        println!("{}", &star_rating.to_string().blue());
    }
}

// privates
fn n_character(count: usize, string: &str) -> String {
    string.repeat(count)
}
