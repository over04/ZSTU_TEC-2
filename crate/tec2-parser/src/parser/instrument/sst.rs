use crate::parser::instrument::{Instrument, ToInstrument};
use crate::to_bytes;
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Debug, Clone, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum SST {
    Keep = 0,       // 四位标志位保持不变
    ALU = 1,        // 接收ALU标志位输出
    Recover = 2,    // 恢复标志位原现场值
    SetC0 = 3,      // 设置C为0，其他保持不变
    SetC1 = 4,      // 设置C为1，其他保持不变
    Right = 5,      // C右移操作，其他保持不变
    Left = 6,       // C左移操作，其他保持不变
    UnionRight = 7, // C联合右移操作，其他保持不变
}

impl ToInstrument for SST {
    fn to_instrument(&self) -> Box<[Instrument]> {
        Box::new([Instrument::SST(to_bytes!(self.to_owned() as u8, 3))])
    }
}
