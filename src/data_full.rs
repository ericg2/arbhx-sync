use tokio::io::AsyncWriteExt;
use std::io::{Read, Seek, SeekFrom, Write};
use arbhx_core::blocking::{DataFullCompat, DataReadCompat, DataReadSeekCompat, DataWriteCompat, DataWriteSeekCompat};
use arbhx_core::{DataFull, DataReadSeek};
use tokio::io::{AsyncReadExt, AsyncSeekExt};
use tokio::runtime::Handle;

#[derive(Debug)]
pub struct VfsFullSeekSync {
    full: Box<dyn DataFull>,
    rt: Handle,
}

impl VfsFullSeekSync {
    pub(crate) fn new(rt: Handle, read: Box<dyn DataFull>) -> Self {
        Self { rt, full: read }
    }
}

impl Read for VfsFullSeekSync {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.rt.block_on(self.full.read(buf))
    }
}

impl Seek for VfsFullSeekSync {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        self.rt.block_on(self.full.seek(pos))
    }
}

impl Write for VfsFullSeekSync {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.rt.block_on(self.full.write(buf))
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.rt.block_on(self.full.flush())
    }
}

impl DataReadCompat for VfsFullSeekSync {}

impl DataReadSeekCompat for VfsFullSeekSync {}

impl DataWriteSeekCompat for VfsFullSeekSync {}

impl DataFullCompat for VfsFullSeekSync {}

impl DataWriteCompat for VfsFullSeekSync {
    fn close(&mut self) -> std::io::Result<()> {
        self.rt.block_on(self.full.close())
    }
}