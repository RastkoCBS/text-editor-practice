use std::cmp::min;
use unicode_segmentation::UnicodeSegmentation;

use crate::Position;

#[derive(Debug)]
#[derive(Default)]
pub struct Row {
    text: String,
    len: usize,
}

impl From<&str> for Row {
    fn from(slice: &str) -> Self {
        let mut row = Self {
            text: String::from(slice), 
            len: 0,
        };
        row.update_len();
        row
    }
}

impl Row {
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = min(end, self.text.len());
        let start = min(start, end);

        let mut result = String::new();
        #[allow(clippy::integer_arithmetic)]
        for grapheme in self.text[..].graphemes(true).skip(start).take(end - start) {

            if grapheme == "\t" {
                result.push_str("  ");
            } else {
                result.push_str(grapheme);
            }
        }

        result
    }

    pub fn insert(&mut self, at: usize, c: char) {
        if at >= self.len() {
            self.text.push(c);
        } else {
            let mut result: String = self.text[..].graphemes(true).take(at).collect();
            let remainder: String = self.text[..].graphemes(true).skip(at).collect();

            result.push(c);
            result.push_str(&remainder);

            self.text = result;
        }

        self.update_len();
    }
    
    #[allow(clippy::integer_arithmetic)]
    pub fn delete(&mut self, at: usize) {
        if at >= self.len() {
            return;
        } else {
            let mut result: String = self.text[..].graphemes(true).take(at).collect();
            let remainder: String = self.text[..].graphemes(true).skip(at + 1).collect();

            result.push_str(&remainder); 

            self.text = result;
        }
        self.update_len();
    }

    pub fn append(&mut self, new: &Self) {
        self.text = format!("{}{}", self.text, new.text);
        self.update_len();
    }

    pub fn split(&mut self, at:usize) -> Self {
        let start: String = self.text[..].graphemes(true).take(at).collect();
        let remainder: String = self.text[..].graphemes(true).skip(at).collect();

        self.text = start;
        self.update_len();
        Self::from(&remainder[..])
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn update_len(&mut self) {
        self.len = self.text[..].graphemes(true).count();
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.text.as_bytes()
    }
}