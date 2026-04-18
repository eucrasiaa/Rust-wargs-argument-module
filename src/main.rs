use std::{env, path::PathBuf};
mod wargs; // Declare the wargs module
use wargs::{ProgramArgs, DefinedArg};


fn main() {
    let passed_args: Vec<String> = env::args().collect();
    // dbg!(passed_args);
    let mut args = ProgramArgs::new();

    args <<= DefinedArg::new().short('v').long("verbose").num_args(0).help("verbose output!");

    args <<= DefinedArg::new()
        .short('o')
        .long("output")
        .num_args(1);
    args <<= DefinedArg::new()
        .short('i')
        .long("input")
        .num_args(2);
    args.define_new(DefinedArg::new().short('b').long("bonusargdemo").num_args(1).help("silly example for demo")).unwrap();
    println!("{}",args);
    println!("{}",args.generate_man());
    args.parse(&passed_args).unwrap();  //lol
    println!("args version 1:");

    if args.is_set("verbose") { 
        println!("verboose mode activated"); 
    }
    let default_output_path = vec![String::from("./output.txt")];
    let output = args.get("output").unwrap_or(&default_output_path);
    println!("{output:?}");
    let inpt = args.get("input").unwrap_or(&default_output_path);
    println!("{inpt:?}");
    if args.is_set("values"){ //unused and also undefined arg. will not fail
        println!("yes!");
    }
    else{
        println!("values not set");
    }

    // println!("args version 2:");
    // let mut args2 = (ProgramArgs::new()
    //     << DefinedArg::new().long("verboose").short('v').help("verboose output")
    //     << DefinedArg::new().long("output").short('o').help("output file").num_args(1)
    //     << DefinedArg::new().long("version")).unwrap();
    // args2.parse(&passed_args).unwrap();
    //
    // if args2.is_set("verbose") { 
    //     println!("verboose mode activated"); 
    // }
    // let output = args2.get("output").unwrap_or(&default_output_path); 
    // println!("{output}");



    // println!("{}",args2.unwrap());
}
