
extern crate config;
extern crate serde;

#[macro_use]
extern crate serde_derive;
mod sender;
use sender::*;
mod settings;
use settings::Settings;
use std::fs;
use std::process;
use std::io::{self};
use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    let function = match args.get(1) {
        Some(p) => p.to_string(),
        None => {println!("You need set first argiment as ./kfsend <function> <config_path>
        where <function> may be <amqp> or <kafka>\nExample: ./kfsend kafka myconf.yml");
        process::exit(0x0100)},
    };
    let confpath = match args.get(2) {
        Some(p) => p.to_string(),
        None => "conf.toml".to_string()
    };
    let workmethod=SenderFunction::validate(&function);
    println!("Driver type: {:#?}", &workmethod);
    let settings = Settings::new(confpath).unwrap();
    let filename = settings.filename();
    let terminator = settings.terminator();
    let contents = fs::read_to_string(&filename)
        .expect("fileconf.filename wrong reading the file");
    let v: Vec<&str> = contents.split_terminator(&terminator).collect();
    println!("Has {} messages. Send it?(Y/N)", &v.len());
    let mut input = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut input).expect("error input");
    match input.trim().as_ref() {
        "Y" => println!("START"),
        _ => process::exit(0x0100),
    }

    workmethod.send_message(&settings,v);

    println!("END\n");
}

