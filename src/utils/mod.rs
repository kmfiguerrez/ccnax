pub mod db_models;

pub mod text_command;

pub fn format_section_title(slug: &str) -> String {
    // 1. Replace "tcp-ip" before splitting by hyphens
    let fixed_slug = slug.to_lowercase().replace("tcp-ip", "TCP/IP");

    fixed_slug
        .split('-')
        .filter(|word| !word.is_empty())
        .map(|word| match word {
            // 2. Keep special acronyms formatted as-is
            "TCP/IP" => word.to_string(),
            "osi" => "OSI".to_string(),
            "http" => "HTTP".to_string(),
            "dns" => "DNS".to_string(),
            
            // 3. Capitalize standard words
            _ => {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => {
                        // If a word is inside parentheses, uppercase the first letter after the opening parenthesis.
                        if first == '(' {
                            let mut chars = chars.as_str().chars();
                            match chars.next() {
                                None => String::new(),
                                Some(inner_first) => {
                                    let capitalized_inner = inner_first.to_uppercase().collect::<String>();
                                    format!("({}{}", capitalized_inner, chars.as_str())
                                }
                            }
                        } else {
                            first.to_uppercase().collect::<String>() + chars.as_str()
                        }
                    },
                }
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}


/// Converts a string into a slug format (lowercase, hyphen-separated)
pub fn slugify(text: &str) -> String {
    text.to_lowercase()
        // 1. Convert slashes, dashes, and underscores to spaces
        .replace(['/', '-', '_'], " ")
        .chars()
        // 2. Keep only letters, numbers, spaces and parentheses (strips !, ?, &, etc.)
        .filter(|c| ['(', ')'].contains(c) || c.is_alphanumeric() || c.is_whitespace())
        .collect::<String>()
        // 3. Split by whitespace (this automatically groups multiple spaces into one)
        .split_whitespace()
        .collect::<Vec<&str>>()
        // 4. Join the words back together with a single hyphen
        .join("-")
}


