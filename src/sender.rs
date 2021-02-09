extern crate kafka;
extern crate amqp;
use amqp::{Session,Basic};
use amqp::protocol;
use kafka::producer::{Producer, Record, RequiredAcks};
use kafka::error::Error as KafkaError;
use uuid::Uuid;
use std::process;
use crate::Settings;


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
                amqp_message(datalist,&conf.url(),&conf.queue())
                
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

fn amqp_message(datalist: Vec<&str>,url: &str,queue: &str) {
    let mut session = Session::open_url(url).unwrap();
    let mut channel = session.open_channel(1).unwrap();
    let mut i = 1;
    for x in &datalist {
        println!("{}: publish a message to: {}",i,&queue);
    channel.basic_publish("", &queue, true, false,
    protocol::basic::BasicProperties{ 
        content_type: Some("text".to_string()), 
        ..Default::default()}, 
        (x.as_bytes()).to_vec()).unwrap();
        i+=1;
    }
}