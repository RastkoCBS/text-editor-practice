use crate::{row, Row}; 
use std::io::{stdin, stdout, Error, Write};
use std::fs;

#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
}

impl Document {
    pub fn open(filename: &str) -> Result<Self, Error> {
        let content = fs::read_to_string(filename)?;
        let mut rows = Vec::new();

        for value in content.lines() {
            rows.push(Row::from(value));
        }
        
        Ok(Self { rows })
    }

    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }
}