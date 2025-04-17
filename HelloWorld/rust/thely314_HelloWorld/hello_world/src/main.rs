// This program prints "Hello, world!" to the console.
// It accepts an optional argument "figlet".
// If the argument is provided, it prints "Hello, world!" in figlet style.
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => println!("Hello, world!"),
        2 => {
            if args[1] == "figlet" {
                println!(r"  _            _  _                                  _      _  _ 
 | |__    ___ | || |  ___     __      __ ___   _ __ | |  __| || |
 | '_ \  / _ \| || | / _ \    \ \ /\ / // _ \ | '__|| | / _` || |
 | | | ||  __/| || || (_) |_   \ V  V /| (_) || |   | || (_| ||_|
 |_| |_| \___||_||_| \___/( )   \_/\_/  \___/ |_|   |_| \__,_|(_)
                          |/                                     ");
            } else {
                println!("Error: unknown argument\n\
                Usage: hello_world [figlet]\n\
                Options:  figlet: print hello world in figlet style");
            }
        }
        _ => println!("Error: too many arguments\n\
        Usage: hello_world [figlet]\n\
          figlet: print hello world in figlet style"),
    }
}
