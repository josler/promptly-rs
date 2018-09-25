use cli;
use std::collections::HashMap;
use failure::{Error};

pub trait Question {
    fn ask(&self, context: &mut HashMap<String, String>) -> Result<String, Error>;
}

pub struct Info<'a> {
    pub ask: &'a str,
    pub default: &'a str,
}

impl<'a> Question for Info<'a> {
    fn ask(&self, context: &mut HashMap<String, String>) -> Result<String, Error> {
        let c = cli::CLI::new();
        let result = c.question(self.ask, self.default)?;
        context.insert(self.ask.to_string(), result.clone());
        Ok(result)
    }
}

pub struct Ask<'a> {
    pub ask: &'a str,
}

impl<'a> Question for Ask<'a> {
    fn ask(&self, _context: &mut HashMap<String, String>) -> Result<String, Error> {
        let c = cli::CLI::new();
        c.yes_no_question(self.ask, "yes")?;
        Ok("yes".to_string())
    }
}

pub struct Command<'a> {
    pub action: &'a str,
    pub args: &'a[&'a str]
}

impl<'a> Question for Command<'a> {
    fn ask(&self, _context: &mut HashMap<String, String>) -> Result<String, Error> {
        let c = cli::CLI::new();
        let res = c.run_command(self.action, self.args)?;
        Ok(res)
    }
}

pub struct PRBranch<'a> {
    pub ask: &'a str,
    pub default: &'a str,
}

impl<'a> Question for PRBranch<'a> {
    fn ask(&self, context: &mut HashMap<String, String>) -> Result<String, Error> {
        let c = cli::CLI::new();
        let default = get_default(context, self.default);
        let branch_name = c.question(self.ask, &format!("jo/{}", &default.replace(" ", "-"))).unwrap();
        context.insert(
            self.ask.to_string(),
            branch_name.clone(),
        );
        let res = c.run_command("git", &["checkout", "-b", &branch_name])?;
        Ok(res)
    }
}

pub struct CommitText<'a> {
    pub ask: &'a str,
    pub default: &'a str,
}

impl<'a> Question for CommitText<'a> {
    fn ask(&self, context: &mut HashMap<String, String>) -> Result<String, Error> {
        let c = cli::CLI::new();
        let commit_text = get_set_existing(&c, context, self.ask, self.default);
        c.run_command("git", &["add", "."])?;
        let res = c.run_command("git", &["commit", "-m", &commit_text])?;
        Ok(res)
    }
}

pub struct CreatePR<'a> {
    pub ask: &'a str,
    pub default: &'a str,
}

impl<'a> Question for CreatePR<'a> {
    fn ask(&self, context: &mut HashMap<String, String>) -> Result<String, Error> {
        let c = cli::CLI::new();
        let pr_text = get_set_existing(&c, context, self.ask, self.default);
        let res = c.run_command("hub", &["pull-request", "-m", &pr_text, "-o"])?;
        Ok(res)
    }
}

fn get_default<'a>(context: &'a mut HashMap<String, String>, key: &str) -> String {
    context
        .entry(key.to_string())
        .or_insert("pr description".to_string()).to_string()
}

fn get_set_existing(cli: &cli::CLI, context: &mut HashMap<String, String>, ask: &str, default: &str) -> String {
    let default = get_default(context, default);
    let existing = cli.question(ask, &default).unwrap();
    context.insert(
        ask.to_string(),
        existing.clone(),
    );
    existing
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format() {
        assert_eq!(&format!("{}!", "my lovely horse"), "my lovely horse!");
    }

    #[test]
    fn test_run_command_spaces() {
        let c = cli::CLI::new();
        c.run_command("ag", &["lovely horse"]).unwrap();
        assert_eq!("", "");
    }
}
