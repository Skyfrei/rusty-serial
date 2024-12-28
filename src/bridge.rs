use serialport;
use serialport::SerialPort;
use serialport::SerialPortInfo;
use std::io;
use std::time::Duration;
use crate::args;

struct SerialCommunicationSettings{
    comm_speed: u32
}

pub fn comm_with_serial_port(){
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
        let mut connected_port = open_port(&port_list[buffer_to_int].port_name);
        loop {    
            read_serial(&mut connected_port);
            //write_serial(&mut connected_port);

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

fn open_port(port_name: &String) -> Box<dyn SerialPort>{
    // Use this to set the comm speed
    let _ = args::get_arguments();

    let serial_settings = SerialCommunicationSettings { comm_speed: 115200 };
    let port = serialport::new(port_name, serial_settings.comm_speed)
        .timeout(Duration::from_millis(10))
        .open().expect("Failed to open port");

    return port;
}


fn read_serial(port: &mut Box<dyn SerialPort>){
    print!("{}", port.bytes_to_read().expect("Error reading bytes!"));

}

fn write_serial(port: &mut Box<dyn SerialPort>){

    let mut buffer = String::new();
    let _ = io::stdin().read_line(&mut buffer);
    port.write(buffer.as_bytes()).expect("Write failed!");
}

fn write_byte_serial(port: &mut Box<dyn SerialPort>, file_path: &str){

}

fn close_port(port: Box<dyn SerialPort>){
    let _ = std::mem::drop(port);
}
