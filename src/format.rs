
use colored::*;

const Top_Left: &str = "┌";
const Top_Right: &str = "┐";
const Bottom_Left: &str = "└";
const Bottom_Right: &str = "┘";
const Horizontal: &str = "─";
const Space: &str = " ";
const Vertical: &str = "│";
const Backspace: &str = "\u{8}";

pub fn title_bar(width: usize, title: &str) -> String {
    let fill = (width - title.len()) - 2;
    format!(
      "{}{}{}{}", 
      Top_Left.magenta(),
      title.blue().bold(), 
      n_character(fill, Horizontal).magenta(), 
      Top_Right.magenta()
  )
}

pub fn pad(width: usize, content: &str) -> String {
    let fill = (width - content.len()) - 2;
    format!(
      "{}{}{}{}", 
      Vertical.magenta(),
      content.blue(), 
      n_character(fill, Space), 
      Vertical.magenta()
  )
}

pub fn end_bar(width: usize) -> String {
    let fill = width - 2;
    format!(
      "{}{}{}", 
      Bottom_Left.magenta(),
      n_character(fill, Horizontal).magenta(), 
      Bottom_Right.magenta()
  )
}

pub fn n_backspace(count: usize) -> String {
    n_character(count, Backspace)
}



// privates
fn n_character(count: usize, character: &str) -> String {
    format!("{}", (0..count).map(|_| character).collect::<String>())
}

fn n_dash(count: usize) -> String {
    n_character(count, Horizontal)
}
