extern crate clap;
mod action;
mod cli;
mod question;
mod statement;

use clap::{App, SubCommand};
use question::{AlwaysNoQuestion, AlwaysYesQuestion, AskQuestion};

fn main() {
    let matches = App::new("promptly")
        .version("1.0")
        .author("James Osler")
        .subcommand(SubCommand::with_name("pr").about("submit pull request"))
        .get_matches();

    match matches.subcommand_name() {
        Some("pr") => {
            statement::while_success(vec![
                &statement::Statement {
                    question: &AskQuestion {
                        ask: "Should I do this?",
                        default: "yes",
                    },
                    action: &action::CommandAction {
                        action: "git",
                        args: &["st"],
                    },
                },
                &statement::Statement {
                    question: &AlwaysYesQuestion {},
                    action: &action::CommandAction {
                        action: "ls",
                        args: &["-l"],
                    },
                },
                &statement::Statement {
                    question: &AlwaysNoQuestion {},
                    action: &action::CommandAction {
                        action: "echo",
                        args: &["'foo'"],
                    },
                },
            ]);
        }
        _ => {
            println!("unknown command");
            std::process::exit(1);
        }
    }
}
