use crate::Error::CanNotBeAchieved;
use crate::ast::expr::{Assignment, Primary};
use crate::ast::expr::{Expr, Term};
use crate::ast::token::{Condition, Flag, Identifier, Operator};
use crate::parser::instrument::{
    A, B, Ci, DC1, DC2, Instrument, MEM, Mi20, Mi53, Mi86, SCi, SST, ToInstrument,
};
use crate::{CanNotBeAchievedReason, Result, to_bytes};
use std::collections::HashMap;
use std::mem::{Discriminant, discriminant};
use std::rc::Rc;

pub struct ExprParser {
    expr: Rc<Expr>,
    instruments: HashMap<Discriminant<Instrument>, (Instrument, bool)>,
}

impl ExprParser {
    pub fn new(expr: Expr) -> Self {
        let mut obj = Self {
            expr: Rc::new(expr),
            instruments: HashMap::new(),
        };
        let mut func = |instrument: Box<dyn ToInstrument>| {
            let instruments = instrument.to_instrument();
            instruments.into_iter().for_each(|instrument| {
                obj.instruments
                    .insert(discriminant(&instrument), (instrument, true));
            })
        };
        func(Box::new(Ci::SEQ));
        func(Box::new(MEM::NONE));
        func(Box::new(Mi86::NONE));
        obj
    }

    pub fn instruments(&self) -> Vec<&Instrument> {
        self.instruments.values().map(|x| &x.0).collect::<Vec<_>>()
    }

    pub fn bin(&self) -> [u8; 56] {
        let mut result = [0u8; 56];

        self.instruments.values().for_each(|(instrument, _)| {
            let val: Box<&[u8]> = match instrument {
                Instrument::NEXT(val) => Box::new(val),
                Instrument::CI(val) => Box::new(val),
                Instrument::SCC(val) => Box::new(val),
                Instrument::SC(val) => Box::new(val),
                Instrument::SST(val) => Box::new(val),
                Instrument::MIO(val) => Box::new(val),
                Instrument::MI86(val) => Box::new(val),
                Instrument::REQ(val) => Box::new(val),
                Instrument::MI53(val) => Box::new(val),
                Instrument::WE(val) => Box::new(val),
                Instrument::MI20(val) => Box::new(val),
                Instrument::A(val) => Box::new(val),
                Instrument::B(val) => Box::new(val),
                Instrument::SCi(val) => Box::new(val),
                Instrument::SSH(val) => Box::new(val),
                Instrument::SA(val) => Box::new(val),
                Instrument::DC1(val) => Box::new(val),
                Instrument::SB(val) => Box::new(val),
                Instrument::DC2(val) => Box::new(val),
            };
            for i in 0..instrument.length() {
                result[i + instrument.begin() as usize] = val[i];
            }
        });
        result
    }

    pub fn hex(&self) -> [u8; 7] {
        let mut result = [0u8; 7];
        let bin = self.bin();
        for (i, bits_chunk) in bin.chunks_exact(8).enumerate() {
            let mut byte = 0u8;
            for (j, &bit) in bits_chunk.iter().enumerate() {
                let valid_bit = bit & 1;
                byte |= valid_bit << (7 - j);
            }
            result[i] = byte;
        }
        result
    }

    fn push_instrument_with_check(&mut self, instrument: Box<dyn ToInstrument>, check: bool) {
        let instruments = instrument.to_instrument();
        instruments.into_iter().for_each(|instrument| {
            let discriminant = discriminant(&instrument);
            if self.instruments.contains_key(&discriminant) {
                let (origin_instrument, origin_check) =
                    self.instruments.get_mut(&discriminant).unwrap();
                match origin_check {
                    &mut true => {
                        *origin_check = check;
                        *origin_instrument = instrument
                    }
                    &mut false => {
                        if origin_instrument != &instrument {
                            println!("添加命令失败: {:?} 被反复添加", instrument)
                        }
                    }
                }
            } else {
                self.instruments.insert(discriminant, (instrument, check));
            }
        })
    }

    fn push_instrument(&mut self, instrument: Box<dyn ToInstrument>) {
        self.push_instrument_with_check(instrument, false)
    }
}

impl ExprParser {
    pub fn parse(&mut self) -> Result {
        let expr = self.expr.clone();
        if let Some(assignment) = &expr.assignment {
            self.parse_assignment(assignment)?;
        }
        self.parse_flag_exprs(expr.get_flag_vec())?;
        Ok(())
    }

