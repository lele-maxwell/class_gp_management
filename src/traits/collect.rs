pub trait Collect {
    fn collect_input() -> Vec<Self> where Self: Sized;
}