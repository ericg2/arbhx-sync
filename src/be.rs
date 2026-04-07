use crate::read::VfsReaderSync;
use crate::write::VfsWriterSync;
use arbhx_core::blocking::{VfsBackendCompat, VfsReaderCompat, VfsWriterCompat};
use arbhx_core::{DataUsage, VfsBackend};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::runtime::Handle;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct VfsBackendSync {
    be: Arc<dyn VfsBackend>,
    rt: Handle,
}

impl VfsBackendSync {
    pub fn new(rt: Handle, be: Arc<dyn VfsBackend>) -> Self {
        Self { rt, be }
    }
}

impl VfsBackendCompat for VfsBackendSync {
    fn id(&self) -> Uuid {
        self.be.id()
    }

    fn name(&self) -> &str {
        self.be.name()
    }

    fn realpath(&self, item: &Path) -> PathBuf {
        self.be.realpath(item)
    }

    fn reader(self: Arc<Self>) -> Option<Arc<dyn VfsReaderCompat>> {
        match self.be.clone().reader() {
            Some(x) => {
                let ret = VfsReaderSync::new(self.rt.clone(), x);
                Some(Arc::new(ret))
            }
            None => None,
        }
    }

    fn writer(self: Arc<Self>) -> Option<Arc<dyn VfsWriterCompat>> {
        match self.be.clone().writer() {
            Some(x) => {
                let ret = VfsWriterSync::new(self.rt.clone(), x);
                Some(Arc::new(ret))
            }
            None => None,
        }
    }

    fn get_usage(&self) -> std::io::Result<Option<DataUsage>> {
        self.rt.block_on(self.be.get_usage())
    }
}
