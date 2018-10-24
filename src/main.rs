extern crate clap;
extern crate dirs;
extern crate failure;
extern crate termcolor;
extern crate toml;
#[macro_use]
extern crate serde_derive;

mod cli;
mod config;
mod question;
use std::collections::HashMap;

use clap::{App, SubCommand};
use question::*;

fn main() {
    let config = config::Config::new();
    let matches = App::new("promptly")
        .version("1.0")
        .author("James Osler")
        .subcommand(SubCommand::with_name("pr").about("submit pull request"))
        .subcommand(SubCommand::with_name("ci").about("check github ci status"))
        .subcommand(SubCommand::with_name("config").about("config"))
        .get_matches();
    match matches.subcommand_name() {
        Some("pr") => {
            while_question_success(vec![
                &Info {
                    ask: "Description?",
                    default: "",
                },
                &PRBranch {
                    ask: "Branch name?",
                    default: "Description?",
                    prefix: &config.prefix,
                },
                &CommitText {
                    ask: "Commit text?",
                    default: "Description?",
                },
                &Ask {
                    ask: "Push to remote?",
                },
                &Command {
                    action: "git",
                    args: &["push", "-u"],
                },
                &Ask { ask: "Create PR?" },
                &CreatePR {
                    ask: "PR title?",
                    default: "Description?",
                },
            ]);
        }
        Some("ci") => while_question_success(vec![
            &Command {
                action: "hub",
                args: &["ci-status", "-v"],
            },
        ]),
        _ => {
            println!("unknown command");
            std::process::exit(1);
        }
    }
}

fn while_question_success<'a>(questions: Vec<&Question>) {
    let mut context = HashMap::new();
    for question in questions {
        match question.ask(&mut context) {
            Err(_) => return,
            Ok(_) => {}
        }
    }
}
