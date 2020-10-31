use std::cmp::Ordering;
use std::env::current_dir;
use std::ffi::OsString;
use std::fs::read_dir;
use std::path::{Path, PathBuf};
use git2::{Blob, ObjectType, Oid, Repository};
use sorted_vec::{ReverseSortedVec, SortedVec};
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
    pub fn collect(&mut self) -> Result<()> {
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
}
#[derive(Debug)]
pub struct BlobItem {
    id: Oid,
    size: usize,
    format: BlobFormat
}
#[derive(Debug)]
pub enum BlobFormat {
    Binary,
    Text
}

impl BlobFormat {
    pub fn from_blob(blob: &Blob) -> Self {
        match blob.is_binary() {
            true => {Self::Binary}
            false => {Self::Text}
        }
    }
}

impl Eq for BlobItem {}

impl PartialEq<Self> for BlobItem {
    fn eq(&self, other: &Self) -> bool {
        self.size.eq(&other.size)
    }
}

impl PartialOrd<Self> for BlobItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.size.partial_cmp(&other.size)
    }
}

impl Ord for BlobItem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.size.cmp(&other.size)
    }
}


#[test]
fn test() -> Result<()> {
    let root = get_project_root()?;
    let mut cleaner = Cleaner::new(&root)?;
    cleaner.collect()?;
    println!("Find {} files and {} dir", cleaner.blobs.len(), cleaner.trees.len());
    let mut sv = ReverseSortedVec::new();
    for i in cleaner.blobs {
        let r = cleaner.repository.find_blob(i)?;
        let item = BlobItem{
            id: i,
            format: BlobFormat::from_blob(&r),
            size: r.size()
        };
        sv.insert(item);

    }
    println!("{:#?}", sv);
    // for i in cleaner.trees {
    //     let r = cleaner.repository.find_tree(i)?;
    //     for e in r.iter() {
    //         println!("{:?}", e.name())
    //     }
    // }
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