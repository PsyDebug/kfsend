[![PsyDebug](https://circleci.com/gh/PsyDebug/kfsend.svg?style=svg)](https://circleci.com/gh/PsyDebug/kfsend)

# kfsend

**Mini tool for send separated messages list to kafka or amqp.**

## Run

You cat use one of this drivers:

* amqp
* kafka

`$ ./kfsend amqp config.toml`

## Example

We can use next config

```
[fileconf]
filename="messages.list"
terminator="#@#@#"
[kafkakonf]
broker="127.0.0.1:9092"
topic="sender-test"
[amqpconf]
url="amqp://guest:guest@localhost:5672"
queue="my-test-queue"
```

Where `filename` is path to messages file. 

For example:

`$ cat messages.list`

```
message1#@#@#message2#@#@#It's
my
multiline
super
message3#@#@#message4
```

You can use any terminator.
