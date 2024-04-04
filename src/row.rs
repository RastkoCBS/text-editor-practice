use std::cmp::min;  

#[derive(Debug)]
pub struct Row {
    text: String,
}

impl From<&str> for Row {
    fn from(slice: &str) -> Self {
        Self { text: String::from(slice), }
    }
}

impl Row {
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = min(end, self.text.len());
        let start = min(start, end);
        self.text.get(start..end).unwrap_or_default().to_string()
    }
}