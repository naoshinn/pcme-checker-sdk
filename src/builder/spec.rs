use std::path::{Path, PathBuf};

use crate::builder::{CheckReport, CheckRule, Diagnostic, Summary, runner::run_rule};

pub struct CheckSpec {
    pub id: String,
    pub path: PathBuf,
    pub rules: Vec<CheckRule>,
}

impl CheckSpec {
    pub fn run(&self, project_root: &Path) -> CheckReport {
        let path = project_root.join(&self.path);

        let details: Vec<Diagnostic> = self
            .rules
            .iter()
            .map(|rule| run_rule(rule, &path))
            .collect();

        let max_rank = details
            .iter()
            .map(|detail| detail.severity.rank())
            .max()
            .unwrap_or(0);

        let path = self.path.display().to_string();
        let summary = match max_rank {
            0 => Summary::ok(path),
            1 => Summary::warn(path),
            2 => Summary::fail(path),
            _ => Summary::stop(path),
        };

        CheckReport { summary, details }
    }
}
