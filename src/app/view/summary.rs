pub(super) fn summarize_one_line(text: &str) -> String {
    let mut line = text
        .lines()
        .map(|line| line.trim_start())
        .find(|line| !line.is_empty())
        .unwrap_or("")
        .trim_end()
        .to_string();
    const MAX_CHARS: usize = 60;
    if line.chars().count() > MAX_CHARS {
        line = line.chars().take(MAX_CHARS - 1).collect::<String>();
        line.push('…');
    }
    line
}
