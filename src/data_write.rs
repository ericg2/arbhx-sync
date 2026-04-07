use std::fmt::{Debug, Formatter};
use std::io::Write;
use arbhx_core::blocking::{DataWriteCompat, VfsWriterCompat};
use arbhx_core::DataWrite;
use tokio::io::AsyncWriteExt;
use tokio::runtime::Handle;

#[derive(Debug)]
pub struct VfsWriteSync {
    write: Box<dyn DataWrite>,
    rt: Handle
}

impl VfsWriteSync {
    pub fn new(rt: Handle, write: Box<dyn DataWrite>) -> Self {
        Self { rt, write }
    }
}

impl Write for VfsWriteSync {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.rt.block_on(self.write.write(buf))
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.rt.block_on(self.write.flush())
    }
}

impl DataWriteCompat for VfsWriteSync {
    fn close(&mut self) -> std::io::Result<()> {
        self.rt.block_on(self.write.close())
    }
}