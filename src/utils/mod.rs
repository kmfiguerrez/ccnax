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

pub struct Chapter<'a> {
    pub title: &'a str,
    pub slug: u32
}


/// Returns a list of chapters for a given volume ID.
pub fn list_chapters(volume_id: &u32) -> &[Chapter] {
    match volume_id {
        1 => &[Chapter { title: "Chapter 1: Introduction to Networking", slug: 1 }],
        _ => &[Chapter { title: "Chapter 9: Device Management Protocols", slug: 9 }],
    }
}

/// Returns a list of sections for a given volume and chapter ID.
/// 
/// For demonstration purposes, we return a static list of sections based on the volume and chapter IDs.
/// In a real application, you might fetch this data from a database or an API.
// pub fn list_sections(volume_id: u32, chapter_id: u32) -> Vec<String> {
pub fn list_sections(volume_id: u32, chapter_id: &u32) -> &[&str] {

    match (volume_id, chapter_id) {
        (1, 1) => &[
            "introduction",
            "networking-basics",
            "protocols-overview",
        ],
        (2, 9) => &[
            "System Message Logging (Syslog)",
            "Network Time Protocol (NTP)",
            "Analyzing Topology Using CDP and LLDP",
        ],
        _ => &["section-1", "section-2"],
    }
}

