# kfsend

Mini tool for send separated messages list to kafka.

## Run

`$ ./kfsend config.toml`

## Example

We can use next config

```
[fileconf]
filename="messages.list"
terminator="#@#@#"
[kafkakonf]
broker="127.0.0.1:9092"
topic="sender-test"
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