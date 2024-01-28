use gtea_lib::config::Config;
use gtea_lib::exec;
use gtea_lib::prelude::*;
use gtea_lib::util::constants;
use gtea_lib::util::filepath;
use gtea_lib::worktree;

pub fn is_bare_root(dir: &str) -> bool {
    let mut checks: Vec<bool> = vec![];
    checks.push(filepath::exists(&format!("{}/worktrees", dir)));
    checks.push(filepath::exists(&format!("{}/branches", dir)));
    checks.push(filepath::exists(&format!("{}/HEAD", dir)));
    let result = checks
        .iter()
        .map(|v| {
            if !v {
                return Err(());
            }
            Ok(())
        })
        .collect::<std::result::Result<(), ()>>();
    match result {
        Ok(_) => true,
        Err(_) => false,
    }
}
pub fn is_worktree() -> bool {
    if !is_bare_root(&filepath::join(&*constants::CWD, "..")) {
        return false;
    }
    if !filepath::exists(&format!("{}/.git", &*constants::CWD)) {
        return false;
    }
    return true;
}

/// rebase root onto ahead
///
/// - *root*: root branch as fullpath
/// - *ahead*: branch with updated changes as fullpath
pub fn rebase(root: &str, ahead: &str) -> Result<()> {
    // stash local changes
    let local_changes = exec::git_local_changes(root)?;
    if local_changes {
        exec::git_stash_push(root)?;
    }
    // $ git rebase ahead
    let branch = exec::git_current_branch(ahead)?;
    debug!("changes from {branch} on {root}");
    exec::git_rebase(&branch, root)?;
    // apply stashed changes
    if local_changes {
        exec::git_stash_pop(root)?;
    }
    Ok(())
}
/// get parent branch name
///
/// - *child*: child branch name
pub fn get_parent_branch(child: &str) -> Result<String> {
    let config = Config::new()?;
    let parent = if config.nightly.enable && child != config.nightly.branch {
        config.nightly.branch
    } else {
        config.main.branch
    };
    Ok(parent)
}
pub fn get_branch_dir(branch: &str) -> Result<String> {
    let worktrees = worktree::get_worktree_list(&*constants::CWD)?;
    debug!("{:?}", worktrees);
    let worktrees: Vec<Option<String>> = worktrees.iter().map(|v| {
        if let Some(branch_bind) = &v.branch {
            if *branch_bind == branch {
                debug!("worktree found. {}", v.worktree);
                return Some(v.worktree.clone());
            }
        }
        None
    }).collect();
    let worktrees: Vec<String> = worktrees.into_iter().flatten().collect();
    debug!("found worktrees. {:?}", worktrees);
    match worktrees.get(0) {
        Some(v) => Ok(v.to_owned()),
        None => Err(make_err!(NotFound, "worktree for branch {branch} not found.")) 
    }
}
/// get fullpath to parent branch
///
/// - *child*: child branch name
pub fn get_parent_dir(child: &str) -> Result<String> {
    let parent = get_parent_branch(&child)?;
    get_branch_dir(&parent)
}
/// update child branch to changes from parent branch
///
/// get parent branch
/// $ git rebase parent
///
/// rebase(cwd, parent)
pub fn update() -> Result<()> {
    let branch = exec::git_current_branch(&*constants::CWD)?;
    let parent_dir = get_parent_dir(&branch)?;
    rebase(&*constants::CWD, &parent_dir)?;
    Ok(())
}
/// push local changes to parent branch.
///
/// work on parent branch
/// - $ git rebase child
/// (optional) push parent branch
pub fn push(upstream: bool) -> Result<()> {
    let branch = exec::git_current_branch(&*constants::CWD)?;
    let branch_dir = get_branch_dir(&branch)?;
    let parent = get_parent_branch(&branch)?;
    let parent_dir = get_parent_dir(&branch)?;
    rebase(&parent_dir, &branch_dir)?;
    if upstream {
        exec::git_push("origin", &parent)?;
    }
    Ok(())
}
pub fn create(name: &str) -> Result<()> {
    todo!()
}
pub fn remove(name: &str) -> Result<()> {
    todo!()
}
