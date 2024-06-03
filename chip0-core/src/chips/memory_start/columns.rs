use p3_derive::Columnar;

#[repr(C)]
#[derive(Columnar, Default, Clone)]
pub struct MemoryStartPreprocessedCols<T> {
    pub addr: T,
    pub value: T,
}

#[repr(C)]
#[derive(Columnar, Default, Clone)]
pub struct MemoryStartCols<T> {
    pub mult: T,
}
