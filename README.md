a barebones, slightly sloppy Rust argument handling module.
by Will Capitos
## usage:
1. declare a new argument options
2. push args
3. run parse
4. use .is-set(long flag name) to see if a flag was passed
5. unwrap .get(long flag name) to pull a vec of the args passed.


## example
``` rust 

// run with -v -i "input1.txt" "input2.txt"
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


args.parse(&passed_args).unwrap();
// check if verboose?
if args.is_set("verbose") { 
    println!("verboose mode activated"); 
}
// define a fallback default
let default_output_path = vec![String::from("./output.txt")];
// check passed output. will be none, so goes to fallback
let output = args.get("output").unwrap_or(&default_output_path);
println!("{output:?}");

// check passed inputs. will be "input1.txt" "input2.txt"
let inpt = args.get("input").unwrap_or(&default_output_path);
println!("{inpt:?}");


//Results:
//- verboose mode activated
//- ["./output.txt"]
//- ["input1.txt", "input2.txt"]
values not set
```

upcoming TODOs:
support chained short flags like `-zxf <arg_f>`
support stacking flags like `-vvv`
