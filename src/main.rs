extern crate kafka;
extern crate config;
extern crate serde;

#[macro_use]
extern crate serde_derive;

mod settings;
use settings::Settings;
use std::fs;
use kafka::producer::{Producer, Record, RequiredAcks};
use kafka::error::Error as KafkaError;
use std::process;
use std::io::{self};
use uuid::Uuid;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let confpath = match args.get(1) {
        Some(p) => p.to_string(),
        None => "conf.toml".to_string()
    };
    let settings = Settings::new(confpath).unwrap();
    let filename = settings.filename();
    let broker = settings.broker();
    let topic = settings.topic();
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
    if let Err(e) = produce_message(v, &topic, vec![broker.to_owned()]) {
        println!("Failed producing messages: {}", e);
    }
    println!("END\n");
}

fn produce_message<'a, 'b>(
    datalist: Vec<&str>,
    topic: &'b str,
    brokers: Vec<String>,
    ) -> Result<(), KafkaError> {

    let mut producer = 
        Producer::from_hosts(brokers.clone())
             .with_required_acks(RequiredAcks::One)
             .create()?;
    let mut i = 1;
    for x in &datalist {
    let my_uuid = Uuid::new_v4();
    println!("{}: publish a message key: {} at {:?} to: {}",i,my_uuid, brokers.clone(), &topic);
    producer.send(&Record {
        topic: topic,
        partition: -1,
        key: (my_uuid.to_string()),
        value: x.as_bytes(),
      })?;
      i+=1;
    }
    Ok(())
}