use arbhx_core::blocking::{DataReadCompat, DataReadSeekCompat};
use arbhx_core::DataReadSeek;
use std::io::{Read, Seek, SeekFrom};
use tokio::io::AsyncReadExt;
use tokio::io::AsyncSeekExt;
use tokio::runtime::Handle;

#[derive(Debug)]
pub struct VfsReadSeekSync {
    read: Box<dyn DataReadSeek>,
    rt: Handle,
}

impl VfsReadSeekSync {
    pub(crate) fn new(rt: Handle, read: Box<dyn DataReadSeek>) -> Self {
        Self { rt, read }
    }
}

impl Read for VfsReadSeekSync {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.rt.block_on(self.read.read(buf))
    }
}

impl Seek for VfsReadSeekSync {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        self.rt.block_on(self.read.seek(pos))
    }
}

impl DataReadCompat for VfsReadSeekSync {}

impl DataReadSeekCompat for VfsReadSeekSync {}
