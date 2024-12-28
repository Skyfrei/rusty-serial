use std::env;

pub fn get_arguments() -> Vec<String>{
    let args: Vec<String> = env::args().collect();
    return args;
}
