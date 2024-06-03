use p3_derive::Bus;

#[derive(Bus)]
pub enum Chip0MachineBus {
    ClearBus = 0,
    DrawBus = 1,
    KeypadBus = 2,
    MemoryBus = 3,
    FrameBufferBus = 4,
    RangeBus = 5,
    MemoryStartBus = 6,
    // HashBus = 7,
}
