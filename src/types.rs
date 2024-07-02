use std::{io, ops::Range};

#[derive(Debug)]
pub enum SearchMode {
    Exact,
    Contains,
}

#[derive(Debug)]
pub struct LineData {
    pub line_number: usize,
    pub string: String,
    pub indices: Vec<Range<usize>>,
}

pub trait WriterTrait {
    fn new() -> Self;
    fn print_painted_line_with_highlighted_substring(&mut self, line: &LineData) -> io::Result<()>;
}

pub trait SearcherTrait {
    fn new(mode: SearchMode, case_sensitive: bool) -> Self;
    fn search(
        &self,
        data: &Vec<String>,
        searched_substring: &String,
        line_num_offset: usize,
    ) -> Vec<LineData>;
    fn search_full_match(
        &self,
        data: &Vec<String>,
        searched_substring: &String,
        line_num_offset: usize,
    ) -> Vec<LineData>;
    fn search_full_match_case_sensitive(
        &self,
        data: &Vec<String>,
        searched_substring: &String,
        line_num_offset: usize,
    ) -> Vec<LineData>;
    fn search_substring(
        &self,
        data: &Vec<String>,
        searched_substring: &String,
        line_num_offset: usize,
    ) -> Vec<LineData>;
    fn search_substring_case_sensitive(
        &self,
        data: &Vec<String>,
        searched_substring: &String,
        line_num_offset: usize,
    ) -> Vec<LineData>;
}
