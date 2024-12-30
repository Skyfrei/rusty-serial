use std::env;
use serialport::{DataBits, FlowControl, StopBits};

use crate::bridge;
use crate::bridge::SerialCommunicationSettings;


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
    let mut data_bits: DataBits = DataBits::Eight;
    let mut stop_bits: StopBits = StopBits::One;
    let mut flow_control: FlowControl = FlowControl::None;

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
        else if args.iter().nth(i).unwrap() == "-d"{
            if i + 1 >= args.len() - 1{
                let next_arg: u32 = args.iter().nth(i + 1).unwrap().parse().expect("Incorrect data bit");
                match next_arg{
                    5 => data_bits = DataBits::Five,
                    6 => data_bits = DataBits::Six,
                    7 => data_bits = DataBits::Seven,
                    8 => data_bits = DataBits::Eight,
                    _ => println!("Wrong Data bits, defaulting to 8")
                }
            }        
        }
        else if args.iter().nth(i).unwrap() == "-s"{
            if i + 1 >= args.len() - 1{
                let next_arg: u32 = args.iter().nth(i + 1).unwrap().parse().expect("Incorrect stop bit");
                match next_arg{
                    1 => stop_bits = StopBits::One,
                    2 => stop_bits = StopBits::Two,
                    _ => println!("Wrong Stop Bits, defaulting to 1")
                }
            }
        }
        else if args.iter().nth(i).unwrap() == "-f"{
            if i + 1 >= args.len() - 1{
                let next_arg: u32 = args.iter().nth(i + 1).unwrap().parse().expect("Incorrect flow control");
                match next_arg{
                    1 => flow_control = FlowControl::Software,
                    2 => flow_control = FlowControl::Hardware,
                    _ => flow_control = FlowControl::None
                }
            }  
        }
    }
    let serial_settings = SerialCommunicationSettings
    {
        comm_speed: baudrate,
        data_bits: data_bits,
        stop_bits: stop_bits,
        flow_control: flow_control,
    };
    call_program(serial_settings);
}

fn call_program(serial_settings: SerialCommunicationSettings){
    bridge::comm_with_serial_port(serial_settings);
}
