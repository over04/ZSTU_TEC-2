use std::fmt::Formatter;

pub enum Error {
    CanNotBeAchieved(CanNotBeAchievedReason),
}

pub enum CanNotBeAchievedReason {
    SACanNotBeWrite,
    LeftRightCanNotBeSame,
    ARCanNotBeRead, // 存疑
    DRCanNotInBinaryWithD,
    UnknownExpr,
}

pub type Result = std::result::Result<(), Error>;

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Error::CanNotBeAchieved(reason) => {
                f.write_str("代码无法实现:")?;
                reason.fmt(f)
            }
        }
    }
}

impl std::fmt::Display for CanNotBeAchievedReason {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CanNotBeAchievedReason::SACanNotBeWrite => {
                f.write_str("SA不能被写入(也有可能是我不会写)")
            }
            CanNotBeAchievedReason::LeftRightCanNotBeSame => f.write_str("表达式左右两侧不能相等"),
            CanNotBeAchievedReason::ARCanNotBeRead => f.write_str("AR不能读取"),
            CanNotBeAchievedReason::UnknownExpr => f.write_str("未知的表达式"),
            CanNotBeAchievedReason::DRCanNotInBinaryWithD => {
                f.write_str("DR不能和D(MEM)组成二元表达式")
            }
        }
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self, f)
    }
}
