use p3_derive::Columnar;

#[repr(C)]
#[derive(Columnar)]
pub struct IsEqualCols<T> {
    pub x: T,
    pub y: T,

    pub diff_inv: T,
    pub is_equal: T,
}
