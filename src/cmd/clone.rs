use std::fs;

use anyhow::Result;
use clap::Parser;

use crate::{Config, Task};

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
        let exec = &config.exec;

        repo.absent()?;

        let temp = config.temp();
        fs::create_dir_all(&temp)?;

        // first clone without checkout
        exec.run(repo.cmd(
            Some(&temp),
            &[
                "-c",
                "core.sharedrepository=0600",
                "clone",
                "--no-checkout",
                format!("--separate-git-dir={}", repo.root.display()).as_str(),
                &self.url,
                &repo.folder,
            ],
        ))?;

        // change bare to false (there is a working directory)
        exec.run(repo.cmd(None, &["config", "core.bare", "false"]))?;
        // set the worktree for the yadm repo
        exec.run(repo.cmd(None, &["config", "core.worktree", work.to_str().unwrap()]))?;
        // by default, do not show untracked files and directories
        exec.run(repo.cmd(None, &["config", "status.showUntrackedFiles", "no"]))?;
        // possibly used later to ensure we're working on the correct repo
        exec.run(repo.cmd(None, &["config", "yadm.rs.managed", "true"]))?;

        fs::remove_dir_all(temp)?;

        // then reset the index as the --no-checkout flag makes the index empty
        exec.run(repo.cmd(None, &["reset", "--quiet", "--", ":/"]))?;

        // finally check out (unless instructed not to) all files that don't exist in work directory
        if !self.no_checkout {
            let deleted = exec.output(repo.cmd(Some(work), &["ls-files", "--deleted"]))?;
            for file in deleted {
                exec.run(repo.cmd(Some(work), &["checkout", "--", &format!(":/{}", file)]))?;
            }

            // TODO: handle submodules
            // git submodule update --init --recursive -- <path>

            let modified = exec.output(repo.cmd(Some(work), &["ls-files", "--modified"]))?;
            if !modified.is_empty() {
                println!("Local files with content that differs from the ones just cloned");
                println!("found in {:?}, they have been left unmodified", work);
            }
        }

        Ok(())
    }
}
