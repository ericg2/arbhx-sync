use crate::data_full::VfsFullSeekSync;
use crate::data_read::VfsReadSync;
use crate::data_read_seek::VfsReadSeekSync;
use crate::data_write::VfsWriteSync;
use crate::data_write_seek::VfsWriteSeekSync;
use crate::query::QueryCompat;
use arbhx_core::blocking::{
    DataFullCompat, DataReadCompat, DataReadSeekCompat, DataWriteCompat, DataWriteSeekCompat,
    SizedQueryCompat, VfsBackendCompat, VfsFullCompat, VfsReaderCompat, VfsSeekWriterCompat,
    VfsWriterCompat,
};
use arbhx_core::{
    DataUsage, FilterOptions, Metadata, VfsBackend, VfsFull, VfsReader, VfsSeekWriter, VfsWriter,
};
use chrono::{DateTime, Local};
use std::io;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::runtime::Handle;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct VfsCompat {
    be: Arc<dyn VfsBackend>,
    reader: Option<Arc<dyn VfsReader>>,
    writer: Option<Arc<dyn VfsWriter>>,
    writer_seek: Option<Arc<dyn VfsSeekWriter>>,
    full: Option<Arc<dyn VfsFull>>,
    rt: Handle,
}

impl VfsCompat {
    fn reader(&self) -> io::Result<&Arc<dyn VfsReader>> {
        self.reader.as_ref().ok_or(ErrorKind::Unsupported.into())
    }
    fn writer(&self) -> io::Result<&Arc<dyn VfsWriter>> {
        self.writer.as_ref().ok_or(ErrorKind::Unsupported.into())
    }
    fn full(&self) -> io::Result<&Arc<dyn VfsFull>> {
        self.full.as_ref().ok_or(ErrorKind::Unsupported.into())
    }
    fn writer_seek(&self) -> io::Result<&Arc<dyn VfsSeekWriter>> {
        self.writer_seek
            .as_ref()
            .ok_or(ErrorKind::Unsupported.into())
    }
    pub fn new(rt: Handle, be: Arc<dyn VfsBackend>) -> Self {
        Self {
            reader: be.clone().reader(),
            writer: be.clone().writer(),
            writer_seek: be.clone().writer_seek(),
            full: be.clone().full(),
            rt,
            be,
        }
    }
}

impl VfsReaderCompat for VfsCompat {
    fn open_read_start(&self, item: &Path) -> io::Result<Box<dyn DataReadCompat>> {
        let handle = self.rt.block_on(self.reader()?.open_read_start(item))?;
        let ret = VfsReadSync::new(self.rt.clone(), handle);
        Ok(Box::new(ret))
    }

    fn open_read_seek(&self, item: &Path) -> io::Result<Box<dyn DataReadSeekCompat>> {
        let handle = self.rt.block_on(self.reader()?.open_read_seek(item))?;
        let ret = VfsReadSeekSync::new(self.rt.clone(), handle);
        Ok(Box::new(ret))
    }

    fn get_metadata(&self, item: &Path) -> io::Result<Option<Metadata>> {
        self.rt.block_on(self.reader()?.get_metadata(item))
    }

    fn list(
        &self,
        item: &Path,
        opts: Option<FilterOptions>,
        recursive: bool,
        include_root: bool,
    ) -> io::Result<Arc<dyn SizedQueryCompat>> {
        let handle = self
            .rt
            .block_on(self.reader()?.list(item, opts, recursive, include_root))?;
        let ret = QueryCompat::new(self.rt.clone(), handle);
        Ok(Arc::new(ret))
    }
}

impl VfsWriterCompat for VfsCompat {
    fn remove_dir(&self, path: &Path) -> io::Result<()> {
        self.rt.block_on(self.writer()?.remove_dir(path))
    }

    fn remove_file(&self, path: &Path) -> io::Result<()> {
        self.rt.block_on(self.writer()?.remove_file(path))
    }

    fn create_dir(&self, path: &Path) -> io::Result<()> {
        self.rt.block_on(self.writer()?.create_dir(path))
    }

    fn set_times(
        &self,
        path: &Path,
        mtime: DateTime<Local>,
        atime: DateTime<Local>,
    ) -> io::Result<()> {
        self.rt
            .block_on(self.writer()?.set_times(path, mtime, atime))
    }

    fn set_length(&self, path: &Path, size: u64) -> io::Result<()> {
        self.rt.block_on(self.writer()?.set_length(path, size))
    }

    fn move_to(&self, old: &Path, new: &Path) -> io::Result<()> {
        self.rt.block_on(self.writer()?.move_to(old, new))
    }

    fn copy_to(&self, old: &Path, new: &Path) -> io::Result<()> {
        self.rt.block_on(self.writer()?.copy_to(old, new))
    }

    fn open_write(&self, path: &Path, truncate: bool) -> io::Result<Box<dyn DataWriteCompat>> {
        let handle = self
            .rt
            .block_on(self.writer()?.open_write(path, truncate))?;
        let ret = VfsWriteSync::new(self.rt.clone(), handle);
        Ok(Box::new(ret))
    }
}

impl VfsSeekWriterCompat for VfsCompat {
    fn open_write_seek(&self, path: &Path) -> io::Result<Box<dyn DataWriteSeekCompat>> {
        let handle = self
            .rt
            .block_on(self.writer_seek()?.open_write_seek(path))?;
        let ret = VfsWriteSeekSync::new(self.rt.clone(), handle);
        Ok(Box::new(ret))
    }
}

impl VfsFullCompat for VfsCompat {
    fn open_full_seek(&self, path: &Path) -> io::Result<Box<dyn DataFullCompat>> {
        let handle = self.rt.block_on(self.full()?.open_full_seek(path))?;
        let ret = VfsFullSeekSync::new(self.rt.clone(), handle);
        Ok(Box::new(ret))
    }
}

impl VfsBackendCompat for VfsCompat {
    fn id(&self) -> Uuid {
        self.be.id()
    }

    fn realpath(&self, item: &Path) -> PathBuf {
        self.be.realpath(item)
    }

    fn reader(self: Arc<Self>) -> Option<Arc<dyn VfsReaderCompat>> {
        if self.reader.is_some() {
            Some(self.clone())
        } else {
            None
        }
    }

    fn writer(self: Arc<Self>) -> Option<Arc<dyn VfsWriterCompat>> {
        if self.writer.is_some() {
            Some(self.clone())
        } else {
            None
        }
    }

    fn writer_seek(self: Arc<Self>) -> Option<Arc<dyn VfsSeekWriterCompat>> {
        if self.writer_seek.is_some() {
            Some(self.clone())
        } else {
            None
        }
    }

    fn full(self: Arc<Self>) -> Option<Arc<dyn VfsFullCompat>> {
        if self.full.is_some() {
            Some(self.clone())
        } else {
            None
        }
    }

    fn get_usage(&self) -> io::Result<Option<DataUsage>> {
        self.rt.block_on(self.be.get_usage())
    }
}
