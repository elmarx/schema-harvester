Schema-Harvester
================

[![Rust](https://github.com/elmarx/schema-harvester/actions/workflows/rust.yml)]

Schema-Harvester is a tool that parses exsiting [JSON](https://www.json.org/json-en.html) documents and tries to derive a [JSON schema](https://json-schema.org/) from these documents.

Usage
-----

Consume a file with line separated JSON documents:

```shell
$ cat line_separated.json | schema-harvester
```

Consume via MQTT (using [Eclipse Mosquitto](https://mosquitto.org/)):

```shell
$ mosquitto_sub -t homeassistant/event | schema-harvester
```

Consume from Kafka (using [kcat](https://github.com/edenhill/kcat#readme)):

```shell
$ kcat -b $KAFKA_BROKER_ADDRESS_LIST -t your_topic | schema-harvester
```

### Verify schemas

You may use any JSON schema validator to validate the input documents with the derived schema. This example uses [yajsv](https://github.com/neilpa/yajsv):

```shell
yajsv -s schema.json line_separated.json
```
