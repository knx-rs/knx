use std::path::Path;

use snafu::{ResultExt, Snafu};

mod xml;
mod zip;

pub(crate) use zip::*;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("failed to read archive"))]
    ReadArchive { source: zip::Error },
}

pub struct Project {
    project_info: Option<ProjectInfo>,
}

impl Project {
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        let mut iter = ZipIter::new(path).context(ReadArchiveSnafu)?;

        while let Some(file) = iter.next() {
            match file.enclosed_name() {
                Some(name) => println!("{}", name.display()),
                None => continue,
            }
        }

        Ok(Self { project_info: None })
    }
}

pub struct ProjectInfo {}
