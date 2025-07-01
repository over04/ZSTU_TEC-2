use tect_2::grammar;
use tect_2::parser::parser::ExprParser;

fn get_hex(input: &str) -> String {
    let mut parser = ExprParser::new(grammar::ExprParser::new().parse(input).unwrap());
    parser.parse().unwrap();
    hex::encode(parser.hex()).to_uppercase()
}

#[test]
fn test_1() {
    assert_eq!(get_hex(", PC + 1 -> PC"), "000E00A0305400");
    assert_eq!(get_hex("PC -> AR, PC + 1 -> PC"), "000E00A0355402"); // 待更新000E00A0B55402
    assert_eq!(get_hex("MEM -> DR, CarryFromALU"), "000E0130F00008");
    assert_eq!(get_hex("MEM -> DR"), "000E0030F00008");
    assert_eq!(get_hex("MEM -> AR"), "000E0010F00002");
    assert_eq!(get_hex("DR -> MEM, CC#=0"), "29030010300018");
    assert_eq!(get_hex("DR -> MEM, CC#=0, CarryFromALU"), "29030110300018");
    assert_eq!(get_hex("Q -> AR"), "000E0090200002");
    assert_eq!(get_hex("Q -> AR, CarryFromALU"), "000E0190200002");
    assert_eq!(get_hex("MEM -> Q"), "000E0000F00000");
    assert_eq!(get_hex("SR -> AR"), "000E0090400082");
    assert_eq!(get_hex("MEM + Q -> AR, CarryFromALU"), "000E0110E00002");
    assert_eq!(get_hex("MEM + Q -> AR"), "000E0010E00002");
    assert_eq!(get_hex("MEM + Q -> Q"), "000E0000E00000");
    assert_eq!(get_hex("MEM + Q -> Q, CarryFromALU"), "000E0100E00000");
    assert_eq!(get_hex("MEM - Q -> Q"), "000E0002E00000");
    assert_eq!(get_hex("MEM - Q -> Q, CarryFromALU"), "000E0102E00000");
    assert_eq!(get_hex("DR -> AR"), "000E009030000A"); // 待更新000E009030008A
    assert_eq!(get_hex("Q -> MEM, CC#=0"), "29030010200010");
    assert_eq!(get_hex("SR - DR -> Q, CarryFromALU"), "000E0182100088"); // 待更新000E0192100088
    assert_eq!(get_hex("PC -> AR , PC + 1 -> PC, CC#=Z"), "2903E0A0355402");
    assert_eq!(get_hex("IP + MEM -> PC, CC#=0"), "29030030D65000");
}
