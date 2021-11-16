use std::path::Path;

struct ContentLoader {
    base_path: Box<Path>,
}

impl ContentLoader {
    pub fn new(base_path: Box<Path>) -> Self {
        ContentLoader {
            base_path,
        }
    }

}