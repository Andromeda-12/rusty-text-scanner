use std::{
    fs::File,
    io::{self, BufRead, BufReader, Read},
};

use crate::consts::consts::CHUNK_SIZE;

pub struct FileReader {
    file: BufReader<File>,
}

impl FileReader {
    pub fn new(file_path: String) -> io::Result<Self> {
        let file = File::open(&file_path)?;
        let reader = BufReader::new(file);
        Ok(Self { file: reader })
    }

    pub fn read_next_chunk(&mut self) -> io::Result<Vec<String>> {
        let mut buffer = Vec::new();
        for line in self.file.by_ref().lines().take(CHUNK_SIZE) {
            let line = line?;
            buffer.push(line);
        }
        Ok(buffer)
    }
}
