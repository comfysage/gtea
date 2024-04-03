use clap::builder::styling;
use gtea::cli;
use gtea_lib::prelude::*;

use clap::{arg, Command};

fn get_commands() -> Command {
    let effects = (styling::Effects::BOLD | styling::Effects::UNDERLINE).clear();
    let styles = styling::Styles::styled()
        .header(styling::AnsiColor::White.on_default() | effects)
        .usage(styling::AnsiColor::White.on_default() | effects)
        .literal(styling::AnsiColor::BrightWhite.on_default() | effects)
        .placeholder(styling::AnsiColor::BrightWhite.on_default() | effects);

    Command::new("gtea")
        .about("a tiny git helper.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .styles(styles)
        .subcommand(Command::new("env").about("show environment script"))
        .subcommand(
            Command::new("config")
                .about("manage local configuration")
                .subcommand_required(true)
                .arg_required_else_help(true)
                .subcommand(Command::new("create").about("create gtea.toml")),
        )
        .subcommand(
            Command::new("workflow")
                .about("Manage git workflow")
                .visible_alias("wf")
                .subcommand_required(true)
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("update")
                        .about("update the current branch")
                        .arg_required_else_help(false),
                )
                .subcommand(
                    Command::new("push")
                        .about("push to parent branch")
                        .arg_required_else_help(false)
                        .arg(arg!(-p --push "push to remote")),
                )
                .subcommand(
                    Command::new("create")
                        .about("create child branch")
                        .arg_required_else_help(true)
                        .arg(arg!(<NAME> "name of branch")),
                )
                .subcommand(
                    Command::new("remove")
                        .about("remove a worktree")
                        .arg_required_else_help(true)
                        .arg(arg!(<NAME> "name of branch")),
                ),
        )
        .subcommand(Command::new("init").about("initialize git repo"))
        .subcommand(
            Command::new("clone")
                .about("clone a git repo")
                .arg_required_else_help(true)
                .arg(arg!(<URL> "git url")),
        )
        .subcommand(
            Command::new("create")
                .about("create a git remote")
                .arg_required_else_help(false)
                .arg(arg!([NAME] "name for the remote")),
        )
        .subcommand(
            Command::new("push")
                .about("push to git remote")
                .arg_required_else_help(false)
                .arg(arg!([BRANCH] "branch to push")),
        )
        .subcommand(
            Command::new("commit")
                .about("commit changes")
                .arg_required_else_help(true)
                .args([
                    arg!(<TYPE> "type of the commit"),
                    arg!(-s --scope <SCOPE> "scope of the commit"),
                    arg!(<MESSAGE> "commit message"),
                ]),
        )
        .subcommand(
            Command::new("changelog")
                .about("show changelog")
                .arg_required_else_help(true)
                .args([
                    arg!(-n --commits <NUM> "number of commits"),
                    arg!(-b --between <COMMIT> "changelog between commits").num_args(2),
                    arg!(-s --since <HASH> "changes since HASH"),
                ]),
        )
        .subcommand(
            Command::new("hash")
                .about("show hash")
                .arg_required_else_help(true)
                .args([arg!([REF] "git ref"), arg!(-b <BRANCH> "git branch")]),
        )
}

fn main() -> Result<()> {
    pretty_env_logger::init();

    let matches = get_commands().get_matches();

    match matches.subcommand() {
        Some(("env", _)) => {
            cli::env::env()?;
            Ok(())
        }
        Some(("config", sub_matches)) => {
            let subcommand = sub_matches.subcommand().unwrap_or(("init", sub_matches));
            match subcommand {
                ("create", _) => cli::config::create(),
                (&_, _) => Err(Error::Unexpected),
            }
        }
        Some(("wf", sub_matches)) => {
            let subcommand = sub_matches.subcommand().ok_or(make_err!())?;
            match subcommand {
                ("update", _) => {
                    cli::workflow::update()?;
                    Ok(())
                }
                ("push", _) => {
                    let upstream = sub_matches
                        .get_one::<u8>("")
                        .map(|v| *v)
                        .unwrap_or(0);
                    cli::workflow::push(upstream == 1)?;
                    Ok(())
                }
                ("create", sub_matches) => {
                    let name = sub_matches
                        .get_one::<String>("NAME")
                        .ok_or(make_err!(Missing, "no branch name specified."))?;
                    cli::workflow::create(&name)?;
                    Ok(())
                }
                ("remove", sub_matches) => {
                    let name = sub_matches
                        .get_one::<String>("NAME")
                        .ok_or(make_err!(Missing, "no branch name specified."))?;
                    cli::workflow::remove(&name)?;
                    Ok(())
                }
                (&_, _) => Err(Error::Unexpected),
            }
        }
        Some(("init", _)) => {
            cli::repo::init()?;
            Ok(())
        }
        Some(("clone", sub_matches)) => {
            let url = sub_matches
                .get_one::<String>("URL")
                .ok_or(make_err!(Missing, "no git url specified."))?;
            cli::repo::clone(url)?;
            Ok(())
        }
        Some(("create", sub_matches)) => {
            let name = sub_matches
                .get_one::<String>("NAME")
                .ok_or(make_err!(Missing, "no remote name specified."))?;
            cli::repo::create(Some(name))?;
            Ok(())
        }
        Some(("push", sub_matches)) => {
            let branch = sub_matches
                .get_one::<String>("BRANCH");
            cli::repo::push(branch.map(|x| x.as_str()))?;
            Ok(())
        }
        Some(("commit", sub_matches)) => {
            let t = sub_matches
                .get_one::<String>("TYPE")
                .ok_or(make_err!(Missing, "no type specified."))?;
            let scope = sub_matches.get_one::<String>("scope");
            let msg = sub_matches
                .get_one::<String>("MESSAGE")
                .ok_or(make_err!(Missing, "no message specified."))?;
            cli::git::commit(&t, scope, &msg)?;
            Ok(())
        }
        Some(("changelog", sub_matches)) => {
            let n = sub_matches.get_one::<String>("commits");
            let commits = sub_matches.get_many::<String>("between");
            let since = sub_matches.get_one::<String>("since");
            if let Some(n_bind) = n {
                cli::git::changelog_for_n(&n_bind)?;
            } else if let Some(commits_bind) = commits {
                if commits_bind.len() != 2 {
                    return Err(make_err!(Missing, "unexpected amount of arguments."));
                }
                let c: Vec<String> = commits_bind.map(|hash| hash.to_string()).collect();
                cli::git::changelog_between(&c[0], &c[1])?;
            } else if let Some(since_bind) = since {
                cli::git::changelog_since(since_bind)?;
            } else {
                return Err(make_err!(Missing, "argument missing."));
            }
            Ok(())
        }
        Some(("hash", sub_matches)) => {
            let gref = sub_matches.get_one::<String>("REF");
            let branch = sub_matches.get_one::<String>("BRANCH");
            if let Some(gref_bind) = gref {
                cli::git::get_ref_hash(gref_bind)?;
            } else if let Some(branch_bind) = branch {
                cli::git::get_branch_hash(branch_bind)?;
            } else {
                return Err(make_err!(Missing, "git ref missing."));
            }
            Ok(())
        }
        // if all subcommands are defined above, anything else is unreachable!()
        _ => Err(make_err!(Missing, "missing command. run saku --help.")),
    }
}
