//! `machine.rs`
//!
//! This module contains not thread 
//! safe Turing machine implementation 
//!
//! Status: Unfinished | Unrefactored | Unreleased 

#![allow(dead_code)]

use tape::{
    Tape, 
    TapeBasics, 
    Link
};

use std::sync::Arc;
use std::cell::RefCell;

use std::io::{
    self,
    Write, 
    stdout
};

use std::sync::mpsc::{
    self,
    Sender,
    Receiver
};

use std::path::Path;
use std::fs::File;
use std::thread;

/// Simulate Turing's machine as it is 
#[derive(Clone)]
pub struct TuringMachine<'a> {
    pub    state: &'a str,
    pub     halt: &'a str,
    pub    rules: Arc<RefCell<Tape<String>>>,
    pub     tape: Arc<RefCell<Tape<String>>>,
    pub    blank: &'a str,
    pub position: Arc<RefCell<i32>>,
    pub   symbol: &'a str,
}


/// Global options for all machines
#[derive(Debug)]
pub struct Preferences {
    pub  datadir: String,
    pub colorize: bool
}


/// Function        prepare_dataset
/// Purpose         Looks for the data inside data/ directory 
///                 and prepares it for the computations
///
/// return          Result<Vec<String>>
pub fn prepare_dataset(datadir: &str) -> io::Result<Vec<String>> {
    /// Oh god 
    /*let pwd_output = Command::new("pwd")
        .output().unwrap_or_else(|e| {
            panic!("failed to run \'pwd\': {}", e)
        });

    if !pwd_output.status.success() {
        panic!("pwd failed");
    }

    let files_output = Command::new(&format!("ls -p \'{}/\' | grep -v /", 
                                             String::from_utf8_lossy(&pwd_output.stdout)))
        .output().unwrap_or_else(|e| {
            panic!("failed to execute process - ls -p: {}", e)
        });

    if files_output.status.success() {
        let out_string = String::from_utf8_lossy(&files_output.stdout);
        let files: Vec<Vec<&str>> = out_string
            .lines()
            .map(|ref s| s.split(" ")
                 .filter(|ref s| !s.is_empty())
                                   .collect::<Vec<_>>())
                 .collect::<Vec<_>>();
    }*/
    
    /*let ls = Command::new("ls")
        .arg(&format!("-p \'{}\'", datapath))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap_or_else(|why| panic!("could not spawn ls -p: {:?}", why));
    
    ls.wait();

    let buff = String::new();
    if let Some(ref mut stdout) = ls.stdout {
        BufReader::new(stdout)
            .lines()
            .map(|line| buff.push_str(&line.unwrap()));
    }

    let files_process = Command::new("grep")
        .arg("-v /")
        .stdin(ls.stdout.unwrap())
        .unwrap_or_else(|why| panic!("could not spawn ls -p: {:?}", why));

    let mut s = String::new();
    match files_process.stdout.unwrap().read_to_string(&mut s) {
        Err(why) => panic!("could not read ls -p stdout: {:?}", why),
        Ok(_) => println!("[?] files_output: {:?}", s),
    }*/

    let datapath = format!("{}/{}/", ::std::env::current_dir()?.display(), datadir);
    println!("[?] datapath: {}", datapath);
    stdout().flush().unwrap();

    /// lifetimes
    /*
      let names = std::fs::read_dir(datapath)
        .unwrap()
        .map(|entry| {
            entry.ok().and_then(|k|
                k.path()
                 .as_path()
                 .to_owned()
                 .to_str()
                 .unwrap()
                 .to_string())
        })
        .collect::<Vec<String>>();
    */
    
    let filepaths = ::std::fs::read_dir(datapath)?;

    /// I am proud of my country
    let names = 
        filepaths.map(|entry| {
            let entry = entry.unwrap().path();
            let  p1ud = entry.as_path().to_owned();
            
            if p1ud.is_file() {
                let  io1f = p1ud.to_str().unwrap();
                let f38ds = String::from(io1f);
                f38ds
            
            } else { String::new() }
    
        }).filter(|ref s| !s.is_empty())
          .collect::<Vec<String>>();

    Ok(names)
}


