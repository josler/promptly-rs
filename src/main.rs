extern crate clap;
#[macro_use]
extern crate failure_derive;
extern crate failure;

mod action;
mod cli;
mod question;
mod statement;

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
            statement::while_success(vec![
                &statement::Statement {
                    question: &InfoQuestion {
                        ask: "PR Description?",
                        default: "",
                    },
                    action: None,
                },
                &statement::Statement {
                    question: &BranchQuestion { ask: "Foo" },
                    action: None,
                },
                &statement::Statement {
                    question: &AlwaysYesQuestion {},
                    action: Some(&action::CommandAction {
                        action: "ls",
                        args: &["-l"],
                    }),
                },
                &statement::Statement {
                    question: &AlwaysNoQuestion {},
                    action: Some(&action::CommandAction {
                        action: "echo",
                        args: &["'foo'"],
                    }),
                },
            ]);
        }
        _ => {
            println!("unknown command");
            std::process::exit(1);
        }
    }
}
