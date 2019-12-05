extern crate dummy_cli_parser;

use dummy_cli_parser::{CliParser, PatternType};

fn main() {
    struct ParseObj {
        f1: i32,
        f2: i32,
        f3: i32,
        f4: i32,
    };

    let mut parser = CliParser::new(ParseObj{
        f1: 0,
        f2: 1,
        f3: 2,
        f4: 3,
    });

    parser.register_pattern("-f1", PatternType::WithArg, "update f1 field of ParseObj", 
        |arg_str, parse_obj| {
            arg_str.parse::<i32>().map(|f1|{
                parse_obj.f1 = f1;
            }).map_err(|_|{
                String::from(format!("fail to parse argument \"{}\"", &arg_str))
            })
        }
    ).unwrap();

    parser.register_pattern("-f2", PatternType::WithoutArg, "update f2 field of ParseObj", 
        |_, parse_obj| {
            parse_obj.f2 = 1024;
            Ok(())
        }
    ).unwrap();

    parser.register_pattern("-f3", PatternType::OptionalWithArg, "update f3 field of ParseObj", 
        |arg_str, parse_obj| {
            arg_str.parse::<i32>().map(|f3|{
                parse_obj.f3 = f3;
            }).map_err(|_|{
                String::from(format!("fail to parse argument \"{}\"", &arg_str))
            })
        }
    ).unwrap();

    parser.register_pattern("-f4", PatternType::OptionalWithoutArg, "update f4 field of ParseObj", 
        |_, parse_obj| {
            parse_obj.f4 = 1024;
            Ok(())
        }
    ).unwrap();

    let parse_obj;
    
    match parser.parse_env_args() {
        Ok(obj) => parse_obj = obj,
        Err(err_msg) => {
            println!("{}", err_msg);
            return;
        }
    };

    println!("The field of ParseObj is {}, {}, {}, {}", &parse_obj.f1, &parse_obj.f2, &parse_obj.f3, &parse_obj.f4);
}