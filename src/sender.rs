extern crate kafka;
extern crate amqp;
use amqp::{Session,Basic,AMQPError};
use amqp::protocol;
use kafka::producer::{Producer, Record, RequiredAcks};
use kafka::error::Error as KafkaError;
use uuid::Uuid;
use std::process;
use crate::Settings;
use std::{thread, time};

#[derive(Debug)]
pub enum SenderFunction {
    KafkaSender,
    AMQPSender,
}

impl SenderFunction {
    pub fn validate(function: &str) -> SenderFunction {
        match function {
            "kafka" => SenderFunction::KafkaSender,
            "amqp" => SenderFunction::AMQPSender,
            a => {
                println!("Unknown function type: {}",a);
                process::exit(0x0100)
            },
        }
    }
    pub fn send_message(
        &self,
        conf: &Settings,
        datalist: Vec<&str>) {
        match *self {
            SenderFunction::KafkaSender => {
                println!("Message will be send to kafka");
                if let Err(e) = kafka_message(datalist,&conf.topic(),vec![conf.broker()])
                {
                    println!("Failed producing messages: {}", e);
                }
            },
            SenderFunction::AMQPSender => {
                println!("Message will be send to AMQP"); 
                if let Err(e) = amqp_message(datalist,&conf.url(),&conf.queue())
                {
                    println!("Failed producing messages: {:?}", e);
                }
                
            },
        }
    }
}


fn kafka_message<'a, 'b>(
    datalist: Vec<&str>,
    topic: &'b str,
    brokers: Vec<String>,
    ) -> Result<(), KafkaError> {

    let mut producer = 
        Producer::from_hosts(brokers.clone())
             .with_required_acks(RequiredAcks::One)
             .create()?;
    for (i,x) in datalist.iter().enumerate() {
    let my_uuid = Uuid::new_v4();
    println!("{}: publish a message key: {} at {:?} to: {}",i+1,my_uuid, brokers.clone(), &topic);
    producer.send(&Record {
        topic: topic,
        partition: -1,
        key: (my_uuid.to_string()),
        value: x.as_bytes(),
      })?;
    }
    Ok(())
}

fn amqp_message(datalist: Vec<&str>,url: &str,queue: &str)-> Result<(), AMQPError>  {
    let mut session = Session::open_url(url)?;
    let mut channel = session.open_channel(1)?;
    for (i,x) in datalist.iter().enumerate() {
        println!("{}: publish a message to: {}",i+1,&queue);
    channel.basic_publish("", &queue, true, false,
    protocol::basic::BasicProperties{ 
        content_type: Some("text".to_string()), 
        ..Default::default()}, 
        (x.as_bytes()).to_vec())?;
    }
    thread::sleep(time::Duration::from_secs(60));
    channel.close(200, "Bye")?;
    session.close(200, "Good Bye");
    Ok(())
}