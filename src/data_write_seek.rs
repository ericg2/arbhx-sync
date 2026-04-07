use std::fmt::{Debug, Formatter};
use std::io::{Seek, SeekFrom, Write};
use arbhx_core::blocking::{DataWriteCompat, DataWriteSeekCompat, VfsWriterCompat};
use arbhx_core::{DataWrite, DataWriteSeek};
use tokio::io::{AsyncSeekExt, AsyncWriteExt};
use tokio::runtime::Handle;

#[derive(Debug)]
pub struct VfsWriteSeekSync {
    write: Box<dyn DataWriteSeek>,
    rt: Handle
}

impl VfsWriteSeekSync {
    pub fn new(rt: Handle, write: Box<dyn DataWriteSeek>) -> Self {
        Self { rt, write }
    }
}

impl Write for VfsWriteSeekSync {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.rt.block_on(self.write.write(buf))
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.rt.block_on(self.write.flush())
    }
}

impl Seek for VfsWriteSeekSync {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        self.rt.block_on(self.write.seek(pos))
    }
}

impl DataWriteSeekCompat for VfsWriteSeekSync {}

impl DataWriteCompat for VfsWriteSeekSync {
    fn close(&mut self) -> std::io::Result<()> {
        self.rt.block_on(self.write.close())
    }
}