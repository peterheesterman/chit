use colored::*;

const HORIZONTAL: &str = "─";

pub fn get_width() -> usize {
    45
}

pub fn print(message: String) {
    println!("{}", &message.to_string().bright_blue());
}

pub fn title_bar(width: usize, title: &str) -> String {
    let fill = (width - title.len()) - 1;
    format!(
        "{}{}{}",
        HORIZONTAL.magenta(),
        title.bright_blue().bold(),
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

pub fn print_rating(rating: usize) {
    let star_rating = get_stars(rating);

    if rating > 0 && rating < 6 {
        println!("{}", &star_rating.to_string().bright_blue());
    }
}

pub fn get_stars(rating: usize) -> String {
    let stars = n_character(rating, "🌟 ");
    format!("Rating: {}", stars)
}

pub fn bound_lines(width: usize, text: &str) -> Vec<String> {
    let chars: Vec<char> = text.chars().collect();
    let lines = &chars
        .chunks(width)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<_>>();
    lines.to_vec()
}

pub fn bounded_print(width: usize, text: &str) {
    for line in bound_lines(width, text).iter() {
        print(line.to_string());
    }
}

fn n_character(count: usize, string: &str) -> String {
    string.repeat(count)
}

pub fn get_crate_search_message(crate_name: &str) -> String {
    format!(
        " {} {}...",
        "Searching for".magenta(),
        &crate_name.bright_blue()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_width_should_return_45() {
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

    #[test]
    fn get_stars_should_have_constant_length_for_n_stars() {
        let stars = get_stars(2);
        assert_eq!(stars.len(), 18);
    }

    #[test]
    fn bound_lines_are_width_lengthed() {
        let width = 20_usize;
        let lines = bound_lines(
            width,
            "Test that this is going to come out as a set width and nothing more.",
        );
        assert_eq!(lines[0].len(), width);
    }

    #[test]
    fn crate_search_message_is_the_same_over_time() {
        let message = get_crate_search_message("test");
        assert_eq!(message.len(), 40);
    }
}
