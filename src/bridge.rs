use serialport;
use serialport::DataBits;
use serialport::SerialPort;
use serialport::SerialPortInfo;
use serialport::StopBits;
use std::io;
use std::process;
use std::time::Duration;
use std::u8;

struct SerialCommunicationSettings{
    comm_speed: u32
}

pub fn comm_with_serial_port(baudrate: u32){
    let port_list = list_all_ports();

    println!();
    println!("Please select the port you want to communicate with");

    let mut buffer = String::new();
    let _ = io::stdin().read_line(&mut buffer);
    let buffer_to_int = buffer.trim().parse::<usize>().unwrap();

    if buffer_to_int >= port_list.len() || buffer_to_int < usize::try_from(0).unwrap() {
        println!("=====================================================");
        println!("Incorrect port. Try running the program again!");    
    }
    else{
        let mut connected_port = open_port(&port_list[buffer_to_int].port_name, baudrate);
        loop {    
            read_serial(&mut connected_port);
            write_serial(&mut connected_port);
        }
    }
}

fn list_all_ports() -> Vec<SerialPortInfo>{
    let ports = serialport::available_ports().expect("No ports found!");
    let mut iterator = 0;
    for p in &ports{
        println!("{}. {}", iterator, p.port_name);
        iterator += 1;
    }
    return ports;
}

fn open_port(port_name: &String, baudrate: u32) -> Box<dyn SerialPort>{
    let serial_settings = SerialCommunicationSettings { comm_speed: baudrate };
    let mut port = serialport::new(port_name, serial_settings.comm_speed)
        .data_bits(DataBits::Eight)
        .stop_bits(StopBits::One)
        .flow_control(serialport::FlowControl::Software)
        .timeout(Duration::from_millis(1000))
        .open().expect("Failed to open port");

    return port;
}

fn read_serial(port: &mut Box<dyn SerialPort>){
    let buffer_size: u32 = port.bytes_to_read().expect("Failed to read the port bytes");
    let mut buffer: Vec<u8> = vec![0; 1000];
    let res = port.read(buffer.as_mut_slice());
    check_for_errors(res);   
    let mut ascii_bufffer: Vec<char> = Vec::new();

    for c in buffer{
        let temp_c: char = c as char;
        ascii_bufffer.push(temp_c);
        print!("{temp_c}");
    }
    print!("=============================================================================================================");
}

fn check_for_errors(res: Result<usize, io::Error>){
    match res{
        Ok(_v) => print!(""),
        Err(e) => println!("Error: {}\n", e),
    }
}

fn write_serial(port: &mut Box<dyn SerialPort>) {
    println!("\nWrite the serial message!\n");
    let mut buffer = String::new();
    let _ = io::stdin().read_line(&mut buffer);
    let buffer_trimmed = buffer.trim();

    if buffer_trimmed == ""{
        return;
    }
    else if buffer_trimmed == "exit"{
        close_port(port);
        kill_program(0);
    }


    let write_buffer = buffer.as_bytes();
    let res = port.write(write_buffer);
    let _ = port.flush();
    check_for_errors(res);
}

fn close_port(port: &mut Box<dyn SerialPort>){
    let _ = std::mem::drop(port);
}

fn kill_program(err_code: i32){
    process::exit(err_code);
}

