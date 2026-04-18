//! Will's Wonderful rust argument helper library. basically for personal use, cause its not really
//! all that
use std::any::Any;
use std::env;
use std::collections::HashMap;
use std::fmt::Display;
use std::ops::{ShlAssign, Shl};

pub struct ProgramArgs{
	pub defined: Vec<DefinedArg>,
  pub matches: HashMap<String, Vec<String>>,
}

pub struct ManFileThing{
    pub name:String,
    pub synopsis:String, 
    pub description:String, 
    pub options:Vec<String>,  //fix this to mimic the nice ProgramArgs display?
    pub see_also:String,
}

impl Display for ManFileThing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let options_str = self.options.join("\n  ");
        write!(
            f,
            "NAME:\n  {}\n\nSYNOPSIS:\n  {}\n\nDESCRIPTION:\n  {}\n\nOPTIONS:\n  {}\n\nSEE ALSO:\n  {}",
            self.name, 
            self.synopsis, 
            self.description, 
            options_str, 
            self.see_also
        )
    }
}

/// Core argument structure for parsing command line inputs.
///
/// created via Builder chain `::new().long("hi").short('h')...`
///
/// This struct holds the definition for a flag your program expects.
/// 
/// long_flag is the only required value!
pub struct DefinedArg {
    /// The short flag character (e.g., 'v' for -v).
    pub short_flag: char,
    /// The long flag string (e.g., "verbose" for --verbose).
    pub long_flag: String,
    /// REQUIRED: The number of values this argument expects to follow it.
    pub num_args: usize,
    /// OFFICIALLY UNUSED AT THE MOMENT. WILL BE USED TO LABEL THE ARGS IN OUTPUT
    pub opt_arg: Vec<String>,
    /// Help text displayed when the user requests usage info.
    help_text: String,
}
/// # Creation of new arguments done via <<=
impl ShlAssign<DefinedArg> for ProgramArgs{
    fn shl_assign(&mut self, rhs: DefinedArg)  {
        self.define_new(rhs).unwrap();
    }
}


/// # Use of chaining << will require unwrapping before use. be warned 
impl Shl<DefinedArg> for ProgramArgs {
    // ret Result<Self, String> to allow chaining with '?'
    type Output = Result<Self, String>;
    /// Overwritten. Shorthand to allow chained new_arguments()
    ///
    /// ex:  
    fn shl(mut self, rhs: DefinedArg) -> Self::Output {
        self.define_new(rhs)?; // ? to fail early
        Ok(self)
    }
}
impl Display for ProgramArgs{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, item) in self.defined.iter().enumerate() {
            if i > 0 {
                write!(f, "\n ")?;
            }
            write!(f, "{}", item)?;
        }
        Ok(())
    }
}
impl Display for DefinedArg{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       // self.long_flag self.short_flag self.help_text self.opt_arg self.num_args
        //fmt flags
        let flags = format!("-{:}, --{:}", self.short_flag, self.long_flag);
        
        let mut args_display = String::new();
        for i in 0..self.num_args {
            args_display.push_str(&format!(" <arg{}>", i + 1));
        }
        // turns it to -short, --long <arg> <arg>
        writeln!(f, "{}{}", flags, args_display)?;
        for line in self.help_text.lines() {
            writeln!(f, "        {}", line.trim())?;
        }

        Ok(())
    }
}


