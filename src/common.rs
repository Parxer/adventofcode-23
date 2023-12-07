#[derive(PartialEq, Clone, Copy)]
pub enum Part {
    First,
    Second
}

#[macro_export]
macro_rules! debug {
 ($e:expr) => {
     if env::var("AOC_DEBUG").is_ok() {
        println!("{:?}", $e);
    }
 };
}