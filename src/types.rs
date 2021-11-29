pub struct Sentinel<T> {
    data: T,
    touched: bool,
}

impl<T: std::cmp::PartialEq> Sentinel<T> {
    pub fn set(&mut self, value: T) {
        if self.data != value {
            self.data = value;
            self.touched = true;
        }
    }

    pub fn get(&self) -> &T { &self.data }
}