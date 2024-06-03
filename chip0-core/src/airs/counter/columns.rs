use p3_derive::Columns;

#[repr(C)]
#[derive(Columns)]
pub struct CounterCols<T> {
    pub counter: T,
}
