use crate::commands::{output, run_command};
use crate::error::Error;
use clap::ArgMatches;
use regex::Regex;

#[derive(Debug)]
pub enum DeleteMode {
    Local,
    Remote,
    Both,
}

pub use self::DeleteMode::*;

impl DeleteMode {
    pub fn new(opts: &ArgMatches) -> DeleteMode {
        if *opts.get_one("locals").expect("locals should be set") {
            Local
        } else if *opts.get_one("remotes").expect("remotes should be set") {
            Remote
        } else {
            Both
        }
    }

    pub fn warning_message(&self) -> String {
        let source = match *self {
            Local => "locally:",
            Remote => "remotely:",
            Both => "locally and remotely:",
        };
        format!("The following branches will be deleted {}", source)
    }
}

pub struct Options {
    pub remote: String,
    pub base_branch: String,
    pub squashes: bool,
    pub delete_unpushed_branches: bool,
    pub ignored_branches: Vec<String>,
    pub delete_mode: DeleteMode,
}

impl Options {
    pub fn new(opts: &ArgMatches) -> Options {
        let ignored: Vec<String> = opts
            .get_many("ignore")
            .unwrap_or_default()
            .map(|s: &String| s.to_string())
            .collect();

        Options {
            remote: opts
                .get_one::<String>("remote")
                .expect("Should be able to get remote")
                .into(),
            base_branch: opts
                .get_one::<String>("branch")
                .expect("Should be able to get base branch")
                .into(),
            ignored_branches: ignored,
            squashes: *opts.get_one("squashes").expect("squashes should be set"),
            delete_unpushed_branches: *opts
                .get_one("delete-unpushed-branches")
                .expect("delete-unpushed-branches should be set"),
            delete_mode: DeleteMode::new(opts),
        }
    }

    pub fn validate(&self) -> Result<(), Error> {
        self.validate_base_branch()?;
        self.validate_remote()?;
        Ok(())
    }

    fn validate_base_branch(&self) -> Result<(), Error> {
        let current_branch = output(&["git", "rev-parse", "--abbrev-ref", "HEAD"]);

        if current_branch != self.base_branch {
            return Err(Error::CurrentBranchInvalid);
        };

        Ok(())
    }

    fn validate_remote(&self) -> Result<(), Error> {
        let remote_rx = Regex::new(&self.remote).unwrap();
        let remotes = run_command(&["git", "remote"]);
        let remotes_output = std::str::from_utf8(&remotes.stdout).unwrap();

        let remote_result =
            remote_rx
                .captures_iter(remotes_output)
                .fold(String::new(), |mut acc, e| {
                    acc.push_str(&e[0]);
                    acc
                });

        if remote_result.is_empty() {
            return Err(Error::InvalidRemote);
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::{DeleteMode, Options};
    use crate::cli;

    // Helpers
    fn parse_args(args: Vec<&str>) -> clap::ArgMatches {
        cli::build_cli().get_matches_from(args)
    }

    // DeleteMode tests
    #[test]
    fn test_delete_mode_new() {
        let matches = parse_args(vec!["git-clean", "-l"]);

        match DeleteMode::new(&matches) {
            DeleteMode::Local => (),
            other => panic!("Expected a DeleteMode::Local, but found: {:?}", other),
        };

        let matches = parse_args(vec!["git-clean", "-r"]);

        if let DeleteMode::Local = DeleteMode::new(&matches) {
            panic!("Expected a DeleteMode::Remote, but found: DeleteMode::Local")
        };

        match DeleteMode::new(&matches) {
            DeleteMode::Remote => (),
            other => panic!("Expected a DeleteMode::Remote, but found: {:?}", other),
        };

        let matches = parse_args(vec!["git-clean"]);

        match DeleteMode::new(&matches) {
            DeleteMode::Both => (),
            other => panic!("Expected a DeleteMode::Both, but found: {:?}", other),
        };
    }

    #[test]
    fn test_delete_mode_warning_message() {
        assert_eq!(
            "The following branches will be deleted locally:",
            DeleteMode::Local.warning_message()
        );
        assert_eq!(
            "The following branches will be deleted remotely:",
            DeleteMode::Remote.warning_message()
        );
        assert_eq!(
            "The following branches will be deleted locally and remotely:",
            DeleteMode::Both.warning_message()
        );
    }

    // Options tests
    #[test]
    fn test_git_options_new() {
        let matches = parse_args(vec!["git-clean"]);
        let git_options = Options::new(&matches);

        assert_eq!("main".to_owned(), git_options.base_branch);
        assert_eq!("origin".to_owned(), git_options.remote);

        let matches = parse_args(vec!["git-clean", "-b", "stable"]);
        let git_options = Options::new(&matches);

        assert_eq!("stable".to_owned(), git_options.base_branch);
        assert_eq!("origin".to_owned(), git_options.remote);

        let matches = parse_args(vec!["git-clean", "-R", "upstream"]);
        let git_options = Options::new(&matches);

        assert_eq!("main".to_owned(), git_options.base_branch);
        assert_eq!("upstream".to_owned(), git_options.remote);
        assert!(!git_options.squashes);
        assert!(!git_options.delete_unpushed_branches);

        let matches = parse_args(vec![
            "git-clean",
            "-R",
            "upstream",
            "--squashes",
            "--delete-unpushed-branches",
        ]);
        let git_options = Options::new(&matches);

        assert!(git_options.squashes);
        assert!(git_options.delete_unpushed_branches);

        let matches = parse_args(vec![
            "git-clean",
            "-i",
            "branch1",
            "-i",
            "branch2",
            "-i",
            "branch3",
        ]);
        let git_options = Options::new(&matches);

        assert_eq!(
            git_options.ignored_branches,
            vec!["branch1", "branch2", "branch3"]
        );
    }
}
