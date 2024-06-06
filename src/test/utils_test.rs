#[cfg(test)]
mod test {
    use crate::utils::split_word_by_substring_inclusive;

    #[test]
    fn test_split_word_by_substring_inclusive() {
        let word = "111test111";
        let substring = "test";
        let expected = vec!["111".to_string(), "test".to_string(), "111".to_string()];

        assert_eq!(expected, split_word_by_substring_inclusive(word, substring));

        let word = "aaaabbbbcccc";
        let substring = "bbbb";
        let expected = vec!["aaaa".to_string(), "bbbb".to_string(), "cccc".to_string()];
        assert_eq!(expected, split_word_by_substring_inclusive(word, substring));

        let word = "aaaabbccbbcc";
        let substring = "bb";
        let expected = vec![
            "aaaa".to_string(),
            "bb".to_string(),
            "cc".to_string(),
            "bb".to_string(),
            "cc".to_string(),
        ];
        assert_eq!(expected, split_word_by_substring_inclusive(word, substring));

        let word = "no_substring_here";
        let substring = "missing";
        let expected = vec!["no_substring_here".to_string()];
        assert_eq!(expected, split_word_by_substring_inclusive(word, substring));

        let word = "repeatedtestrepeatedtestrepeated";
        let substring = "test";
        let expected = vec![
            "repeated".to_string(),
            "test".to_string(),
            "repeated".to_string(),
            "test".to_string(),
            "repeated".to_string(),
        ];
        assert_eq!(split_word_by_substring_inclusive(word, substring), expected);

        let word = "repeatedtest";
        let substring = "";
        let expected = vec!["repeatedtest".to_string()];
        assert_eq!(split_word_by_substring_inclusive(word, substring), expected);
    }
}
