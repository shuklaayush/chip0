use p3_derive::Columnar;

#[repr(C)]
#[derive(Columnar)]
pub struct LoopCounterCols<T> {
    pub is_start: T,
    pub counter: T,
}
