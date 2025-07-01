use std::io::Write;
use tect_2::grammar;
use tect_2::parser::parser::ExprParser;

fn main() {
    let mut input = String::new();
    loop {
        print!(">> ");
        std::io::stdout().flush().unwrap();
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        match grammar::ExprParser::new().parse(input.trim_end()) {
            Ok(expr) => {
                // println!("{:#?}", expr);
                let mut parser = ExprParser::new(expr);
                match parser.parse() {
                    Ok(_) => {
                        println!("{:?}", parser.bin());
                        let hex_string = hex::encode(parser.hex());
                        println!("{:?}", hex_string.to_uppercase());
                    }
                    Err(e) => {
                        println!("报错: {}", e)
                    }
                }
            }
            Err(e) => {
                println!("{:#?}", e);
            }
        }
    }
}
