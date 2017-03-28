//! `main.rs`
//!
//! This is the main executable module of the project
//!
//! Status: Finished | Unrefactored | Released

#![allow(dead_code)]

#[macro_use]
mod machine;
mod tape;

use machine::*;

use std::io::{
    self,
    Write,
};

use std::env::args;


#[macro_export]
macro_rules! println {
    
    ( $fmt : expr ) => 
        {{
            ( print!( concat!( $fmt, "\n" )) );
            ( io::stdout().flush().unwrap() );
        }};

    ( $fmt : expr, $( $arg : tt )* ) => 
        {{
            ( print!( concat!( $fmt, "\n" ), $( $arg )*  ));
            ( io::stdout().flush().unwrap() );
        }};

}


/// Function     usage
/// Purpose      Prints usage to stdout
///
/// return       ()
fn usage() {
    let name: String = args().nth(0).unwrap_or("utm".into());
    {
        println!("{:?} Universal Turing Machine [{}]", 
                 name, option_env!("CARGO_PKG_VERSION").unwrap_or("unknown")                   );
        println!("-h, {: <14} show this message",                         "--help"             );
        println!("-v, {: <14} show this project version",                 "--version"          );
        println!("-e, {: <14} compute files inside `examples` directory", "--examples"         );
        println!("-c, {: <14} pretty colorize the output",                "--color"            );
        println!("-d, {: <14} force to print more debug output{: >16}",   "--debug", "[DUMMY]" );  
    }
}


fn main() {
    let mut opts = Preferences {
         datadir: String::from("data"),
        colorize: false
    };

    let mut iterator = args().skip(1).peekable();

    while let Some(arg) = iterator.next() {
        if arg == "-h" || arg == "--help" {
            usage();
            return
        }

        if arg == "-v" || arg == "--version" {
            println!("{}", option_env!("CARGO_PKG_VERSION").unwrap_or("unknown"));
            return
        }

        if arg == "-e" || arg == "--examples" {
            opts.datadir = String::from("examples");
        }

        if arg == "-c" || arg == "--colorize" {
            opts.colorize = true;
        }

        if arg == "-d" || arg == "--debug" {
            println!("Later");
            return
        }
    }

    run(opts);
}
