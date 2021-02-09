
use config::{ConfigError, Config, File};

#[derive(Debug, Deserialize)]
struct KafkaConf {
    broker: String,
    topic: String,
}
#[derive(Debug, Deserialize)]
struct AMQPConf {
    url: String,
    queue: String,
}
#[derive(Debug, Deserialize)]
struct FileConf {
    filename: String,
    terminator: String,
}
#[derive(Debug, Deserialize)]
pub struct Settings {
    fileconf: FileConf,
    kafkakonf: KafkaConf,
    amqpconf: AMQPConf,
}

impl Settings {
    pub fn new(filepath: String) -> Result<Self, ConfigError> {
        println!("Load config: {}",&filepath);
        let mut s = Config::new();
        s.merge(File::with_name(&filepath)).expect("Wrong reading config file");
        s.try_into()
    }
    pub fn filename(&self) -> String {
        self.fileconf.filename.to_string()
    }
    pub fn broker(&self) -> String {
        self.kafkakonf.broker.to_string()
    }
    pub fn topic(&self) -> String {
        self.kafkakonf.topic.to_string()
    }
    pub fn terminator(&self) -> String {
        self.fileconf.terminator.to_string()
    }
    pub fn url(&self) -> String {
        self.amqpconf.url.to_string()
    }
    pub fn queue(&self) -> String {
        self.amqpconf.queue.to_string()
    }
}
