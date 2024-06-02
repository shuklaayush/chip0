use p3_derive::Columns;

#[repr(C)]
#[derive(Columns, Default, Clone)]
pub struct MemoryStartPreprocessedCols<T> {
    pub addr: T,
    pub value: T,
}

#[repr(C)]
#[derive(Columns, Default, Clone)]
pub struct MemoryStartCols<T> {
    pub mult: T,
}
