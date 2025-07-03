from enum import IntEnum, Enum
from typing import Tuple, Self

from pydantic import BaseModel, Field

_bin = lambda x: bin(x)[2:]


class CI(IntEnum):
    INIT = 0  # 初始化
    IF = 3  # 条件转移
    SEQ = 14  # 顺序执行


class byte(IntEnum):
    ZERO = 0
    ONE = 1
    ANY = 2

    @property
    def value(self) -> Self:
        match self:
            case byte.ZERO:
                return 0
            case byte.ONE:
                return 1
            case byte.ANY:
                return 0
        return None


class SCC(IntEnum):
    INIT = 0
    CC_Z = 3
    CC_S = 5
    IR10_8 = 7


class CC(Enum):
    ZERO = (SCC.INIT, byte.ZERO)
    CC_Z = (SCC.CC_Z, byte.ONE)
    CC_S = (SCC.CC_S, byte.ONE)
    IR10_8 = (SCC.IR10_8, byte.ZERO)

    @classmethod
    def decode(cls, value: str):
        try:
            value = SCC(int(value, 2))
        except ValueError:
            raise ValueError("CC解析错误")
        match value:
            case SCC.INIT:
                return CC.ZERO
            case SCC.CC_Z:
                return CC.CC_Z
            case SCC.IR10_8:
                return CC.IR10_8
            case SCC.CC_S:
                return CC.CC_S
        raise ValueError("CC解析错误")


class SST(IntEnum):
    CONST = 0  # 四位标志位的值保持不变
    ALU = 1  # 接收ALU的标志位输出


class MEM(Enum):
    MEM_WRITE = (byte.ZERO, byte.ZERO, byte.ZERO)  # 储存器写
    MEM_READ = (byte.ZERO, byte.ZERO, byte.ONE)  # 储存器读
    IO_READ = (byte.ZERO, byte.ONE, byte.ZERO)
    IO_WRITE = (byte.ZERO, byte.ONE, byte.ONE)
    NONE = (byte.ONE, byte.ZERO, byte.ANY)
    LOAD = (byte.ONE, byte.ONE, byte.ANY)

    @classmethod
    def decode(cls, mio: int, req: int, we: int):
        if mio == 0 and req == 0 and we == 0:
            return MEM.MEM_WRITE
        elif mio == 0 and req == 0 and we == 1:
            return MEM.MEM_READ
        elif mio == 0 and req == 1 and we == 0:
            return MEM.IO_READ
        elif mio == 0 and req == 1 and we == 1:
            return MEM.NONE
        elif mio == 1 and req == 0:
            return MEM.NONE
        else:
            return MEM.LOAD


class MI_RESULT(IntEnum):
    FQF = 0  # F->Q F
    NONE = 1  # 无 F
    FBA = 2  # F->B A
    FBF = 3  # F->B F
    F_2BQ_2QF = 4  # F/2->Q F/2->Q F
    F_2BF = 5  # F/2->B F
    _2FB2QQF = 6  # 2F->B 2Q->Q F
    _2FB = 7  # 2F->B  F


class MI_OP(IntEnum):
    R_ADD_S = 0
    S_SUB_R = 1
    R_SUB_S = 2


class MI_SOURCE(IntEnum):
    """
    R S
    """
    A_Q = 0
    A_B = 1
    _0_Q = 2
    _0_B = 3
    _0_A = 4
    D_A = 5
    D_Q = 6
    D_0 = 7


class SA(IntEnum):
    FROM_A = 0
    SR = 1  # 从SR寄存器(IR3-0)


class SB(IntEnum):
    FROM_B = 0
    DR = 1  # 从DR寄存器(IR3-0)


class SSH(IntEnum):
    ZERO = 0
    ONE = 1
    TWO = 2
    THREE = 3


class DC1(IntEnum):
    HANDLE = 0  # 开关手拨数据
    ALU = 1  # 运算器送总线IB


class DC2(IntEnum):
    NONE = 0  # 未使用
    IR = 1  # 指令寄存器
    AR = 2  # 地址寄存器


INSTRUCT_FORMAT = ("{:0>10}"
                   "{:0>2}"
                   "{:0>4}"
                   "{:0>3}"
                   "{:1}"
                   "{:1}"
                   "{:0>3}"
                   "{:1}"
                   "{:0>3}"
                   "{:1}"
                   "{:0>3}"
                   "{:1}"
                   "{:0>3}"
                   "{:0>4}"
                   "{:0>4}"
                   "{:0>2}"
                   "{:0>2}"
                   "{:1}"
                   "{:0>3}"
                   "{:1}"
                   "{:0>3}")


