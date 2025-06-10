use distance::levenshtein;

/* pub fn fuzzy_search() {
} */

#[test]
fn test_fuzzy_search() {
    println!("Running fuzzy search test...");
    let distance = levenshtein("this test", "tt");
    println!("Distance: {}", distance);
    println!("something test");
}
