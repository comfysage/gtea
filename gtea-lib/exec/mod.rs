use crate::prelude::*;
use crate::util::constants;

use self::run::{run_one, run_one_return_code, run_with_output};

mod run;

pub fn git_push(remote: &str, branch: &str) -> Result<()> {
    run_one(format!("git push -u {remote} heads/{branch}"), &constants::CWD)
}

pub fn git_commit(msg: &str) -> Result<()> {
    run_one(format!("git commit -m {msg}"), &constants::CWD)
}

pub fn git_changelog(arg: &str) -> Result<()> {
    run_one(format!("git -c pager.show=false show --format=\" - %C(yellow)%h%C(reset) %<(80,trunc)%s\" -q {arg}"), &constants::CWD)
}

pub fn git_show_ref(arg: &str) -> Result<()> {
    git_rev_parse(format!("$(git show-ref -s {arg})").as_str())
}
pub fn git_rev_parse(arg: &str) -> Result<()> {
    run_one(format!("git rev-parse --short {arg}"), &constants::CWD)
}

pub fn git_init() -> Result<()> {
    run_one(format!("git init"), &constants::CWD)
}
pub fn git_clone(arg: &str) -> Result<()> {
    run_one(format!("gh repo clone {arg}"), &constants::CWD)
}
pub fn git_create(name: &str) -> Result<()> {
    run_one(format!("gh repo create {name} --public --source=. --remote=origin --push"), &constants::CWD)
}

pub fn git_current_branch(pwd: &str) -> Result<String> {
    run_with_output(format!("git branch --show-current"), pwd).map(|x| x.trim_end_matches("\n").to_string())
}
pub fn git_worktree_list(pwd: &str) -> Result<String> {
    run_with_output(format!("git worktree list --porcelain"), pwd)
}

pub fn git_local_changes(pwd: &str) -> Result<bool> {
    let result = run_one_return_code(format!("test -z \"$(git diff)\" || exit 1"), pwd)?;
    Ok(!result)
}
pub fn git_stash_push(pwd: &str) -> Result<()> {
    run_one(format!("git stash -a -u"), pwd)
}
pub fn git_stash_pop(pwd: &str) -> Result<()> {
    run_one(format!("git stash pop -q"), pwd)
}
pub fn git_rebase(branch: &str, pwd: &str) -> Result<()> {
    run_one(format!("git rebase heads/{branch}"), pwd)
}
