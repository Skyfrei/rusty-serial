use serialport;
use serialport::DataBits;
use serialport::FlowControl;
use serialport::SerialPort;
use serialport::SerialPortInfo;
use serialport::StopBits;
use std::io;
use std::process;
use std::time::Duration;
use std::u8;

pub struct SerialCommunicationSettings{
    pub comm_speed: u32,
    pub data_bits: DataBits,
    pub stop_bits: StopBits,
    pub flow_control: FlowControl
}

pub fn comm_with_serial_port(serial_settings: SerialCommunicationSettings){
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
        let mut connected_port = open_port(&port_list[buffer_to_int].port_name, serial_settings);
        loop {    
            write_serial(&mut connected_port);
            read_serial(&mut connected_port);

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

fn open_port(port_name: &String, settings: SerialCommunicationSettings) -> Box<dyn SerialPort>{
    let mut port = serialport::new(port_name, settings.comm_speed)
        .data_bits(settings.data_bits)
        .stop_bits(settings.stop_bits)
        .flow_control(settings.flow_control)
        .timeout(Duration::from_millis(3000))
        .open().expect("Failed to open port");

//    port.write_data_terminal_ready(true).unwrap();
//    port.write_request_to_send(true).unwrap();

    return port;
}

fn read_serial(port: &mut Box<dyn SerialPort>){
    let mut buffer: Vec<u8> = vec![0; 10000];
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


    let write_buffer = format!("{}\r\n", buffer_trimmed).into_bytes();
    match port.write_all(&write_buffer) {
        Ok(_) => {
            match port.flush() {
                Ok(_) => println!("Message sent successfully"),
                Err(e) => println!("Failed to flush port: {}", e)
            }
        },
        Err(e) => println!("Failed to write to port: {}", e)
    }
}

fn close_port(port: &mut Box<dyn SerialPort>){
    let _ = std::mem::drop(port);
}

fn kill_program(err_code: i32){
    process::exit(err_code);
}

