use distance::levenshtein;

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SearchResult {
    word: String,
    value: usize,
    path: String,
}

/* pub fn fuzzy_search() {
} */

/* #[test]
fn test_fuzzy_search() {
    println!("Running fuzzy search test...");
    let distance = levenshtein("this test", "tt");
    println!("Distance: {}", distance);
} */

#[test]
fn test_fuzzy_search() {
    let file_name = "test".to_lowercase();
    let query = "t*st".to_lowercase();
    /* let score = fuzzy_scrore(&file_name, &query);
    println!("Fuzzy score: {}", score); */
    char_match(&file_name, &query);
}

fn fuzzy_scrore(file_name: &str, query: &str) -> usize {
    if file_name == query {
        return 100;
    } else if file_name.contains(query) {
        return 50;
    } else {
        return 0;
    }
}

fn char_match(file_name: &str, query: &str) {
    let mut score = 0;
    let file_chars = file_name.chars();
    let mut query_chars = query.chars();

    let mut current_char = query_chars.next();

    for file_char in file_chars {
        if let Some(query_char) = current_char {
            if file_char.eq_ignore_ascii_case(&query_char) {
                println!("Matched char: {}", file_char);
                score += 1;
            }
        }
        current_char = query_chars.next();
    }
    println!("Total score: {}", score);
}
