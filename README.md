Schema-Harvester
================

[![Tests](https://github.com/elmarx/schema-harvester/actions/workflows/test.yaml/badge.svg)](https://github.com/elmarx/schema-harvester/actions/workflows/test.yaml)

Schema-Harvester is a tool that parses exsiting [JSON](https://www.json.org/json-en.html) documents
and tries to derive a [JSON schema](https://json-schema.org/) from these documents.

CLI Usage
---------

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

## Verify schemas

To verify that the generated schema is a valid JSON schema, we use
the [jsonschema crate's schema-validation](https://docs.rs/jsonschema/0.29.0/jsonschema/index.html#meta-schema-validation)
baked into an [executable](./core/examples/validate.rs).

```shell
cargo run --example validate schema.json
# or, eg directly from kafka
kcat -b localhost:9092 -t schemas -o-1 -C -e | cargo run --example validate
```