# Rusty Text Scanner (rtc)

**Rusty Text Scanner** is a command-line tool written in Rust for scanning text files and highlighting occurrences of a search word. It provides flexibility with search modes and case sensitivity options.

## Installation

To install **Rusty Text Scanner**, use Cargo (Rust's package manager):

```sh
git clone https://github.com/Andromeda-12/rusty-text-scanner
cd rusty-text-scanner
cargo install --path .
```

This will compile the project and install the rtc binary into your Cargo binaries directory.

## Usage

```sh
rtc <file_path> <search_word> [search_mode] [case_sensitive]
```

- `<file_path>`: Path to the text file to be scanned.
- `<search_word>`: Word to search for in the text file.
- `[search_mode]`: Optional. Specify search mode:
  - `exact` (default): Highlight exact matches of the search word.
  - `contains`: Highlight lines containing the search word.
- `[case_sensitive]`: Optional. Specify case sensitivity:
  - `false` (default): Perform a case-insensitive search.
  - `true`: Perform a case-sensitive search.

## Examples

Search for the word "error" in a log file (log.txt):

```sh
rtc log.txt error
```

Search for lines containing "warning" in a case-sensitive manner:

```sh
rtc log.txt warning exact true
```

## Contributing

Contributions are welcome! If you encounter any issues or have suggestions, please open an [issue](https://github.com/Andromeda-12/rusty-text-scanner/issues) on GitHub.
