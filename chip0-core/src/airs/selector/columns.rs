use p3_derive::Columnar;

#[repr(C)]
#[derive(Columnar)]
pub struct SelectorCols<T, const N: usize> {
    pub selectors: [T; N],
}
