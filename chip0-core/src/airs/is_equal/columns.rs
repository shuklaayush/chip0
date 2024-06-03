use p3_derive::Columns;

#[repr(C)]
#[derive(Columns)]
pub struct IsEqualCols<T> {
    pub x: T,
    pub y: T,

    pub diff_inv: T,
    pub is_equal: T,
}
