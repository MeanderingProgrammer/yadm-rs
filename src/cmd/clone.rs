use std::fs;

use anyhow::Result;
use clap::Parser;

use crate::{Config, Exec, Task, cmd::Bootstrap};

#[derive(Debug, Parser)]
/// Clone an existing repository
pub struct Clone {
    /// Repository to clone
    url: String,

    /// Skip checking out files after cloning
    #[arg(short, long)]
    no_checkout: bool,

    /// Execute the bootstrap script
    #[arg(short, long)]
    bootstrap: bool,
}

impl Task for Clone {
    fn run(&self, config: &Config) -> Result<()> {
        let work = &config.work;
        let repo = &config.repo;

        repo.absent()?;

        let temp = config.temp();
        fs::create_dir_all(&temp)?;

        // first clone without checkout
        let mut clone = repo.cmd(&[
            "-c",
            "core.sharedrepository=0600",
            "clone",
            "--no-checkout",
            format!("--separate-git-dir={}", repo.root.display()).as_str(),
            &self.url,
            &repo.folder,
        ]);
        Exec::run(clone.current_dir(&temp))?;

        let configs = [
            // change bare to false (there is a working directory)
            ("core.bare", "false"),
            // set the worktree for the yadm repo
            ("core.worktree", &work.to_string_lossy()),
            // by default, do not show untracked files and directories
            ("status.showUntrackedFiles", "no"),
            // possibly used later to ensure we're working on the correct repo
            ("yadm.rs.managed", "true"),
        ];
        for (key, value) in configs {
            Exec::run(&mut repo.cmd(&["config", key, value]))?;
        }

        fs::remove_dir_all(temp)?;

        // then reset the index as the --no-checkout flag makes the index empty
        Exec::run(&mut repo.cmd(&["reset", "--quiet", "--", ":/"]))?;

        // finally check out (unless instructed not to) all files that don't exist in work directory
        if !self.no_checkout {
            let deleted = Exec::output(repo.cmd(&["ls-files", "--deleted"]).current_dir(work))?;
            for file in deleted {
                let mut checkout = repo.cmd(&["checkout", "--", &format!(":/{}", file)]);
                Exec::run(checkout.current_dir(work))?;
            }

            // TODO: handle submodules
            // git submodule update --init --recursive -- <path>

            let modified = Exec::output(repo.cmd(&["ls-files", "--modified"]).current_dir(work))?;
            if !modified.is_empty() {
                println!("local files with content that differs from the ones just cloned");
                println!("found in {:?}, they have been left unmodified", work);
            }
        }

        // execute the bootstrap script
        if self.bootstrap {
            Bootstrap::default().run(config)?;
        }

        Ok(())
    }
}