/// Function        parse_data
/// Purpose         Parses the given data to <Tape<String>> 
///                 and makes it readable for the machine  
///
/// data            The given data you want to parse
///
/// return          Option<Arc<RefCell<Tape<String>>>>
pub fn parse_data(data: Vec<&str>) -> Link<Tape<String>> {
    let   data = data.join("\n");
    let  lines = data.lines();
    let vector = lines.map(|ref w| w.split(" ")
                                    .filter(|ref s| !s.is_empty())
                                    .collect::<Vec<_>>())
                      .collect::<Vec<_>>();

    //let ref mut tape_clone = *&vector[0].clone().join("").to_string();
    //let processed = tape_clone.drain(..(tape_clone.find(']').unwrap_or(tape_clone.len()))).collect();

    let tape = Arc::new(RefCell::new(Tape::new()));
    for sym in &vector[0] { 
        tape.borrow_mut().push_back(sym.to_string());
    }

    Some(tape)
}


/// Function        compute
/// Purpose         Make the Turing machine work 
///
/// machine         The given machine with all data inside
///
/// return          String
pub fn compute(machine: TuringMachine) -> String {
    /*
    if let Some(t) = machine.tape.borrow().unwrap().peek_front() { 
        machine.tape.borrow_mut().unwrap().push_back(String::from(machine.blank));
    };
    
    while let Ok(tape) = Arc::try_unwrap(machine.tape) {

        let mut tape = tape.into_inner().ok().unwrap();
        let mut tape = tape.into_inner(); 
            {
                let mut local_tape = tape.clone();

                match tape.pop_front() {
                    Some(_) => { },
                    None    => { break },
                }

                let iter = tape.clone().into_iter(); 
                
                let mut   position = Arc::try_unwrap(machine.position)
                    .ok()
                    .unwrap()
                    .into_inner()
                    .unwrap();

                for element in iter.clone() {
                        print!("{} ", element);
                }

                if machine.state == machine.halt { break }
                if machine.state "where state and tape are not in rules exception"

                 better implement reborrow_mut() function
            
                tape.peek_front();

                let dr ="left";
                match dr {
                     "left" => tape.move_left(),
                     "right" => tape.move_right(),
                           _ => { },
                }
                    tape = local_tape.clone();
                    tape = local_tape.clone();
                
            }
        
        println!();

        break;
    }
   */ 
    Arc::try_unwrap(machine.tape).unwrap();
    String::from("[unreleased]")
}

