use clap::{App, Arg};

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn build_cli() -> App<'static, 'static> {
    App::new("git-clean")
        .version(VERSION)
        .author("Matt Casper <matthewvcasper@gmail.com>")
        .about("Cleans stuff")
        .arg(
            Arg::with_name("locals")
                .short("l")
                .long("locals")
                .help("only delete local branches")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("remotes")
                .short("r")
                .long("remotes")
                .help("only delete remote branches")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("yes")
                .short("y")
                .long("yes")
                .help("skip the check for deleting branches")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("squashes")
                .short("s")
                .long("squashes")
                .help("check for squashes by finding branches incompatible with master")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("remote")
                .short("R")
                .long("remote")
                .help("changes the git remote used (default is origin)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("branch")
                .short("b")
                .long("branch")
                .help("changes the base for merged branches (default is master)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("ignore")
                .short("i")
                .long("ignore")
                .help("ignore given branches")
                .takes_value(true)
                .multiple(true),
        )
}
