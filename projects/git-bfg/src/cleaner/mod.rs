mod blob_item;

use std::cmp::Ordering;
use std::env::current_dir;
use std::ffi::OsString;
use std::fmt::{Debug, Display, Formatter, Write};
use std::fs::read_dir;
use std::path::{Path, PathBuf};
use byte_unit::Byte;
use git2::{Blob, ObjectType, Oid, Repository};
use sorted_vec::{ReverseSortedVec};
use crate::Result;

pub struct Cleaner {
    repository: Repository,
    trees: Vec<Oid>,
    blobs: Vec<Oid>,
}

impl Cleaner {
    pub fn new(root: &Path) -> Result<Self> {
        Ok(Self {
            repository: Repository::open(root)?,
            trees: vec![],
            blobs: vec![],
        })
    }
    pub fn collect_info(&mut self) -> Result<()> {
        let db = self.repository.odb()?;
        db.foreach(|c| {
            let o = match db.read(c.to_owned()) {
                Ok(o) => { o }
                Err(_) => { return true; }
            };
            match o.kind() {
                ObjectType::Any => {}
                ObjectType::Commit => {}
                ObjectType::Tree => {
                    self.trees.push(c.to_owned())
                }
                ObjectType::Blob => {
                    self.blobs.push(c.to_owned())
                }
                ObjectType::Tag => {}
            }
            true
        })?;
        Ok(())
    }
    pub fn largest_objects(&self, show: usize) -> Vec<BlobItem>  {
        println!("Find {} files and {} dir, here's {} largest objects:", self.blobs.len(), self.trees.len(), show);
        let mut sv = ReverseSortedVec::new();
        for i in &self.blobs {
            let blob = match self.repository.find_blob(i.to_owned()) {
                Ok(o) => {o}
                Err(_) => {
                    println!("{} had broken", i);
                    continue
                }
            };
            let item = BlobItem {
                id: i.to_owned(),
                format: BlobFormat::from_blob(&blob),
                size: blob.size(),
            };
            sv.insert(item);
        }
        for (index, item) in sv.iter().take(show).enumerate() {
            println!("{:width$} | {}", index + 1, item, width = show.log10() as usize)
        }
        sv.into_vec()
    }
}

#[derive(Debug)]
pub struct BlobItem {
    id: Oid,
    size: usize,
    format: BlobFormat,
}

#[derive(Debug)]
pub enum BlobFormat {
    Binary,
    Text,
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