/// Function    run
/// Purpose     Machine in action, reads data/ directory 
///             and computes everything it can 
///             and then puts its output to the data/output/ directory
///
/// return      ()
pub fn run(opts: Preferences) {
    println!("[\x1B[1;32m+\x1B[0m] Machine is: \x1B[1;32mLoaded\x1B[0m");

    let (tx, rx): (Sender<String>, 
                 Receiver<String>) = mpsc::channel();
    
    let dataset: Vec<String> = prepare_dataset(&opts.datadir)
        .map_err(|err| err.to_string()).unwrap();
        //.and_then(|data| data);

    /*
    let dataset: Vec<String> = match prepare_dataset() { 
        Ok(data) => data,
        Err(_) => { panic!("can't read dataset"); },
    };
    */
    
    {
        println!("[?]  dataset: {}", dataset[0]);
        for datafile in &dataset[1..] {
            for _ in 0.."[?]  dataset: ".len() { print!(" "); }
            print!("{}\n", datafile);
        }
    }

    //let dataset_length = dataset.len();
    let mut processed: Vec<File> = Vec::with_capacity(dataset.len());

    for datafile in &dataset {
        let ttx: Sender<String>  = tx.clone();

        let filepath: &Path = Path::new(&datafile);

        /*
        let mut file: Box<io::Read> = File::open(&filepath)
            .map_err(|err| err.to_string())
            .and_then(|file| Box::new(file));
        */

        let mut file: Box<io::Read> = match File::open(&filepath) {
            Ok(file) => Box::new(file),
            Err(why) => panic!("can't open the file {}: {}", filepath.display(), why),
        };
        

        /*
        let mut content: String = String::new()
            // oh god
            .and_then(|string| { 
                file.read_to_string(&mut string)
                    .expect(&format!("cannot read file {}", file)) 
            }); 
        */

        let mut content: String = String::new();
        file.read_to_string(&mut content).expect(&format!("cannot read file {}", "file"));

        if content.is_empty() { continue };


        let thread_name: String = datafile.clone();

        /// Make a thread for a specific file and compute data inside it
        thread::Builder::new().name((&thread_name).to_string()).spawn(move || {
            {
                println!("\n\n\n[\x1B[1;32m+\x1B[0m] Thread started for \x1B[2m{}\x1B[0m", &thread_name);
                println!("\n[\x1B[1;33m*\x1B[0m] Given data:");
          
                for line in content.lines() { println!("[-]\t{}", line) }
            }

            let rules: Vec<&str> = content.split("\n")
                .filter(|ref s| !s.is_empty())
                .collect(); 
            
            {
                println!("\n[\x1B[1;33m*\x1B[0m] Parsed rules:");
                println!("[t]\t{}", &rules[0]);
                println!("[s]\t{}", &rules[1]);

                for rule in &rules[2..] { println!("[r]\t{}", rule); }
            
                println!("[\x1B[1;33m?\x1B[0m] t − tape | s − state blank | r − rule");
            }

            /// TODO: unwrap
            let  tape = parse_data(vec!(rules[0])).unwrap();
            
            let  state: char = rules[1].to_string().pop().unwrap_or(' ');
            let  blank: char = rules[1].to_string().pop().unwrap_or(' ');

            /*let symbol = &rules
                .last()
                .unwrap()
                .char_indices()
                .rev()
                .map(|(i, _)| i)
                .nth(0)
                .unwrap();
            */

            /*let halt = Some(&rules
                .last()
                .unwrap()
                .split(" ")
                .collect::<Vec<&str>>())
                .map(|rule| rule
                     .last()
                     .unwrap());
            */

            let rules_b627: &Vec<&str> = &rules
                .last()
                .unwrap()
                .split(" ")
                .collect();
            
            /// TODO: unwrap()
            let halt: &&str = rules_b627.last().unwrap();

            /// TODO: unwrap()
            let rules = parse_data(rules[2..].to_vec()).unwrap();

            let /*mut*/ machine: TuringMachine = TuringMachine { 
                   state: &state.to_string(),
                    halt: halt, //"halt",
                   rules: rules,
                    tape: tape,
                   blank: &blank.to_string(),
                position: Arc::new(RefCell::new(0)),
                  symbol: "",
            };

            let result_data: &str = &compute(machine);
            //let result_data: &str = "carpe diem";

            thread::sleep(::std::time::Duration::from_millis(63)); // looks pretty dynamic 
            ttx.send(result_data.to_string()).unwrap();
        }).unwrap();
     
        /// Recieve and write computed data to the files
        {
            /// TODO: get rid of this crap
            let mut datamut: String = datafile.clone();
            let        bump: &usize = &datamut.rfind('/').unwrap_or(datamut.len());
            let    filename: String = datamut.drain(*bump..).collect();
            let output_path: String = format!("{}/{}/output{}", 
                                      ::std::env::current_dir()
                                                 .unwrap()
                                                 .display(), 
                                      opts.datadir.to_string(),
                                      &filename);
            //let path = Path::new(bump); 
            if let Ok(data) = rx.recv() {

                /*
                File::create(&output_path)
                     .map_err(|err| err.to_string())
                     .and_then(|mut file| { file.write_all(data.as_bytes()) })
                     .and_then(|    file| { processed.push(file) });
                */

                /// TODO: unwrap()
                let mut result_file: File = match File::create(&output_path) {
                    Err(why) => panic!("could not create {}: {:?}",
                                    output_path, why),
                    Ok(file) => file, 
                };

                /// TODO: unwrap()
                result_file.write_all(data.as_bytes()).unwrap();
                
                processed.push(result_file);
            }
        }        
    }

    println!("\n\n[\x1B[1;33m@\x1B[0m] Processed files:");
    for test in &processed { println!("[-] {:?}", test); }
}
