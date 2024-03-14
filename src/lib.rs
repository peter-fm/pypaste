fn line_starts_with_non_whitespace(line: &str) -> bool {
    line.chars().next().map_or(false, |c| !c.is_whitespace())
}

fn adjust_leading_whitespace(input: &str) -> String {
    let leading_whitespace_count = input
        .lines()
        .find_map(|line| {
            if !line.trim().is_empty() {
                Some(line.chars().take_while(|c| c.is_whitespace()).count())
            } else {
                None
            }
        })
        .unwrap_or(0);

    input
        .lines()
        .map(|line| {
            if line.len() >= leading_whitespace_count {
                &line[leading_whitespace_count..]
            } else {
                line
            }
        })
        .collect::<Vec<&str>>()
        .join("\n")
}

pub fn process_string(input: &str) -> String {
    let input = adjust_leading_whitespace(input);
    let mut result = Vec::new();
    let mut previous_line: Option<String> = None;
    for line in input.lines() {
        let line = line.replace('\t', "    ");
        if line.trim().is_empty() {
            continue;
        }
        if line_starts_with_non_whitespace(&line) && !result.is_empty() {
            if let Some(previous) = previous_line {
                if !line_starts_with_non_whitespace(previous.as_str()) {
                    result.push("\n".to_string())
                }
            }
        }
        previous_line = Some(line.clone());
        result.push(line);
    }
    result.push("".to_string());
    result.join("\n")
}
