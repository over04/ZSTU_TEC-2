use crate::parser::instrument::{Instrument, ToInstrument};
use crate::to_bytes;
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Debug, Clone, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum MIO {
    Zero = 0,
    One = 1,
}

#[derive(Debug, Clone, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum REQ {
    Zero = 0,
    One = 1,
}

#[derive(Debug, Clone, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum WE {
    Zero = 0,
    One = 1,
}

pub enum MEM {
    MemWrite,
    MemRead,
    IoRead,
    IoWrite,
    NONE,
    LOAD,
}

impl ToInstrument for MIO {
    fn to_instrument(&self) -> Box<[Instrument]> {
        Box::new([Instrument::MIO(to_bytes!(self.to_owned() as u8, 1))])
    }
}

impl ToInstrument for REQ {
    fn to_instrument(&self) -> Box<[Instrument]> {
        Box::new([Instrument::REQ(to_bytes!(self.to_owned() as u8, 1))])
    }
}

impl ToInstrument for WE {
    fn to_instrument(&self) -> Box<[Instrument]> {
        Box::new([Instrument::WE(to_bytes!(self.to_owned() as u8, 1))])
    }
}

impl ToInstrument for MEM {
    fn to_instrument(&self) -> Box<[Instrument]> {
        let val = match self {
            MEM::MemWrite => (0u8, 0u8, 0u8),
            MEM::MemRead => (0u8, 0u8, 1u8),
            MEM::IoRead => (0u8, 1u8, 0u8),
            MEM::IoWrite => (0u8, 1u8, 1u8),
            MEM::NONE => (1u8, 0u8, 0u8),
            MEM::LOAD => (1u8, 1u8, 0u8),
        };
        Box::new([
            Instrument::MIO(to_bytes!(val.0, 1)),
            Instrument::REQ(to_bytes!(val.1, 1)),
            Instrument::WE(to_bytes!(val.2, 1)),
        ])
    }
}
