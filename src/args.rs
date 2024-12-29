use std::env;


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
    let auth = Author{
        email: String::from("Klavio Tarka <tarkaklavio@gmail.com>"),
        name: String::from("Rusty-serial"),
        version: 1.0
    };
    for arg in args{
        if arg == "-h"{
            println!("\nName: {}\nAuthor: {}\nVersion: {:.1}\n", auth.name, auth.email, auth.version);
            std::process::exit(0);
        }
    }

}
