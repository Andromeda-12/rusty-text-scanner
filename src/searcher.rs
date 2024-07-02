use crate::{
    consts::consts::CHUNK_SIZE,
    types::{LineData, SearchMode, SearcherTrait},
};

pub struct Searcher {
    mode: SearchMode,
    case_sensitive: bool,
}

impl SearcherTrait for Searcher {
    fn new(mode: SearchMode, case_sensitive: bool) -> Self {
        Self {
            mode,
            case_sensitive,
        }
    }

    fn search(
        &self,
        data: &Vec<String>,
        searched_substring: &String,
        line_num_offset: usize,
    ) -> Vec<LineData> {
        match self.mode {
            SearchMode::Exact => {
                if self.case_sensitive {
                    return self.search_full_match_case_sensitive(
                        data,
                        searched_substring,
                        line_num_offset,
                    );
                }

                self.search_full_match(data, searched_substring, line_num_offset)
            }
            SearchMode::Contains => {
                if self.case_sensitive {
                    return self.search_substring_case_sensitive(
                        data,
                        searched_substring,
                        line_num_offset,
                    );
                }

                return self.search_substring(data, searched_substring, line_num_offset);
            }
        }
    }

    fn search_full_match(
        &self,
        lines: &Vec<String>,
        searched_substring: &String,
        line_num_offset: usize,
    ) -> Vec<LineData> {
        let lower_cased_searched_substring: String = searched_substring.to_lowercase();

        lines
            .into_iter()
            .enumerate()
            .map(|(line_number, line)| {
                let mut indices = Vec::new();
                let mut search_start_idx = 0;
                let lower_cased_line = line.to_lowercase();

                lower_cased_line
                    .split_whitespace()
                    .into_iter()
                    .for_each(|word| {
                        if word == lower_cased_searched_substring.as_str() {
                            if let Some(word_start) = lower_cased_line[search_start_idx..]
                                .find(&lower_cased_searched_substring)
                            {
                                let word_start_idx = search_start_idx + word_start;
                                let word_end_idx =
                                    word_start_idx + lower_cased_searched_substring.len();
                                indices.push(word_start_idx..word_end_idx);
                                search_start_idx = word_end_idx;
                            }
                        }
                    });

                LineData {
                    line_number: line_num_offset * CHUNK_SIZE + line_number + 1,
                    string: line.to_string(),
                    indices,
                }
            })
            .collect()
    }

    fn search_full_match_case_sensitive(
        &self,
        lines: &Vec<String>,
        searched_substring: &String,
        line_num_offset: usize,
    ) -> Vec<LineData> {
        lines
            .into_iter()
            .enumerate()
            .map(|(line_number, line)| {
                let mut indices = Vec::new();
                let mut search_start_idx = 0;

                line.split_whitespace().into_iter().for_each(|word| {
                    if word == searched_substring.as_str() {
                        if let Some(word_start) = line[search_start_idx..].find(searched_substring)
                        {
                            let word_start_idx = search_start_idx + word_start;
                            let word_end_idx = word_start_idx + searched_substring.len();
                            indices.push(word_start_idx..word_end_idx);
                            search_start_idx = word_end_idx;
                        }
                    }
                });

                LineData {
                    line_number: line_num_offset * CHUNK_SIZE + line_number + 1,
                    string: line.to_string(),
                    indices,
                }
            })
            .collect()
    }

    fn search_substring(
        &self,
        lines: &Vec<String>,
        searched_substring: &String,
        line_num_offset: usize,
    ) -> Vec<LineData> {
        let lower_cased_searched_substring: String = searched_substring.to_lowercase();

        lines
            .into_iter()
            .enumerate()
            .map(|(line_number, line)| {
                let mut indices = Vec::new();
                let mut pos = 0;
                let lower_cased_line = line.to_lowercase();

                while let Some(start) =
                    lower_cased_line[pos..].find(&lower_cased_searched_substring)
                {
                    let start_idx = pos + start;
                    let end_idx = start_idx + lower_cased_searched_substring.len();
                    indices.push(start_idx..end_idx);
                    pos = end_idx;
                }

                LineData {
                    line_number: line_num_offset * CHUNK_SIZE + line_number + 1,
                    string: line.to_string(),
                    indices,
                }
            })
            .collect()
    }

    fn search_substring_case_sensitive(
        &self,
        lines: &Vec<String>,
        searched_substring: &String,
        line_num_offset: usize,
    ) -> Vec<LineData> {
        lines
            .into_iter()
            .enumerate()
            .map(|value| {
                let mut indices = Vec::new();
                let mut pos = 0;

                while let Some(start) = value.1[pos..].find(searched_substring) {
                    let start_idx = pos + start;
                    let end_idx = start_idx + searched_substring.len();
                    indices.push(start_idx..end_idx);
                    pos = end_idx;
                }

                LineData {
                    line_number: line_num_offset * CHUNK_SIZE + value.0 + 1,
                    string: value.1.to_string(),
                    indices,
                }
            })
            .collect()
    }
}
