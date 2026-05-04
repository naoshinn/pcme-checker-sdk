use std::fmt;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Severity {
    Ok,
    Warn,
    Fail,
    Stop,
}

impl Severity {
    pub fn rank(&self) -> i32 {
        match self {
            Self::Ok => 0,
            Self::Warn => 1,
            Self::Fail => 2,
            Self::Stop => 3,
        }
    }
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let kind = match self {
            Self::Ok => "OK",
            Self::Warn => "WARN",
            Self::Fail => "FAIL",
            Self::Stop => "STOP",
        };

        write!(f, "{kind}")
    }
}

pub struct Summary {
    pub severity: Severity,
    pub path: String,
}

impl Summary {
    pub fn ok(path: impl Into<String>) -> Self {
        Self {
            severity: Severity::Ok,
            path: path.into(),
        }
    }

    pub fn warn(path: impl Into<String>) -> Self {
        Self {
            severity: Severity::Warn,
            path: path.into(),
        }
    }

    pub fn fail(path: impl Into<String>) -> Self {
        Self {
            severity: Severity::Fail,
            path: path.into(),
        }
    }

    pub fn stop(path: impl Into<String>) -> Self {
        Self {
            severity: Severity::Stop,
            path: path.into(),
        }
    }
}

impl fmt::Display for Summary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", self.severity, self.path)
    }
}

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub severity: Severity,
    pub message: String,
}

impl Diagnostic {
    pub fn ok(message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Ok,
            message: message.into(),
        }
    }

    pub fn warn(message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Warn,
            message: message.into(),
        }
    }

    pub fn fail(message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Fail,
            message: message.into(),
        }
    }

    pub fn stop(message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Stop,
            message: message.into(),
        }
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", self.severity, self.message)
    }
}

pub struct CheckReport {
    pub summary: Summary,
    pub details: Vec<Diagnostic>,
}

impl CheckReport {
    pub fn from_diagnostic(summary: Summary) -> Self {
        Self {
            summary,
            details: Vec::new(),
        }
    }
}

impl fmt::Display for CheckReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.details.is_empty() {
            return write!(f, "{}", self.summary);
        }

        let details_str = self
            .details
            .iter()
            .map(|detail| format!("  - {}", detail))
            .collect::<Vec<_>>()
            .join("\n");

        write!(f, "{}\n{}", self.summary, details_str)
    }
}
