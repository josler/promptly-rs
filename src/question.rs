use cli;
use std::collections::HashMap;

pub trait Question {
    fn ask(&self, context: &mut HashMap<String, String>) -> Option<&str>;
}

pub struct AlwaysYesQuestion {}

impl Question for AlwaysYesQuestion {
    fn ask(&self, _context: &mut HashMap<String, String>) -> Option<&str> {
        Some("yes")
    }
}

pub struct AlwaysNoQuestion {}

impl Question for AlwaysNoQuestion {
    fn ask(&self, _context: &mut HashMap<String, String>) -> Option<&str> {
        None
    }
}

pub struct AskQuestion<'a> {
    pub ask: &'a str,
    pub default: &'a str,
}

impl<'a> Question for AskQuestion<'a> {
    fn ask(&self, context: &mut HashMap<String, String>) -> Option<&str> {
        let c = cli::CLI::new();
        return match c.yes_no_question(self.ask, self.default) {
            Ok(true) => {
                context
                    .entry("foo".to_string())
                    .or_insert("bar".to_string());
                Some("yes")
            }
            Ok(false) => None,
            Err(_) => None,
        };
    }
}
