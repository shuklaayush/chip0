use p3_derive::Columnar;

#[repr(C)]
#[derive(Columnar, Default, Clone)]
pub struct FrameBufferCols<T> {
    pub addr: T,
    pub clk: T,
    pub value: T,
    pub is_read: T,
    pub is_write: T,
    pub addr_unchanged: T,
    pub diff_limb_lo: T,
    pub diff_limb_hi: T,
    pub is_first_read: T,
    pub is_last_write: T,
}
