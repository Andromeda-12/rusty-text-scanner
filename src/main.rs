mod color_utils;
mod consts;

fn main() {
    color_utils::print_highlighted(
        vec!["1: test".to_string(), "2: test".to_string()],
        "тест нетест".to_owned(),
        vec!["4: test".to_string(), "5: test".to_string()],
        "ТЕСТ"
    );
}
