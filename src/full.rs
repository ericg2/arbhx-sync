use crate::data_full::VfsFullSeekSync;
use crate::data_read_seek::VfsReadSeekSync;
use crate::read::VfsReaderSync;
use crate::write::VfsWriterSync;
use arbhx_core::blocking::{
    DataFullCompat, DataReadCompat, DataReadSeekCompat, DataWriteCompat, DataWriteSeekCompat,
    SizedQueryCompat, VfsFullCompat, VfsReaderCompat, VfsWriterCompat,
};
use arbhx_core::{FilterOptions, Metadata, VfsFull, VfsWriter};
use chrono::{DateTime, Local};
use std::path::Path;
use std::sync::Arc;
use tokio::runtime::Handle;

#[derive(Debug, Clone)]
pub struct VfsFullSync {
    read: VfsReaderSync,
    write: VfsWriterSync,
    full: Arc<dyn VfsFull>,
    rt: Handle,
}

impl VfsFullSync {
    pub(crate) fn new(rt: Handle, full: Arc<dyn VfsFull>) -> Self {
        Self {
            read: VfsReaderSync::new(rt.clone(), full.clone()),
            write: VfsWriterSync::new(rt.clone(), full.clone()),
            full,
            rt,
        }
    }
}

impl VfsReaderCompat for VfsFullSync {
    fn open_read_start(&self, item: &Path) -> std::io::Result<Box<dyn DataReadCompat>> {
        self.read.open_read_start(item)
    }

    fn open_read_random(
        &self,
        item: &Path,
    ) -> std::io::Result<Option<Box<dyn DataReadSeekCompat>>> {
        self.read.open_read_random(item)
    }

    fn get_metadata(&self, item: &Path) -> std::io::Result<Option<Metadata>> {
        self.read.get_metadata(item)
    }

    fn read_dir(
        &self,
        item: &Path,
        opts: Option<FilterOptions>,
        recursive: bool,
        include_root: bool,
    ) -> std::io::Result<Arc<dyn SizedQueryCompat>> {
        self.read.read_dir(item, opts, recursive, include_root)
    }
}

impl VfsWriterCompat for VfsFullSync {
    fn remove_dir(&self, dirname: &Path) -> std::io::Result<()> {
        self.write.remove_dir(dirname)
    }

    fn remove_file(&self, filename: &Path) -> std::io::Result<()> {
        self.write.remove_file(filename)
    }

    fn create_dir(&self, item: &Path) -> std::io::Result<()> {
        self.write.create_dir(item)
    }

    fn set_times(
        &self,
        item: &Path,
        mtime: DateTime<Local>,
        atime: DateTime<Local>,
    ) -> std::io::Result<()> {
        self.write.set_times(item, mtime, atime)
    }

    fn set_length(&self, item: &Path, size: u64) -> std::io::Result<()> {
        self.write.set_length(item, size)
    }

    fn move_to(&self, old: &Path, new: &Path) -> std::io::Result<()> {
        self.write.move_to(old, new)
    }

    fn copy_to(&self, old: &Path, new: &Path) -> std::io::Result<()> {
        self.write.copy_to(old, new)
    }

    fn open_write_append(
        &self,
        item: &Path,
        overwrite: bool,
    ) -> std::io::Result<Box<dyn DataWriteCompat>> {
        self.write.open_write_append(item, overwrite)
    }

    fn open_write_random(
        &self,
        item: &Path,
    ) -> std::io::Result<Option<Box<dyn DataWriteSeekCompat>>> {
        self.write.open_write_random(item)
    }
}

impl VfsFullCompat for VfsFullSync {
    fn open_full_random(&self, item: &Path) -> std::io::Result<Option<Box<dyn DataFullCompat>>> {
        match self.rt.block_on(self.full.open_full_random(item))? {
            Some(x) => Ok(Some(Box::new(VfsFullSeekSync::new(self.rt.clone(), x)))),
            None => Ok(None),
        }
    }
}
