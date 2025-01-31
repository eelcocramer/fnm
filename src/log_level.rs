#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
pub enum LogLevel {
    Quiet,
    Error,
    Info,
}

impl LogLevel {
    pub fn is_writable(&self, logging: &Self) -> bool {
        use std::cmp::Ordering;
        matches!(self.cmp(logging), Ordering::Greater | Ordering::Equal)
    }

    pub fn writer_for(&self, logging: &Self) -> Box<dyn std::io::Write> {
        if self.is_writable(logging) {
            match logging {
                Self::Error => Box::from(std::io::stderr()),
                _ => Box::from(std::io::stderr()),
            }
        } else {
            Box::from(std::io::sink())
        }
    }

    pub fn possible_values() -> &'static [&'static str; 4] {
        &["quiet", "info", "all", "error"]
    }
}

impl From<LogLevel> for &'static str {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Quiet => "quiet",
            LogLevel::Info => "info",
            LogLevel::Error => "error",
        }
    }
}

impl std::str::FromStr for LogLevel {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<LogLevel, Self::Err> {
        match s {
            "quiet" => Ok(Self::Quiet),
            "info" | "all" => Ok(Self::Info),
            "error" => Ok(Self::Error),
            _ => Err("Unsupported log level"),
        }
    }
}

#[macro_export]
macro_rules! outln {
    ($config:ident, $level:path, $($expr:expr),+) => {{
        use $crate::log_level::LogLevel::*;
        writeln!($config.log_level().writer_for(&$level), $($expr),+).expect("Can't write output");
    }}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_writable() {
        assert!(!LogLevel::Quiet.is_writable(&LogLevel::Info));
        assert!(!LogLevel::Error.is_writable(&LogLevel::Info));
        assert!(LogLevel::Info.is_writable(&LogLevel::Info));
        assert!(LogLevel::Info.is_writable(&LogLevel::Error));
    }
}
