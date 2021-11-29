use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::path::Path;
use crate::model::{Model, ModelLoader};
use anyhow::{Context, Result};
use wgpu::{BindGroupLayout, Device, Queue};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LoadError {
    #[error("File does not exist")]
    FileDoesNotExist,
    #[error("Unknown file format")]
    UnknownFileFormat,
    #[error("Unable to read file")]
    UnableToReadFile,
    #[error("Another error occurred loading the file: {0}")]
    OtherError(String)
}

pub struct ContentLoader<'a> {
    base_path: Box<Path>,
    model_loaders: Vec<Box<dyn ModelLoader>>,
    device: &'a Device,
    queue: &'a Queue,
    layout: &'a BindGroupLayout,
}

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
        if !path.as_ref().exists() {
            return Err(LoadError::FileDoesNotExist);
        }

        match self.model_loaders.iter().find(|x| x.can_handle_extension(path.as_ref())) {
            Some(loader) => loader.load(self.device, self.queue, self.layout, path.as_ref()),
            None => Err(LoadError::UnknownFileFormat)
        }
    }
}