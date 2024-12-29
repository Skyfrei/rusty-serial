use std::env;
use crate::bridge;


struct Author{
    email: String,
    name: String,
    version: f32,
}

pub fn get_arguments(){
    let args: Vec<String> = env::args().collect();
    let mut real_args: Vec<String> = Vec::<String>::new();
    if args.len() > 1{
       for index in 1..args.len(){
            real_args.push(args.iter().nth(index).unwrap().to_string());
        }
    }

    parse_arguments(real_args);
}

fn parse_arguments(args: Vec<String>){
    let mut baudrate: u32 = 115200;
    let auth = Author{
        email: String::from("Klavio Tarka <tarkaklavio@gmail.com>"),
        name: String::from("Rusty-serial"),
        version: 1.0
    };

    for i in 0..args.len(){
        if args.iter().nth(i).unwrap() == "-h"{
            println!("\nName: {}\nAuthor: {}\nVersion: {:.1}\n", auth.name, auth.email, auth.version);
            std::process::exit(0);
        }
        else if args.iter().nth(i).unwrap() == "-b"{
            if i + 1 >= args.len() - 1{
                let next_arg: u32 = args.iter().nth(i + 1).unwrap().parse().expect("Incorrect baudrate");
                if next_arg <= 1000000{
                    baudrate = next_arg;
                }
            }        
        }
    }
    call_program(baudrate);
}

fn call_program(baudrate: u32){
    bridge::comm_with_serial_port(baudrate);
}
