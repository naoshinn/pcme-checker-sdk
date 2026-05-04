use std::path::PathBuf;

use crate::builder::{CheckRule, CheckSpec};

pub struct CheckerBuilder {
    id: String,
    path: PathBuf,
    rules: Vec<CheckRule>,
}

impl CheckerBuilder {
    pub fn new(id: impl Into<String>, path: PathBuf) -> Self {
        Self {
            id: id.into(),
            path,
            rules: Vec::new(),
        }
    }

    pub fn file_exists(mut self) -> Self {
        self.rules.push(CheckRule::FileExists);
        self
    }

    pub fn build(self) -> CheckSpec {
        CheckSpec {
            id: self.id,
            path: self.path,
            rules: self.rules,
        }
    }
}
