use crate::parser::instrument::{Instrument, ToInstrument};
use crate::to_bytes;
use num_enum::{IntoPrimitive, TryFromPrimitive};

/// Mi86就是MI的寄存器选择和Y输出选择
#[derive(Debug, Clone, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum Mi86 {
    FQF = 0,      // F->Q F
    NONE = 1,     // 无 F
    FBA = 2,      // F->B A
    FBF = 3,      // F->B F
    F2BQ2QF = 4,  // F/2->Q F/2->Q F
    F2BF = 5,     // F/2->B F
    _2FB2QQF = 6, // 2F->B 2Q->Q F
    _2FB = 7,     // 2F->B  F
}

#[derive(Debug, Clone, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum Mi53 {
    RAddS = 0,
    SSubR = 1,
    RSubS = 2,
}

/// 数据的来源，D为MEM读取
#[derive(Debug, Clone, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum Mi20 {
    AQ = 0,
    AB = 1,
    _0Q = 2,
    _0B = 3,
    _0A = 4,
    DA = 5,
    DQ = 6,
    D0 = 7,
}

impl ToInstrument for Mi86 {
    fn to_instrument(&self) -> Box<[Instrument]> {
        Box::new([Instrument::MI86(to_bytes!(self.to_owned() as u8, 3))])
    }
}

impl ToInstrument for Mi53 {
    fn to_instrument(&self) -> Box<[Instrument]> {
        Box::new([Instrument::MI53(to_bytes!(self.to_owned() as u8, 3))])
    }
}

impl ToInstrument for Mi20 {
    fn to_instrument(&self) -> Box<[Instrument]> {
        Box::new([Instrument::MI20(to_bytes!(self.to_owned() as u8, 3))])
    }
}
