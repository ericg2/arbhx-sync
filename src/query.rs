use arbhx_core::blocking::{MetaStream, SizedQueryCompat};
use arbhx_core::{Metadata, SizedQuery};
use futures_lite::StreamExt;
use std::io;
use std::pin::Pin;
use std::sync::Arc;
use tokio::runtime::Handle;

pub struct QueryCompat {
    rt: Handle,
    query: Arc<dyn SizedQuery>,
}

impl QueryCompat {
    pub(crate) fn new(rt: Handle, query: Arc<dyn SizedQuery>) -> Self {
        Self { rt, query }
    }
}

impl SizedQueryCompat for QueryCompat {
    fn size(self: Arc<Self>) -> std::io::Result<Option<u64>> {
        self.rt.block_on(self.query.clone().size())
    }

    fn stream(self: Arc<Self>) -> std::io::Result<Box<MetaStream>> {
        let st = self.rt.block_on(self.query.clone().stream())?;
        let ret = Box::new(QueryStreamCompat::new(self.rt.clone(), st));
        Ok(ret)
    }
}

pub struct QueryStreamCompat {
    rt: Handle,
    stream: Pin<Box<arbhx_core::MetaStream>>,
}

impl QueryStreamCompat {
    pub(crate) fn new(rt: Handle, stream: Pin<Box<arbhx_core::MetaStream>>) -> Self {
        Self { rt, stream }
    }
}

impl Iterator for QueryStreamCompat {
    type Item = io::Result<Metadata>;

    fn next(&mut self) -> Option<Self::Item> {
        self.rt.block_on(self.stream.next())
    }
}
