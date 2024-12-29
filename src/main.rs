mod bridge;
mod args;

fn main() {
    println!("Hello, world!");
    args::get_arguments();
    bridge::comm_with_serial_port();
}
