mod bridge;
mod args;

fn main() {
    println!("Hello, world!");
    bridge::comm_with_serial_port();
}
