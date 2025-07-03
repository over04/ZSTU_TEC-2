use crate::parser::instrument::{Instrument, ToInstrument};
use crate::to_bytes;
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Debug, Clone, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum SCi {
    None = 0,
    PCStep = 1,
}

impl ToInstrument for SCi {
    fn to_instrument(&self) -> Box<[Instrument]> {
        Box::new([Instrument::SCi(to_bytes!(self.to_owned() as u8, 2))])
    }
}
