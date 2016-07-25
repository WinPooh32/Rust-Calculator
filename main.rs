pub mod stack;
pub mod calc;

use std::env;

fn main() {
    let mut args_str = String::new();
    let mut is_silent = false;

    for argument in env::args().skip(1) {
        match argument.as_ref(){
            "-s" | "--silent" => is_silent = true,
            _ => args_str = args_str + " " + &argument
        }
    }

    if(is_silent){
        print!("{}", calc::calc(&args_str).unwrap());
    }else{
        println!("Result = {}",  calc::calc(&args_str).unwrap());
    }
}
