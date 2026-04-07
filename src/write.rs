use crate::data_read_seek::VfsReadSeekSync;
use crate::data_write::VfsWriteSync;
use crate::data_write_seek::VfsWriteSeekSync;
use arbhx_core::blocking::{DataWriteCompat, DataWriteSeekCompat, VfsWriterCompat};
use arbhx_core::{VfsReader, VfsWriter};
use chrono::{DateTime, Local};
use std::path::Path;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::runtime::Handle;

#[derive(Debug, Clone)]
pub struct VfsWriterSync {
    write: Arc<dyn VfsWriter>,
    rt: Handle,
}

impl VfsWriterSync {
    pub(crate) fn new(rt: Handle, write: Arc<dyn VfsWriter>) -> Self {
        Self { rt, write }
    }
}

impl VfsWriterCompat for VfsWriterSync {
    fn remove_dir(&self, dirname: &Path) -> std::io::Result<()> {
        self.rt.block_on(self.write.remove_dir(dirname))
    }

    fn remove_file(&self, filename: &Path) -> std::io::Result<()> {
        self.rt.block_on(self.write.remove_file(filename))
    }

    fn create_dir(&self, item: &Path) -> std::io::Result<()> {
        self.rt.block_on(self.write.create_dir(item))
    }

    fn set_times(
        &self,
        item: &Path,
        mtime: DateTime<Local>,
        atime: DateTime<Local>,
    ) -> std::io::Result<()> {
        self.rt.block_on(self.write.set_times(item, mtime, atime))
    }

    fn set_length(&self, item: &Path, size: u64) -> std::io::Result<()> {
        self.rt.block_on(self.write.set_length(item, size))
    }

    fn move_to(&self, old: &Path, new: &Path) -> std::io::Result<()> {
        self.rt.block_on(self.write.move_to(old, new))
    }

    fn copy_to(&self, old: &Path, new: &Path) -> std::io::Result<()> {
        self.rt.block_on(self.write.copy_to(old, new))
    }

    fn open_write_append(
        &self,
        item: &Path,
        overwrite: bool,
    ) -> std::io::Result<Box<dyn DataWriteCompat>> {
        let st = self
            .rt
            .block_on(self.write.open_write_append(item, overwrite))?;
        let ret = Box::new(VfsWriteSync::new(self.rt.clone(), st));
        Ok(ret)
    }

    fn open_write_random(
        &self,
        item: &Path,
    ) -> std::io::Result<Option<Box<dyn DataWriteSeekCompat>>> {
        match self.rt.block_on(self.write.open_write_random(item))? {
            Some(x) => Ok(Some(Box::new(VfsWriteSeekSync::new(self.rt.clone(), x)))),
            None => Ok(None),
        }
    }
}
