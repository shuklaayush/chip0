use p3_derive::AirColumns;

#[repr(C)]
#[derive(AirColumns, Default, Clone)]
pub struct HashCols<T> {
    pub is_real: T,
}
