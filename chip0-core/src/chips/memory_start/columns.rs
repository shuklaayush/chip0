use p3_derive::AirColumns;

#[repr(C)]
#[derive(AirColumns, Default, Clone)]
pub struct MemoryStartPreprocessedCols<T> {
    pub addr: T,
    pub value: T,
}

#[repr(C)]
#[derive(AirColumns, Default, Clone)]
pub struct MemoryStartCols<T> {
    pub mult: T,
}
