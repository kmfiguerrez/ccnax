//! This is database models are for demostration purposes only.
//! For real application, use a database.
use serde::Deserialize;
use std::collections::BTreeMap;

// Notice that the root is no longer a Vec, but a Map where the key is the Volume Number (u32)
pub type Database = BTreeMap<u32, Volume>;


#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct Volume {
    // name: String,
    // The key is the chapter_number
    pub parts: BTreeMap<u32, Part>, 
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct Part {
    pub name: String,
    pub chapters: BTreeMap<u32, Chapter>
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct Chapter {
    pub name: String,
    // The key is the section_number
    pub sections: BTreeMap<u32, Section>,
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct Section {
    pub name: String,
    // The key is the subheader_number
    pub subheaders: BTreeMap<u32, Subheader>,
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct Subheader {
    pub name: String,
    // pub content: String,
}