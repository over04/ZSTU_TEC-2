use crate::parser::instrument::{Instrument, ToInstrument};
use crate::to_bytes;
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Debug, Clone, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum DC1 {
    None = 0,
    FromALU = 1, // 运算器送地址总线
}

#[derive(Debug, Clone, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum DC2 {
    None = 0, // 未使用
    IR = 1,   // 指令寄存器
    AR = 2,   // 地址寄存器
}

impl ToInstrument for DC1 {
    fn to_instrument(&self) -> Box<[Instrument]> {
        Box::new([Instrument::DC1(to_bytes!(self.to_owned() as u8, 3))])
    }
}

impl ToInstrument for DC2 {
    fn to_instrument(&self) -> Box<[Instrument]> {
        Box::new([Instrument::DC2(to_bytes!(self.to_owned() as u8, 3))])
    }
}
