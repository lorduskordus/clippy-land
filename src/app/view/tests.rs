use super::summary::summarize_one_line;

#[test]
fn summarizes_first_nonempty_line() {
    let input = "\n   \n  hello world  \nsecond line";
    assert_eq!(summarize_one_line(input), "hello world");
}

#[test]
fn truncates_long_lines_with_ellipsis() {
    let input = "abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnop";
    assert_eq!(
        summarize_one_line(input),
        "abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefg…"
    );
}

#[test]
fn returns_empty_for_blank_text() {
    assert_eq!(summarize_one_line("\n  \n\t"), "");
}
