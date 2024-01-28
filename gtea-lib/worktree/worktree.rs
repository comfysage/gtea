use nom::{
    branch::permutation,
    bytes::complete::{tag, take_until},
    character::complete::space0,
    IResult,
};

use crate::prelude::*;

#[derive(Debug, PartialEq)]
#[derive(Clone)]
pub struct Worktree {
    pub worktree: String,
    pub bare: bool,
    pub head: Option<String>,
    pub branch: Option<String>,
}

impl Worktree {
    // parsing mandatory fields in any order
    pub fn parse(input: &str) -> IResult<&str, Worktree> {
        permutation((
            Worktree::parse_worktree,
            Worktree::parse_info,
        ))(input)
        .map(|(next_input, (worktree, info))| {
            (
                next_input,
                match info {
                        Some(v) => Worktree {
                            worktree,
                            bare: false,
                            head: Some(v.0),
                            branch: Some(v.1),
                        },
                        None => Worktree {
                            worktree,
                            bare: true,
                            head: None,
                            branch: None,
                        },
                    }
            )
        })
    }

    pub fn from_string(input: &str) -> Result<Vec<Worktree>> {
        let input: String = input.to_string().trim_start_matches('\n').trim_end_matches('\n').to_string();
        let result: Vec<Result<Worktree>> = input.split("\n\n").map(|v| format!("{v}\n")).map(|field| {
            let object = Self::parse(&field);
            match object {
                Ok(object) => Ok(object.1.clone()),
                Err(err) => {
                    let str = err.to_string();
                    Err(make_err!(Parse, "{str}"))
                },
            }
        }).collect();
        let result = result.into_iter().collect();
        result
    }

    fn parse_field<'a>(input: &'a str, field: &str) -> IResult<&'a str, String> {
        let (input, _) = tag(field)(input)?;
        // let (input, _) = space0(input)?;
        // let (input, _) = tag("=")(input)?;
        let (input, _) = space0(input)?;
        let (input, value) = take_until("\n")(input)?;
        let (input, _) = tag("\n")(input)?;
        Ok((input, value.to_string().trim_matches('"').to_string()))
    }

    fn parse_worktree(input: &str) -> IResult<&str, String> {
        Self::parse_field(input, "worktree")
    }

    fn parse_info(input: &str) -> IResult<&str, Option<(String, String)>> {
        let result: IResult<&str, &str> = tag("bare")(input);
        match result {
            Err(_) => permutation((
                Worktree::parse_head,
                Worktree::parse_branch,
            ))(input).map(|(next_input, info)| {
                    (
                        next_input,
                        Some(info),
                    )
                }),
            Ok((input, _)) => {
                Ok((input, None))
            },
        }
    }

    fn parse_head(input: &str) -> IResult<&str, String> {
        Self::parse_field(input, "HEAD")
    }

    fn parse_branch(input: &str) -> IResult<&str, String> {
        let (input, _) = tag("branch")(input)?;
        // let (input, _) = space0(input)?;
        // let (input, _) = tag("=")(input)?;
        let (input, _) = space0(input)?;
        let (input, _) = tag("refs/heads/")(input)?;
        let (input, value) = take_until("\n")(input)?;
        let (input, _) = tag("\n")(input)?;
        Ok((input, value.to_string().trim_matches('"').to_string()))
    }

    // pub fn to_string(&self) -> String {
    //     format!("worktree {}\nHEAD {}\nbranch {}", self.worktree, self.branch, self.head)
    // }
}

#[cfg(test)]
mod tests {
    #[test]
    fn worktree() {
        let input = "worktree /path/to/repo.git/foo\n";
        let expected = "/path/to/repo.git/foo";
        let (input, worktree) = super::Worktree::parse_worktree(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(worktree, expected);
    }

    #[test]
    fn head() {
        let input = "HEAD 1111111111111111111111111111111111111111\n";
        let expected = "1111111111111111111111111111111111111111";
        let (input, head) = super::Worktree::parse_head(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(head, expected);
    }

    #[test]
    fn branch() {
        let input = "branch refs/heads/foo/bar\n";
        let expected = "foo/bar";
        let (input, branch) = super::Worktree::parse_branch(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(branch, expected);
    }

    #[test]
    fn worktree_object() {
        let input = "worktree /path/to/repo.git/foo\nHEAD 1111111111111111111111111111111111111111\nbranch refs/heads/foo/bar\n";
        let expected = super::Worktree {
            worktree: "/path/to/repo.git/foo".to_string(),
            bare: false,
            head: Some("1111111111111111111111111111111111111111".to_string()),
            branch: Some("foo/bar".to_string()),
        };
        let (input, worktree_object) = super::Worktree::parse(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(worktree_object, expected);
    }

    #[test]
    fn worktree_output() {
        let input = "\nworktree /path/to/repo.git\nbare\n\nworktree /path/to/repo.git/foo\nHEAD 1111111111111111111111111111111111111111\nbranch refs/heads/foo/bar\n\nworktree /path/to/repo.git/foo\nHEAD 1111111111111111111111111111111111111111\nbranch refs/heads/foo/bar\n\n";
        let bare_value = super::Worktree {
            worktree: "/path/to/repo.git".to_string(),
            bare: true,
            head: None,
            branch: None,
        };
        let value = super::Worktree {
            worktree: "/path/to/repo.git/foo".to_string(),
            bare: false,
            head: Some("1111111111111111111111111111111111111111".to_string()),
            branch: Some("foo/bar".to_string()),
        };
        let expected = vec![bare_value, value.clone(), value.clone()];
        let worktree_object = super::Worktree::from_string(input).unwrap();
        assert_eq!(worktree_object, expected);
    }
}
