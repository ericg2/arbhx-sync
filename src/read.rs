use crate::data_read::VfsReadSync;
use crate::data_read_seek::VfsReadSeekSync;
use crate::query::QueryCompat;
use arbhx_core::blocking::{DataReadCompat, DataReadSeekCompat, SizedQueryCompat, VfsReaderCompat};
use arbhx_core::{DataRead, FilterOptions, Metadata, VfsReader};
use std::io::Read;
use std::path::Path;
use std::sync::Arc;
use tokio::runtime::Handle;

#[derive(Debug, Clone)]
pub struct VfsReaderSync {
    read: Arc<dyn VfsReader>,
    rt: Handle,
}

impl VfsReaderSync {
    pub(crate) fn new(rt: Handle, read: Arc<dyn VfsReader>) -> Self {
        Self { rt, read }
    }
}

impl VfsReaderCompat for VfsReaderSync {
    fn open_read_start(&self, item: &Path) -> std::io::Result<Box<dyn DataReadCompat>> {
        let res = self.rt.block_on(self.read.open_read_start(item))?;
        Ok(Box::new(VfsReadSync::new(self.rt.clone(), res)))
    }

    fn open_read_random(
        &self,
        item: &Path,
    ) -> std::io::Result<Option<Box<dyn DataReadSeekCompat>>> {
        match self.rt.block_on(self.read.open_read_random(item))? {
            Some(x) => Ok(Some(Box::new(VfsReadSeekSync::new(self.rt.clone(), x)))),
            None => Ok(None),
        }
    }

    fn get_metadata(&self, item: &Path) -> std::io::Result<Option<Metadata>> {
        self.rt.block_on(self.read.get_metadata(item))
    }

    fn read_dir(
        &self,
        item: &Path,
        opts: Option<FilterOptions>,
        recursive: bool,
        include_root: bool,
    ) -> std::io::Result<Arc<dyn SizedQueryCompat>> {
        let st = self
            .rt
            .block_on(self.read.read_dir(item, opts, recursive, include_root))?;
        let ret = QueryCompat::new(self.rt.clone(), st);
        Ok(Arc::new(ret))
    }
}
