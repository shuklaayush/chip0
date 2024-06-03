use p3_derive::Columnar;

#[repr(C)]
#[derive(Columnar, Default, Clone)]
pub struct HashCols<T> {
    pub is_real: T,
}
