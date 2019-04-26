use colored::*;

const HORIZONTAL: &str = "─";

pub fn get_width() -> usize {
    45
}

pub fn print(message: String) {
    println!("{}", &message.to_string().blue());
}

pub fn title_bar(width: usize, title: &str) -> String {
    let fill = (width - title.len()) - 1;
    format!(
        "{}{}{}",
        HORIZONTAL.magenta(),
        title.blue().bold(),
        n_character(fill, HORIZONTAL).magenta()
    )
}

pub fn end_bar(width: usize) -> String {
    format!("{}", HORIZONTAL.repeat(width).magenta())
}

pub fn remove_quotes(value: String) -> String {
    let mut string = value.clone();
    string.remove(0);
    string.pop();
    string
}

// TODO: This should return a String
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

pub fn bounded_print(width: usize, text: &str) {
    let chars: Vec<char> = text.chars().collect();
    let split = &chars
        .chunks(width)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<_>>();

    for bit in split.iter() {
        print(bit.to_string());
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn  get_width_should_return_45() {
        let width = get_width();
        assert_eq!(width, 45);
    }

    #[test]
    fn title_bar_length_is_constant_for_input() {
        let title = title_bar(30, "tester").normal().to_string();
        assert_eq!(title.len(), 107);
    }

    #[test]
    fn end_bar_length_is_constant_for_input() {
        let end = end_bar(50).normal().to_string();
        assert_eq!(end.len(), 159);
    }

    #[test]
    fn remove_quotes_should_remove_a_single_set_of_quotes() {
        let quoted = String::from("\"Something\"");
        assert_eq!(remove_quotes(quoted), String::from("Something"));
    }
}

