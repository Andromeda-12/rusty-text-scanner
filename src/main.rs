use std::env;

use config::Config;
use consts::consts::CHUNK_SIZE;
use reader::FileReader;
use searcher::Searcher;
use types::{SearcherTrait, WriterTrait};
use writer::Writer;

mod config;
mod consts;
mod reader;
mod searcher;
mod types;
mod writer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args)?;

    let mut reader = FileReader::new(config.file_path)?;
    let searcher = Searcher::new(config.search_mode, config.case_sensitive);
    let mut writer = Writer::new();

    let mut iteration = 0;
    loop {
        let lines = reader.read_next_chunk()?;

        let res = searcher.search(&lines, &config.search_word, iteration);

        for r in &res {
            writer.print_painted_line_with_highlighted_substring(r)?;
        }

        if lines.len() < CHUNK_SIZE {
            break;
        }

        iteration += 1;
    }

    Ok(())
}
