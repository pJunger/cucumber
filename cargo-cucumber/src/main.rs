mod new_project;

extern crate rustyline;
#[macro_use] extern crate clap;
#[macro_use] extern crate log;
extern crate env_logger;


use clap::{Arg, App, SubCommand};

fn main() {
    env_logger::init().expect("Logger should be creatable");

    let matches = App::new("Cucumber")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Cucumber(.rs) utility")
        .bin_name("cargo-cucumber")
        .subcommand(SubCommand::with_name("new")
            .about("Creates a new Cucumber project")
            .version(crate_version!())
            .author(crate_authors!())
            .arg(Arg::with_name("default")
                .short("d")
                .help("if used, creates the project using defaults (non-interactive mode)"))
            .arg(Arg::with_name("overwrite")
                .short("o")
                .help("if used, will overwrite preexisting Cucumber projects)")))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("new") {
        let overwrite = matches.is_present("overwrite");
        if matches.is_present("default") {
            new_project::new_default(overwrite);
        } else {
            new_project::new(overwrite);
        }
    } else {
        error!("Cucumber App is currently only usable with its subcommands! Type --help to see its proper usage.");
    }
}


