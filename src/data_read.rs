use arbhx_core::DataRead;
use arbhx_core::blocking::DataReadCompat;
use std::io::Read;
use tokio::io::AsyncReadExt;
use tokio::runtime::Handle;

#[derive(Debug)]
pub struct VfsReadSync {
    read: Box<dyn DataRead>,
    rt: Handle,
}

impl VfsReadSync {
    pub(crate) fn new(rt: Handle, read: Box<dyn DataRead>) -> Self {
        Self { rt, read }
    }
}

impl Read for VfsReadSync {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.rt.block_on(self.read.read(buf))
    }
}

impl DataReadCompat for VfsReadSync {}