    fn parse_flag_exprs(&mut self, flags: Vec<&Flag>) -> Result {
        flags.into_iter().for_each(|flag| match flag {
            Flag::Condition(condition) => {
                self.push_instrument_with_check(
                    Box::new(Instrument::NEXT([0, 0, 1, 0, 1, 0, 0, 1, 0, 0])),
                    true,
                );
                let (scc, sc): (u8, u8) = match condition {
                    Condition::Zero => (0, 0),
                    Condition::One => (1, 0),
                    Condition::NotFS1 => (2, 0),
                    Condition::NotFS2 => (3, 0),
                    Condition::NotFS3 => (4, 0),
                    Condition::NotWait => (5, 0),
                    Condition::NotC => (2, 1),
                    Condition::NotZ => (3, 1),
                    Condition::NotV => (4, 1),
                    Condition::NotS => (5, 1),
                    Condition::NotINT => (6, 1),
                    Condition::IR108 => (7, 0), // 也可以是(7, 1)
                };
                self.push_instrument(Box::new(Ci::IF));
                self.push_instrument(Box::new(Instrument::SCC(to_bytes!(scc, 3))));
                self.push_instrument(Box::new(Instrument::SC(to_bytes!(sc, 1))));
            }
            Flag::PCStep => {
                self.push_instrument(Box::new(SCi::PCStep));
                self.push_instrument(Box::new(Mi86::FBA));
                self.push_instrument(Box::new(B::FromSB(5))); // 更新R5
                self.push_instrument(Box::new(Mi20::_0B))
            }
            Flag::CarryFromALU => self.push_instrument(Box::new(SST::ALU)),
            Flag::Next(_) => todo!(),
        });
        Ok(())
    }

    fn parse_assignment(&mut self, assignment: &Assignment) -> Result {
        self.parse_term(&assignment.term)?;
        self.parse_assignment_identifier(&assignment.identifier)
    }

    fn parse_term(&mut self, term: &Term) -> Result {
        if let Some((operator, right)) = &term.right {
            self.parse_binary(&term.left, operator, right)
        } else {
            self.parse_unary(&term.left)
        }
    }

    fn parse_unary(&mut self, primary: &Primary) -> Result {
        match primary {
            Primary::Identifier(identifier) => match identifier {
                Identifier::PC => {
                    self.push_instrument(Box::new(Mi86::FBA));
                    self.push_instrument(Box::new(A::FromSA(5))); // 把原来的PC值放到Y再传递到AR里面
                    self.push_instrument(Box::new(B::FromSB(5)));
                    self.push_instrument(Box::new(Mi20::_0B));
                } // 设置B口为R5
                Identifier::AR => {
                    return Err(CanNotBeAchieved(CanNotBeAchievedReason::ARCanNotBeRead));
                }
                Identifier::MEM => {
                    self.push_instrument(Box::new(Mi20::D0));
                    self.push_instrument(Box::new(MEM::MemRead))
                }
                Identifier::SR => {
                    self.push_instrument(Box::new(A::SR));
                    self.push_instrument(Box::new(Mi20::_0A));
                }
                Identifier::DR => {
                    self.push_instrument(Box::new(B::DR));
                    self.push_instrument(Box::new(Mi20::_0B));
                }
                Identifier::Q => {
                    self.push_instrument(Box::new(Mi20::_0Q));
                }
                Identifier::IP => {
                    self.push_instrument(Box::new(B::FromSB(6)));
                    self.push_instrument(Box::new(Mi20::_0B));
                }
                Identifier::R(val) => {
                    self.push_instrument(Box::new(B::FromSB(val.to_owned())));
                    self.push_instrument(Box::new(Mi20::_0B));
                }
            },
            Primary::Number(_) => {
                todo!("赋值")
            }
        };
        Ok(())
    }

    fn parse_assignment_identifier(&mut self, identifier: &Identifier) -> Result {
        match identifier {
            Identifier::PC => {
                // self.push_instrument(Box::new(A::FromSA(5)))
                self.push_instrument(Box::new(Mi86::FBF));
            }
            Identifier::AR => self.push_instrument(Box::new(DC2::AR)),
            Identifier::MEM => {
                self.push_instrument(Box::new(DC1::FromALU));
                self.push_instrument(Box::new(MEM::MemWrite))
            }
            Identifier::SR => {
                return Err(CanNotBeAchieved(CanNotBeAchievedReason::SACanNotBeWrite));
            }
            Identifier::Q => {
                self.push_instrument(Box::new(Mi86::FQF));
            }
            Identifier::DR => {
                self.push_instrument(Box::new(Mi86::FBF));
                self.push_instrument(Box::new(B::DR))
            }
            Identifier::R(val) => {
                self.push_instrument(Box::new(Mi86::FBF));
                self.push_instrument(Box::new(A::FromSA(val.to_owned())));
                self.push_instrument(Box::new(B::FromSB(val.to_owned())))
            }
            Identifier::IP => {
                todo!()
            }
        };
        Ok(())
    }

