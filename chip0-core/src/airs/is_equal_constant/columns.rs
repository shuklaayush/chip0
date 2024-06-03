use p3_derive::Columnar;

#[repr(C)]
#[derive(Columnar)]
pub struct IsEqualConstantCols<T> {
    pub x: T,

    pub diff_inv: T,
    pub is_equal: T,
}
