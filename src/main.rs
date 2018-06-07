extern crate clap;
use clap::{App, SubCommand};
mod cli;
mod statement;

fn main() {
    println!("Hello, world!");

    let matches = App::new("promptly")
        .version("1.0")
        .author("James Osler")
        .subcommand(SubCommand::with_name("pr").about("submit pull request"))
        .get_matches();

    match matches.subcommand_name() {
        Some("pr") => {
            println!("pr");
            statement::while_success(vec![
                &statement::Statement {
                    action: "git st".to_string(),
                    question: &statement::AskQuestion {},
                },
                &statement::Statement {
                    action: "ls".to_string(),
                    question: &statement::YesQuestion {},
                },
                &statement::Statement {
                    action: "echo 'foo'".to_string(),
                    question: &statement::NoQuestion {},
                },
            ]);
        }
        _ => {
            println!("unknown command");
            std::process::exit(1);
        }
    }
}