    fn parse_binary(&mut self, left: &Primary, operator: &Operator, right: &Primary) -> Result {
        match (left, right) {
            (Primary::Identifier(left), Primary::Identifier(right)) => {
                let (mut left, mut right) = (left, right);
                if left == right {
                    return Err(CanNotBeAchieved(
                        CanNotBeAchievedReason::LeftRightCanNotBeSame,
                    ));
                }
                match operator {
                    Operator::Add => self.push_instrument(Box::new(Mi53::RAddS)),
                    Operator::Minus => match (left, right) {
                        (Identifier::Q, Identifier::SR)
                        | (Identifier::DR, Identifier::SR)
                        | (_, Identifier::MEM) => {
                            (left, right) = (right, left);
                            self.push_instrument(Box::new(Mi53::SSubR))
                        }
                        _ => self.push_instrument(Box::new(Mi53::RSubS)),
                    },
                }
                match (left, right) {
                    (Identifier::PC, _) | (_, Identifier::PC) => {
                        todo!()
                    }
                    (Identifier::AR, _) | (_, Identifier::AR) => {
                        return Err(CanNotBeAchieved(CanNotBeAchievedReason::ARCanNotBeRead));
                    }
                    (Identifier::SR, Identifier::Q) | (Identifier::Q, Identifier::SR) => {
                        self.push_instrument(Box::new(A::SR));
                        self.push_instrument(Box::new(Mi20::AQ));
                    }
                    (Identifier::SR, Identifier::DR) | (Identifier::DR, Identifier::SR) => {
                        self.push_instrument(Box::new(A::SR));
                        self.push_instrument(Box::new(B::DR));
                        self.push_instrument(Box::new(Mi20::AB));
                    }
                    (Identifier::MEM, Identifier::SR) | (Identifier::SR, Identifier::MEM) => {
                        self.push_instrument(Box::new(A::SR));
                        self.push_instrument(Box::new(MEM::MemRead));
                        self.push_instrument(Box::new(Mi20::DA));
                    }
                    (Identifier::MEM, Identifier::Q) | (Identifier::Q, Identifier::MEM) => {
                        self.push_instrument(Box::new(MEM::MemRead));
                        self.push_instrument(Box::new(Mi20::DQ));
                    }
                    (Identifier::MEM, Identifier::DR) | (Identifier::DR, Identifier::MEM) => {
                        return Err(CanNotBeAchieved(
                            CanNotBeAchievedReason::DRCanNotInBinaryWithD,
                        ));
                    }
                    (Identifier::IP, Identifier::SR) | (Identifier::SR, Identifier::IP) => {
                        self.push_instrument(Box::new(A::SR));
                        self.push_instrument(Box::new(B::FromSB(6)));
                        self.push_instrument(Box::new(Mi20::AB));
                    }
                    (Identifier::IP, Identifier::DR) | (Identifier::DR, Identifier::IP) => {
                        self.push_instrument(Box::new(B::DR));
                        self.push_instrument(Box::new(A::FromSA(6)));
                        self.push_instrument(Box::new(Mi20::AB));
                    }
                    (Identifier::IP, Identifier::Q) | (Identifier::Q, Identifier::IP) => {
                        self.push_instrument(Box::new(A::FromSA(6)));
                        self.push_instrument(Box::new(Mi20::AQ));
                    }
                    (Identifier::IP, Identifier::MEM) | (Identifier::MEM, Identifier::IP) => {
                        self.push_instrument(Box::new(MEM::MemRead));
                        self.push_instrument(Box::new(A::FromSA(6)));
                        self.push_instrument(Box::new(B::FromSB(5)));
                        self.push_instrument(Box::new(Mi20::DA));
                    }
                    _ => return Err(CanNotBeAchieved(CanNotBeAchievedReason::UnknownExpr)),
                }
            }
            _ => todo!(),
        };
        Ok(())
    }
}
