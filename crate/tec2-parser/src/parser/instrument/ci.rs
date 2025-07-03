use crate::parser::instrument::{Instrument, ToInstrument};
use crate::to_bytes;
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Debug, Clone, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum Ci {
    INIT = 0, // 初始化
    IF = 3,   // 条件转移
    SEQ = 14, // 顺序执行
}

impl ToInstrument for Ci {
    fn to_instrument(&self) -> Box<[Instrument]> {
        Box::new([Instrument::CI(to_bytes!(self.to_owned() as u8, 4))])
    }
}
