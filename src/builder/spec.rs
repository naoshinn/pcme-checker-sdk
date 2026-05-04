use std::path::{Path, PathBuf};

use crate::builder::{CheckRule, Diagnostic, runner::run_rule};

pub struct CheckSpec {
    pub id: String,
    pub path: PathBuf,
    pub rules: Vec<CheckRule>,
}

impl CheckSpec {
    pub fn run(&self, project_root: &Path) -> Vec<Diagnostic> {
        let path = project_root.join(&self.path);

        self.rules
            .iter()
            .map(|rule| run_rule(rule, &path))
            .collect()
    }
}
