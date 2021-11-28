use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::path::Path;
use crate::model::{Model, ModelLoader};
use anyhow::{Context, Result};
use wgpu::{BindGroupLayout, Device, Queue};

struct ContentLoader<'a> {
    base_path: Box<Path>,
    model_loaders: Vec<Box<dyn ModelLoader>>,
    device: &'a Device,
    queue: &'a Queue,
    layout: &'a BindGroupLayout,
}

#[derive(Debug)]
pub enum LoadError {
    UnknownFileFormat
}

impl Display for LoadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LoadError::UnknownFileFormat => write!(f, "Unknown file format"),
        }
    }
}

impl Error for LoadError {}

impl<'a> ContentLoader<'a> {
    pub fn new(base_path: Box<Path>, device: &'a Device, queue: &'a Queue, layout: &'a BindGroupLayout) -> Self {
        ContentLoader {
            base_path,
            model_loaders: Vec::new(),
            device,
            queue,
            layout,
        }
    }

    pub fn register_model_loader(&mut self, loader: Box<dyn ModelLoader>) {
        self.model_loaders.push(loader);
    }

    pub fn load_model<P: AsRef<Path>>(&self, path: P) -> Result<Model, LoadError> {
        /*for loader in &self.model_loaders {
            if loader.can_handle_extension(path) {
                return loader
            }
        }*/

        unimplemented!()
        /*match self.model_loaders.iter().find(|x| x.can_handle_extension(path.as_ref())) {
            Some(loader) => loader.load(self.device, self.queue, self.layout, path.as_ref()),
            None => Err(LoadError::UnknownFileFormat)
        }*/
    }
}