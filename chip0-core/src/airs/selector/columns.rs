use p3_derive::Columns;

#[repr(C)]
#[derive(Columns)]
pub struct SelectorCols<T, const N: usize> {
    pub selectors: [T; N],
}
