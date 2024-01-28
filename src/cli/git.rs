use gtea_lib::{prelude::*, exec};

pub fn commit(t: &str, scope: Option<&String>, msg: &str) -> Result<()> {
    let message: String;
    if let Some(scope_bind) = scope {
        message = format!("{t}({scope_bind}): {msg}");
    } else {
        message = format!("{t}: {msg}");
    }
    debug!("message = {message}");
    exec::git_commit(&message)?;
    Ok(())
}
pub fn changelog_for_n(n: &str) -> Result<()> {
    debug!("changelog between 'HEAD..HEAD~{n}'");
    exec::git_changelog(format!("HEAD~{n}..HEAD").as_str())?;
    Ok(())
}
pub fn changelog_between(start: &str, end: &str) -> Result<()> {
    debug!("changelog between '{start}..{end}'");
    exec::git_changelog(format!("{start}..{end}").as_str())?;
    Ok(())
}
pub fn changelog_since(hash: &str) -> Result<()> {
    debug!("changelog between '{hash}..@@{{0}}'");
    exec::git_changelog(format!("{hash}..@@{{0}}").as_str())?;
    Ok(())
}

pub fn get_branch_hash(branch: &str) -> Result<()> {
    exec::git_show_ref(branch)?;
    Ok(())
}
pub fn get_ref_hash(gref: &str) -> Result<()> {
    exec::git_rev_parse(gref)?;
    Ok(())
}
