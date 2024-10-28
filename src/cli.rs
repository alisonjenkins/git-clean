use clap::{Arg, Command};

const DEFAULT_BRANCH: &str = "main";
const DEFAULT_REMOTE: &str = "origin";
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn build_cli() -> Command {
    Command::new("git-clean")
        .version(VERSION)
        .about("A tool for cleaning old git branches.")
        .arg(
            Arg::new("locals")
                .short('l')
                .long("locals")
                .action(clap::ArgAction::SetTrue)
                .help("Only delete local branches"),
        )
        .arg(
            Arg::new("remotes")
                .short('r')
                .long("remotes")
                .action(clap::ArgAction::SetTrue)
                .help("Only delete remote branches"),
        )
        .arg(
            Arg::new("yes")
                .short('y')
                .long("yes")
                .help("Skip the check for deleting branches"),
        )
        .arg(
            Arg::new("squashes")
                .short('s')
                .long("squashes")
                .action(clap::ArgAction::SetTrue)
                .help("Check for squashes by finding branches incompatible with main"),
        )
        .arg(
            Arg::new("delete-unpushed-branches")
                .short('d')
                .long("delete-unpushed-branches")
                .help("Delete any local branch that is not present on the remote. Use this to speed up the checks if such branches should always be considered as merged"),
        )
        .arg(
            Arg::new("remote")
                .short('R')
                .long("remote")
                .default_value(DEFAULT_REMOTE)
                .help("Changes the git remote used (default is origin)"),
        )
        .arg(
            Arg::new("branch")
                .short('b')
                .long("branch")
                .default_value(DEFAULT_BRANCH)
                .help("Changes the base for merged branches (default is main)"),
        )
        .arg(
            Arg::new("ignore")
                .short('i')
                .long("ignore")
                .help("Ignore given branch (repeat option for multiple branches)"),
        )
}
