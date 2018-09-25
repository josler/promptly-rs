extern crate clap;
extern crate failure;

mod cli;
mod question;
use std::collections::HashMap;

use clap::{App, SubCommand};
use question::*;

fn main() {
    let matches = App::new("promptly")
        .version("1.0")
        .author("James Osler")
        .subcommand(SubCommand::with_name("pr").about("submit pull request"))
        .get_matches();
    match matches.subcommand_name() {
        Some("pr") => {
            while_question_success(vec![
                &Info{
                    ask: "Description?",
                    default: "",
                },
                &PRBranch {
                    ask: "Branch name?",
                    default: "Description?",
                },
                &CommitText {
                    ask: "Commit text?",
                    default: "Description?",
                },
                &Ask{
                    ask: "Push to remote?",
                },
                &Command {
                    action: "git",
                    args: &["push", "-u"],
                },
                &Ask {
                    ask: "Create PR?",
                },
                &CreatePR {
                    ask: "PR title?",
                    default: "Description?",
                }
            ]);
        }
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
            Ok(_) => {},
        }
    }
}