use crate::types::SearchMode;

#[derive(Debug)]
pub struct Config {
    pub file_path: String,
    pub search_word: String,
    pub search_mode: SearchMode,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments. Usage: rtc <file_path> <search_word> [search_mode] [case_sensitive]");
        }

        let file_path = args[1].clone();
        let search_word = args[2].clone();

        let search_mode = args
            .get(3)
            .map_or(SearchMode::Contains, |arg| match arg.as_str() {
                "exact" => SearchMode::Exact,
                "contains" => SearchMode::Contains,
                _ => {
                    eprintln!(
                    "Invalid search mode. Use 'exact' or 'contains'. Defaulting to 'contains'."
                );
                    SearchMode::Contains
                }
            });

        let case_sensitive = args.get(4).map_or(true, |arg| match arg.as_str() {
            "false" => false,
            "true" => true,
            _ => {
                eprintln!(
                    "Invalid case sensitive value. Use 'true' or 'false'. Defaulting to 'true'."
                );
                true
            }
        });

        Ok(Config {
            file_path,
            search_word,
            search_mode,
            case_sensitive,
        })
    }
}
