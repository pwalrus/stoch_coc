
pub trait SearchModel<T> {
    fn done(&self, x: &T) -> bool;
    fn finalize(&self, x: T) -> Result<T,String>;
    fn next(&self, x: &T) -> Vec<T>;
    fn weight(&self, x: &T) -> i32;
}

