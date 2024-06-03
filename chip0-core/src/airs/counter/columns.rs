use p3_derive::Columnar;

#[repr(C)]
#[derive(Columnar)]
pub struct CounterCols<T> {
    pub counter: T,
}
