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
            // let statements = vec![];
            // statements.push(&statement::YesQuestion {});
            statement::while_success(vec![
                &statement::Statement {
                    name: "foo".to_string(),
                    question: &statement::YesQuestion {},
                },
                &statement::Statement {
                    name: "bar".to_string(),
                    question: &statement::NoQuestion {},
                },
            ]);
        }
        _ => {
            println!("unknown command");
            std::process::exit(1);
        }
    }
    let c = cli::CLI::new();
    c.agree("foo");
    c.say_green("bar");
}
