use std::env::current_dir;
use std::ffi::OsString;
use std::fs::read_dir;
use std::path::{Path, PathBuf};
use git2::Repository;
use crate::Result;

pub struct Cleaner {}


#[test]
fn test() -> Result<()> {
    let root = get_project_root()?;
    let repo = Repository::open(root)?;

    for i in repo.revwalk()? {
        println!("{:?}", i?);
    }

    Ok(())
}


pub fn get_project_root() -> std::io::Result<PathBuf> {
    let path = current_dir()?;
    let mut path_ancestors = path.as_path().ancestors();

    while let Some(p) = path_ancestors.next() {
        let has_cargo = read_dir(p)?
            .into_iter()
            .any(|p| p.unwrap().file_name() == OsString::from(".git"));
        if has_cargo {
            return Ok(PathBuf::from(p));
        }
    }
    Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Ran out of places to find Cargo.toml"))
}