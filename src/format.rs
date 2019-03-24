use colored::*;

const TOP_LEFT: &str = "┌";
const TOP_RIGHT: &str = "┐";
const BOTTOM_LEFT: &str = "└";
const BOTTOM_RIGHT: &str = "┘";
const HORIZONTAL: &str = "─";
const SPACE: &str = " ";
const VERTICAL: &str = "│";
//const BACKSPACE: &str = "\u{8}";

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
    let fill = (width - content.len()) - 2;
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

// privates
fn n_character(count: usize, character: &str) -> String {
    format!("{}", (0..count).map(|_| character).collect::<String>())
}
