use p3_derive::Columnar;

#[repr(C)]
#[derive(Columnar, Default, Clone)]
pub struct ModuloCounterCols<T> {
    pub counter: T,
    pub diff_inv: T,
    pub is_max: T,
}
