use crate::parser::instrument::{Instrument, ToInstrument};
use crate::to_bytes;

pub enum A {
    FromSA(u8), // = 0
    SR,         // =1
}

pub enum B {
    FromSB(u8), // = 0
    DR,         // =1
}

impl ToInstrument for A {
    fn to_instrument(&self) -> Box<[Instrument]> {
        match self {
            A::FromSA(val) => Box::new([Instrument::A(to_bytes!(val, 4)), Instrument::SA([0])]),
            A::SR => Box::new([Instrument::SA([1])]),
        }
    }
}

impl ToInstrument for B {
    fn to_instrument(&self) -> Box<[Instrument]> {
        match self {
            B::FromSB(val) => Box::new([Instrument::B(to_bytes!(val, 4)), Instrument::SB([0])]),
            B::DR => Box::new([Instrument::SB([1])]),
        }
    }
}
