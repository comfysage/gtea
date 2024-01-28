use gtea_lib::prelude::*;
use gtea_lib::exec;
use gtea_lib::util::constants;
use gtea_lib::util::filepath;

pub fn init() -> Result<()> {
    exec::git_init()?;
    Ok(())
}
pub fn clone(url: &str) -> Result<()> {
    exec::git_clone(&format!("{url} -- --filter=blob:none"))?;
    Ok(())
}
pub fn create(name: Option<&str>) -> Result<()> {
    let name: String = match name {
        Some(v) => v.to_string(),
        None => filepath::base_name(&constants::CWD)?,
    };
    exec::git_create(&name)?;
    Ok(())
}
pub fn push(branch: Option<&str>) -> Result<()> {
    if let Some(branch) = branch {
        exec::git_push("origin", branch)?;
    } else {
        let branch = exec::git_current_branch(&*constants::CWD)?;
        exec::git_push("origin", &branch)?;
    }
    Ok(())
}
