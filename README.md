# 介绍

计算机组成原理课设实在是太费时间了，正好这学期也学了编译原理（~~其实这个项目使用了lalroop这个工具，和yacc类似，实和编译原理考试内容无关~~），所以就有了这个项目，因为没有多层次的递归，这个文法还是非常简单的，附赠一个python的二进制/十六进制转换工具(tec-2.py)

## 文法

```
expr ->  assignment flag_expr
flag_expr ->  ε | GAP flag_expr_ flag_expr?
flag_expr_ -> CC ASSIGN Condition | PCStep | CarryFromALU
assignment -> term rux Identifier
term -> primary (Oprator primary)?
primary -> Identifier | Number
```

## 语法

语法分为两部分，操作和指令标志符，可以查看了`crate/tec-2/src/test/tests/parser.rs`文件中的语法

### 操作

```
PC -> AR
MEM -> Q
SR -> AR
MEM - Q -> Q
...
```

这些全部都是操作，是可以选提供的

### 指令标志符

#### 1. PC + 1 -> PC

用于PC进位（PS: 实际上应该设计成多个命令能够合并，而不是使用标志符号）

#### 2.CC#=XXX

XXX有多种写法

1. 0,满足条件; 1,不满足条件
2. \S, \V, \Z, \C
3. S, V, Z, C，选择这些后必须要传入IR10-8才能生效

#### 3.CarryFromALU

标志符的来源，默认是不变，使用此标志符后来源于ALU
