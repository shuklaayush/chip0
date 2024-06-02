use p3_derive::Columns;

#[repr(C)]
#[derive(Columns, Default, Clone)]
pub struct HashCols<T> {
    pub is_real: T,
}
