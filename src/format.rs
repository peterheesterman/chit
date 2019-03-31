use colored::*;

const TOP_LEFT: &str = "┌";
const TOP_RIGHT: &str = "┐";
const BOTTOM_LEFT: &str = "└";
const BOTTOM_RIGHT: &str = "┘";
const HORIZONTAL: &str = "─";
const SPACE: &str = " ";
const VERTICAL: &str = "│";

pub fn title_bar(width: usize, title: &str) -> String {
    let fill = (width - title.len()) - 2;
    format!(
        "{}{}{}{}",
        TOP_LEFT.magenta(),
        title.blue().bold(),
        n_character(fill, HORIZONTAL).magenta(),
        TOP_RIGHT.magenta()
    )
}

pub fn pad(width: usize, content: &str) -> String {
    let fill = if width < content.len() + 10 {
        0
    } else {
        (width - content.len()) - 2
    };

    format!(
        "{}{}{}{}",
        VERTICAL.magenta(),
        content.blue(),
        n_character(fill, SPACE),
        VERTICAL.magenta()
    )
}

pub fn end_bar(width: usize) -> String {
    let fill = width - 2;
    format!(
        "{}{}{}",
        BOTTOM_LEFT.magenta(),
        n_character(fill, HORIZONTAL).magenta(),
        BOTTOM_RIGHT.magenta()
    )
}

pub fn get_width(crate_name: &str) -> usize {
    let width = 45;
    if crate_name.len() > width - 2 {
        10 + crate_name.len()
    } else {
        width
    }
}

pub fn remove_quotes(value: String) -> String {
    let mut string = value.clone();
    string.remove(0);
    string.pop();
    string
}

pub fn print_rating(width: usize, rating: usize) {
    let stars = n_character(rating, "🌟 ");
    let star_rating = format!("Rating: {}", stars);

    if rating > 0 && rating < 6 {
        println!("{}", pad(width + rating * 3, &star_rating))
    }
}

// privates
fn n_character(count: usize, character: &str) -> String {
    format!("{}", (0..count).map(|_| character).collect::<String>())
}
