#[derive(Debug, PartialEq)]
pub enum Identifier {
    PC,    // PC
    AR,    // 地址寄存器
    MEM,   // 内存
    SR,    // SR寄存器，A寄存器
    Q,     // Q，ALU输出寄存器
    DR,    // DR寄存器，B寄存器
    IP,    // IP，当前执行指令的地址
    R(u8), // 普通寄存器，R0、R1...
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    Add,   // 加法
    Minus, // 减法
}

#[derive(Debug)]
pub enum Condition {
    Zero, // 满足条件
    One,
    NotFS1,
    NotFS2,
    NotFS3,
    NotWait,
    NotS,
    NotV,
    NotZ,
    NotC,
    NotINT,
    IR108, // 从IR10-8读取数据，比如说你把程序写入到100H中，然后用D5（表中对应是D4）调用，那么SC=5，/CC= Z
}

#[derive(Debug)]
pub enum Extra {
    CC,
    ASSIGN,
    GAP,
    EQUAL,
}

#[derive(Debug)]
pub enum Flag {
    Condition(Condition),
    PCStep,
    CarryFromALU,
    Next(u16),
}
