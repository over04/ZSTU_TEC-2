mod ci;
mod dc;
mod instrument;
mod ir;
mod mem;
mod mi;
mod sci;
mod sst;

pub use ci::*;
pub use dc::*;
pub use instrument::*;
pub use ir::*;
pub use mem::*;
pub use mi::*;
pub use sci::*;
pub use sst::*;

pub trait ToInstrument {
    fn to_instrument(&self) -> Box<[Instrument]>;
}
