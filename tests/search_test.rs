extern crate minigrep;

#[cfg(test)]
mod tests {
    use minigrep::*;

    // This tests should be edited because some logics were changed.
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.\nDuct tape"
            .lines()
            .map(|s| s.to_string())
            .enumerate()
            .collect();

        let expect_match = vec![1]
            .into_iter()
            .zip(vec!["safe, fast, productive.".to_string()])
            .collect::<Vec<_>>();

        assert_eq!(expect_match, search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.\nTrust me."
            .lines()
            .map(|s| s.to_string())
            .enumerate()
            .collect();

        let expect_match = vec![0, 3]
            .into_iter()
            .zip(vec!["Rust:".to_string(), "Trust me.".to_string()])
            .collect::<Vec<_>>();

        assert_eq!(expect_match, search_ignore_case(query, contents));
    }
}
