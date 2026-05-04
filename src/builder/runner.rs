use std::path::Path;

use crate::builder::{CheckRule, Diagnostic};

pub fn run_rule(rule: &CheckRule, path: &Path) -> Diagnostic {
    match rule {
        CheckRule::FileExists => {
            if path.exists() {
                Diagnostic::ok("file exists")
            } else {
                Diagnostic::stop("file is missing")
            }
        }
    }
}
