fn or_match_pattern_example() {
    let x = Some(2u8);
    // Before
    matches!(x, Some(1) | Some(2));
    // Now
    matches!(x, Some(1 | 2));
}
