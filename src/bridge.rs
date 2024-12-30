use serialport;
use serialport::SerialPort;
use serialport::SerialPortInfo;
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
        println!("");
        let mut connected_port = open_port(&port_list[buffer_to_int].port_name, baudrate);
        loop {    
            read_serial(&mut connected_port);
            write_serial(&mut connected_port);
        }
    }
}

fn list_all_ports() -> Vec<SerialPortInfo>{
    let ports = serialport::available_ports().expect("No ports found!");
    let mut iterator = 1;
    for p in &ports{
        println!("{}. {}", iterator, p.port_name);
        iterator += 1;
    }
    return ports;
}

fn open_port(port_name: &String, baudrate: u32) -> Box<dyn SerialPort>{
    // Use this to set the comm speed
    
    let serial_settings = SerialCommunicationSettings { comm_speed: baudrate };
    let port = serialport::new(port_name, serial_settings.comm_speed)
        .timeout(Duration::from_millis(1000))
        .open().expect("Failed to open port");

    return port;
}

fn read_serial(port: &mut Box<dyn SerialPort>){

    let buffer_size: u32 = port.bytes_to_read().expect("Failed to read the port bytes");
    let mut buffer: Vec<u8> = Vec::new();
    let res = port.read(&mut buffer);
    check_for_errors(res);

    print!("There is {} bytes to read.\nBytes: {:?} \n", buffer_size, buffer);
}

fn check_for_errors(res: Result<usize, io::Error>){
    match res{
        Ok(_v) => print!(""),
        Err(e) => println!("{}\n", e),
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

    let _ = port.flush();
    let write_buffer = buffer.as_bytes();
    let res = port.write(&write_buffer);
    check_for_errors(res);

}

fn close_port(port: &mut Box<dyn SerialPort>){
    let _ = std::mem::drop(port);
}

fn kill_program(err_code: i32){
    process::exit(err_code);
}

