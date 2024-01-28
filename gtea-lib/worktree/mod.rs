use crate::prelude::*;

use crate::exec;

pub mod worktree;
use self::worktree::Worktree;

pub fn get_worktree_list(pwd: &str) -> Result<Vec<Worktree>> {
    let output = exec::git_worktree_list(pwd)?;
    Worktree::from_string(&output)
}