impl ProgramArgs{
    ///Creates a new, empty set of arguments. Ideally only one exists in a program
    pub fn new() -> Self {
        Self { defined: Vec::new(), matches:HashMap::new() }
    }
    /// Registers a new argument definition.
    /// see overloaded <<= and << for other ways to do it
    ///
    /// # Errors
    /// Returns an `Err` if:
    /// * A long flag is missing.
    /// * Long flag contains spaces
    /// * Flags contain non-ASCII characters.
    /// * The flag (short or long) has already been defined.
    pub fn define_new(&mut self, new_arg:DefinedArg) -> Result<(), String> {
        // ensure at least a long flag exist, or both .but not none
        if new_arg.long_flag.is_empty() {
            return Err(
                format!("ON: [-{}, --{}]: Please define a long flag! only short flag is optional!"
                    ,new_arg.short_flag,new_arg.long_flag));
        }

        if new_arg.long_flag.contains(" "){

            return Err(
                format!("ON: [-{}, --{}]: No spaces allowed in long flag name!!!"
                    ,new_arg.short_flag,new_arg.long_flag));
        }
        // if a short flag, ensure valid
        if new_arg.short_flag != '\0' && !new_arg.short_flag.is_ascii() {
            return Err(
                format!("ON: [-{}, --{}]: Short flag must be ASCII!",
                    new_arg.short_flag,new_arg.long_flag));
        }

        // if long, ensure valid
        if !new_arg.long_flag.is_ascii() {
            return Err(
                format!("ON: [-{}, --{}]: Long flag must only be ASCII!",
                    new_arg.short_flag,new_arg.long_flag));
        }

        //mayb later        
        // if new_arg.opt_arg.len() != new_arg.num_args {
        //     return Err(
        //         format!("ON: [-{}, --{}]: Number of args defined doesn't match number defined",
        //             new_arg.short_flag,new_arg.long_flag));
        // }
        // check its not already defined in some way
        if self.defined.iter().any(|a| 
            (new_arg.short_flag != '\0' && a.short_flag == new_arg.short_flag) || 
            (!new_arg.long_flag.is_empty() && a.long_flag == new_arg.long_flag)
        ) {
            return Err(
                format!("ON: [-{}, --{}]: Flag already defined!",
                    new_arg.short_flag, new_arg.long_flag));
        }
        self.defined.push(new_arg);
        Ok(())
    }

    pub fn parse(&mut self, input: &Vec<String>) -> Result<(), String> {
        let mut iter = input.into_iter().skip(1); // as program name sits in arg 1, we skip

        while let Some(arg) = iter.next() {
            let definition = self.defined.iter().find(|d| {
                arg == &format!("-{}", d.short_flag) || arg == &format!("--{}", d.long_flag)
            }).ok_or_else(|| format!("Unknown argument: {}", arg))?;

            let mut values = Vec::new();
            // collect expected num of arguments this flag expects
            for _ in 0..definition.num_args {
                if let Some(val) = iter.next() {
                    values.push(val.clone());
                } else {
                    return Err(format!("Argument {} requires {} values", arg, definition.num_args));
                }
            }
            
            // Use the long_flag as the key for the matches map
            self.matches.insert(definition.long_flag.clone(), values);
        }
        Ok(())
    }

    /// checks if an argument was provided by long name
    pub fn is_set(&self, flag: &str) -> bool {
        self.matches.contains_key(flag)
    }
    /// access vector of arguments
    pub fn get(&self, flag: &str) -> Option<&Vec<String>> {
        self.matches.get(flag)
    }
    /// less pretty than the implemented display trait! 
    pub fn generate_man(&self) -> ManFileThing {
        let mut options_list = Vec::new();

        for arg in &self.defined {
            let entry = format!(
                "-{}, --{} : {}", 
                arg.short_flag, arg.long_flag, arg.help_text
            );
            options_list.push(entry);
        }

        ManFileThing {
            name: "myProgram".to_string(),
            synopsis: "./my_program [OPTIONS]".to_string(),
            description: "thing in rust.".to_string(),
            options: options_list,
            see_also: "willcapitos.com".to_string(),
        }
    }

    // fn check_for_def(&self, an_arg:DefinedArg){
    //     if self.defined.iter().any(|a| 
    //         (new_arg.short_flag != '\0' && a.short_flag == new_arg.short_flag) || 
    //         (!new_arg.long_flag.is_empty() && a.long_flag == new_arg.long_flag)
    //     ) {
    //         return Err("Flag already defined!".to_string());
    //     }
    // }
}


impl Shl<DefinedArg> for Result<ProgramArgs, String> {
    type Output = Self;

    fn shl(self, rhs: DefinedArg) -> Self::Output {
        // If 'self' is Ok, call define_new. If it's Err, just pass the Err along.
        let mut program_args = self?; 
        program_args.define_new(rhs)?;
        Ok(program_args)
    }
}

impl DefinedArg{
    pub fn new() -> Self {
        Self {
            short_flag: '\0',
            long_flag: String::new(),
            num_args: 0,
            opt_arg: vec![],
            help_text: String::new(),
        }
    }
    pub fn short(mut self, flag: char) -> Self {
        self.short_flag = flag;
        self
    }
    pub fn long(mut self, flag: &str) -> Self {
        self.long_flag = flag.to_string();
        self
    }
    pub fn num_args(mut self, n: usize) -> Self {
        self.num_args = n;
        self
    }
    pub fn help(mut self, text: &str) -> Self {
        self.help_text = text.to_string();
        self
    }
}

// pub fn new_argument(c:char,long_flag:String, num_args:usize, )

// quick dirty args handler:
fn validateArgs(raw_input:Vec<String>) {

}