class Instruct(BaseModel):
    next: str = Field(description="下地址, 10位")
    cc: CC = Field(description="条件码，由SCC和SC决定")
    ci: CI = Field(description="控制码")
    mem: MEM = Field(description="MIO REQ WE")
    sst: SST = Field(description="标志寄存器的写入方式")
    sci: int = Field(default=0, description="SCi的输入")
    mi: Tuple[MI_RESULT, MI_OP, MI_SOURCE] = Field(description="ALU的配置")
    a: int = Field(default=0, description="就是从哪个寄存器读取数据")
    b: int = Field(default=0, description="就是从哪个寄存器读取数据")
    sa: SA
    sb: SB
    ssh: SSH = Field(default=SSH.ZERO)
    dc1: DC1 = Field(description="数据控制1")
    dc2: DC2 = Field(description="数据控制2，写回到AR中要用这个")

    def _data(self) -> tuple[
        str, int, str, str, str, int, str, int, str, int, str, int, str, str, str, str, str, str, str, str, str]:
        return (
            self.next,
            0,
            _bin(self.ci),
            _bin(self.cc.value[0]),
            _bin(self.cc.value[1]),
            0,
            _bin(self.sst),
            self.mem.value[0].value,
            _bin(self.mi[0]),
            self.mem.value[1].value,
            _bin(self.mi[1]),
            self.mem.value[2].value,
            _bin(self.mi[2]),
            _bin(self.a),
            _bin(self.b),
            _bin(self.sci),
            _bin(self.ssh),
            _bin(self.sa),
            _bin(self.dc1),
            _bin(self.sb),
            _bin(self.dc2)
        )

    @property
    def bin(self) -> str:
        if self.mi[2] in (MI_SOURCE.D_Q, MI_SOURCE.D_A) and self.mem is not MEM.MEM_READ:
            raise ValueError("D需要读如MEM")
        return INSTRUCT_FORMAT.format(*self._data())

    @property
    def split_bin(self) -> str:
        data = self.bin
        return " ".join([data[i * 4: i * 4 + 4] for i in range(0, 16)])

    @property
    def hex(self) -> str:
        return "{:0>16}".format(format(int(self.bin, 2), 'X'))

    @property
    def table(self) -> str:
        # noinspection PyStringFormat
        return "\n".join((
            "|下地址|备用|CI|SCC|SC|备用|SST|MIO|MI8-6|REQ|MI5-3|WE|MI2-0|A口|B口|SCi|SSH|SA|DC1|SB|DC2|",
            f"|{'-|' * 21}",
            f"|{INSTRUCT_FORMAT.replace('}', '}|')}".format(*self._data()),
        ))

    @classmethod
    def decode(cls, data: str) -> Self:
        return cls(
            next=data[0:10],
            cc=CC.decode(data[16:19]),
            ci=CI(int(data[12:16], 2)),
            mem=MEM.decode(int(data[24]), int(data[28]), int(data[32])),
            sst=SST(int(data[21:24], 2)),
            sci=int(data[44:46], 2),
            mi=(MI_RESULT(int(data[25:28], 2)), MI_OP(int(data[29:32], 2)), MI_SOURCE(int(data[33:36], 2))),
            a=int(data[36:40], 2),
            b=int(data[40:44], 2),
            sa=SA(int(data[48], 2)),
            sb=SB(int(data[52], 2)),
            ssh=SSH(int(data[46:48], 2)),
            dc1=DC1(int(data[49:52], 2)),
            dc2=DC2(int(data[53:56], 2)),
        )

    @classmethod
    def decode_hex(cls, data: str) -> Self:
        data = "{:0>56}".format(_bin(int(data.replace(" ", ""), 16)))
        return cls.decode(data)

    @classmethod
    def decode_bin(cls, data: str) -> Self:
        return cls.decode("{:0>56}".format(data.replace("|", "").replace(" ", "").strip()))


if __name__ == '__main__':
    instruct = Instruct(
        next='0',
        cc=CC.ZERO,
        ci=CI.IF,
        mem=MEM.MEM_WRITE,
        sst=SST.ALU,
        mi=(MI_RESULT.FQF, MI_OP.R_ADD_S, MI_SOURCE._0_B),
        a=0,
        b=0,
        sa=SA.FROM_A,
        sb=SB.FROM_B,
        dc1=DC1.ALU,
        dc2=DC2.NONE
    )
    # print(instruct.bin)
    print(instruct.hex)
    # print(instruct.table)

    # 解码
    # print("解码")
    ins = instruct.decode_hex("0000 0E00 90B0 008A")

    print(ins.table)
    print()
    print(ins.bin)
    print(ins.split_bin)
    print()
    print(ins.hex)
    print(ins.mem)
    print(ins.cc)
    print(ins.mi)
