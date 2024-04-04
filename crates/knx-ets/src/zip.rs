use std::{fs::File, path::Path};

use snafu::{ResultExt, Snafu};
use zip_next::{read::ZipFile, ZipArchive};

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("failed to open file"))]
    OpenFile { source: std::io::Error },

    #[snafu(display("failed to read zip archive"))]
    ReadArchive { source: zip_next::result::ZipError },

    #[snafu(display("failed to retrieve inner file"))]
    GetInnerFile { source: zip_next::result::ZipError },
}

pub struct ZipIter {
    archive: ZipArchive<File>,
    index: usize,
}

impl ZipIter {
    pub(crate) fn new(path: impl AsRef<Path>) -> Result<Self> {
        let file = File::open(path).context(OpenFileSnafu)?;
        let archive = ZipArchive::new(file).context(ReadArchiveSnafu)?;

        Ok(Self { archive, index: 0 })
    }

    pub(crate) fn next<'a>(&'a mut self) -> Option<ZipFile<'a>> {
        if self.index == self.archive.len() {
            return None;
        }

        let item = self.archive.by_index(self.index).ok();
        self.index += 1;

        item
    }
}
