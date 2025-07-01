use crate::map;
use crate::parser::instrument::ToInstrument;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::mem::Discriminant;
use std::mem::discriminant;

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
pub enum Instrument {
    NEXT([u8; 10]),
    CI([u8; 4]),
    SCC([u8; 3]),
    SC([u8; 1]),
    SST([u8; 3]),
    MIO([u8; 1]),
    MI86([u8; 3]),
    REQ([u8; 1]),
    MI53([u8; 3]),
    WE([u8; 1]),
    MI20([u8; 3]),
    A([u8; 4]),
    B([u8; 4]),
    SCi([u8; 2]),
    SSH([u8; 2]),
    SA([u8; 1]),
    DC1([u8; 3]),
    SB([u8; 1]),
    DC2([u8; 3]),
}

lazy_static! {
    static ref E_INSTRUMENTS_MAP: HashMap<Discriminant<Instrument>, (u8, u8)> = map![
        discriminant(&Instrument::NEXT([0;10])) => (0, 10),
        discriminant(&Instrument::CI([0;4])) => (12, 4),
        discriminant(&Instrument::SCC([0;3])) => (16, 3),
        discriminant(&Instrument::SC([0;1])) => (19, 1),
        discriminant(&Instrument::SST([0;3])) => (21, 3),
        discriminant(&Instrument::MIO([0;1])) => (24, 1),
        discriminant(&Instrument::MI86([0;3])) => (25, 3),
        discriminant(&Instrument::REQ([0;1])) => (28, 1),
        discriminant(&Instrument::MI53([0;3])) => (29, 3),
        discriminant(&Instrument::WE([0;1])) => (32, 1),
        discriminant(&Instrument::MI20([0;3])) => (33, 3),
        discriminant(&Instrument::A([0;4])) => (36, 4),
        discriminant(&Instrument::B([0;4])) => (40, 4),
        discriminant(&Instrument::SCi([0;2])) => (44, 2),
        discriminant(&Instrument::SSH([0;2])) => (46, 2),
        discriminant(&Instrument::SA([0;1])) => (48, 1),
        discriminant(&Instrument::DC1([0;3])) => (49, 3),
        discriminant(&Instrument::SB([0;1])) => (52, 1),
        discriminant(&Instrument::DC2([0;3])) => (53, 3),
    ];
}

impl Instrument {
    pub fn length(&self) -> usize {
        E_INSTRUMENTS_MAP[&discriminant(self)].1 as usize
    }

    pub fn begin(&self) -> u8 {
        E_INSTRUMENTS_MAP[&discriminant(self)].0
    }

    pub fn end(&self) -> u8 {
        self.begin() + E_INSTRUMENTS_MAP[&discriminant(self)].1
    }
}

impl ToInstrument for Instrument {
    fn to_instrument(&self) -> Box<[Instrument]> {
        Box::new([self.to_owned()])
    }
}

