extern crate dummy_cli_parser;

use std::net::{IpAddr};
use dummy_cli_parser::{CliParser, PatternType};

fn main() {
    struct ParseObj {
        addr: IpAddr,
        port: i32,
    }

    let mut parser = CliParser::new(ParseObj{
        addr: "127.0.0.1".parse::<IpAddr>().unwrap(),
        port: 1024,
    });

    parser.register_pattern("-ip", PatternType::OptionalWithArg, "ip address", 
        |arg_str, parse_obj| {
            arg_str.parse::<IpAddr>().map(|addr|{
                parse_obj.addr = addr;
            }).map_err(|_|{
                String::from(format!("fail to parse argument \"{}\"", &arg_str))
            })
        }
    ).unwrap();

    parser.register_pattern("-p", PatternType::OptionalWithArg, "port", 
        |arg_str, parse_obj| {
            let parse_res = arg_str.parse::<i32>();
            if parse_res.is_ok() {
                let port = parse_res.unwrap();
                if port >=0 && port <= 65535 {
                    parse_obj.port = port;
                    Ok(())
                }
                else {
                    Err(String::from(format!("port number {} is invalid", &port)))
                }
            }
            else {
                Err(String::from(format!("fail to parse argument \"{}\"", &arg_str)))
            }
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

    println!("The network address is {}:{}", &parse_obj.addr, &parse_obj.port);
}