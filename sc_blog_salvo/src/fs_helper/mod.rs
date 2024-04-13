use std::{
    io::{self, BufRead, BufReader},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

pub fn read_to_string(path: &str) -> String {
    if let Ok(content) = std::fs::read_to_string(path) {
        content
    } else {
        read_404()
    }
}

fn read_404() -> String {
    if let Ok(content) = std::fs::read_to_string("src/static/html/404.html") {
        return content;
    } else {
        return "Something went wrong reading the file".to_string();
    }
}

pub fn render_md_to_catalog(pre: Option<usize>, size: Option<usize>) -> String {
    let path = "src/static/md/catalog.md";
    if let Ok(file) = std::fs::File::open(path) {
        let reader = BufReader::new(file);
        let mut lines = reader.lines();
        let mut content = String::new();
        let mut count = 0;
    } else {
        read_404()
    }
}

pub fn render_md_to_html(name: &str) -> String {
    let path = format!("src/static/md/{}.md", name);
    if let Ok(content) = std::fs::read_to_string(&path) {
        markdown::to_html(&content)
    } else {
        read_404()
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MdInfo {
    title: String,
    date: String,
    tags: Vec<String>,
    writers: Vec<String>,
    summary: String,
}
