use regex::Regex;

pub fn extract_links(markdown: impl Into<String>) -> (String, Vec<String>) {
    let link_re = Regex::new(r"\[([^\]]+)\]\((https?://[^\)]+)\)").unwrap();
    let markdown_str = markdown.into();
    let mut extracted_text = markdown_str.clone();
    let mut links = Vec::new();

    for (i, cap) in link_re.captures_iter(&markdown_str).enumerate() {
        let link_text = &cap[1];
        let url = &cap[2];
        let link_number = i + 1;

        let placeholder = format!("{}[{}]", link_text, link_number);
        extracted_text = extracted_text.replacen(&cap[0], &placeholder, 1);
        links.push(url.to_string());
    }

    (extracted_text, links)
}

pub fn render_links(markdown: String, links: &[String]) -> String {
    if links.is_empty() {
        return markdown;
    }

    let link_content = links
        .iter()
        .enumerate()
        .map(|(i, url)| format!("[{}] {}", i + 1, url))
        .collect::<Vec<String>>()
        .join("\n");

    format!("{}\n\n{}", markdown, link_content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_link() {
        let input = "Check out [this link](https://example.com).";
        let expected_output = "Check out this link[1].\n\n[1] https://example.com";
        let (output, links) = extract_links(input);
        assert_eq!(render_links(output, &links), expected_output);
    }

    #[test]
    fn test_multiple_links() {
        let input = "See [site one](https://one.com) and [site two](https://two.com).";
        let expected_output =
            "See site one[1] and site two[2].\n\n[1] https://one.com\n[2] https://two.com";
        let (output, links) = extract_links(input);
        assert_eq!(render_links(output, &links), expected_output);
    }

    #[test]
    fn test_no_links() {
        let input = "This string has no links.";
        let expected_output = "This string has no links.";
        let (output, links) = extract_links(input);
        assert_eq!(render_links(output, &links), expected_output);
    }

    #[test]
    fn test_mixed_content() {
        let input = "Here is a [link](https://example.com) with some [text](https://text.com) and more content.";
        let expected_output = "Here is a link[1] with some text[2] and more content.\n\n[1] https://example.com\n[2] https://text.com";
        let (output, links) = extract_links(input);
        assert_eq!(render_links(output, &links), expected_output);
    }

    #[test]
    fn test_links_with_special_characters() {
        let input = "Check [this](https://example.com/special?query=1&val=two) out!";
        let expected_output =
            "Check this[1] out!\n\n[1] https://example.com/special?query=1&val=two";
        let (output, links) = extract_links(input);
        assert_eq!(render_links(output, &links), expected_output);
    }

    #[test]
    fn test_multiple_same_links() {
        let input = "Here is [link1](https://example.com) and here is another [link1](https://example.com).";
        let expected_output = "Here is link1[1] and here is another link1[2].\n\n[1] https://example.com\n[2] https://example.com";
        let (output, links) = extract_links(input);
        assert_eq!(render_links(output, &links), expected_output);
    }

    #[test]
    fn test_empty_link_text() {
        let input = "An empty [link]() should stay as is.";
        let expected_output = "An empty [link]() should stay as is.";
        let (output, links) = extract_links(input);
        assert_eq!(render_links(output, &links), expected_output);
    }

    #[test]
    fn test_empty_input() {
        let input = "";
        let expected_output = "";
        let (output, links) = extract_links(input);
        assert_eq!(render_links(output, &links), expected_output);
    }
}
