# run tests/checks that are also run by github actions
ci:
    cargo check --all-targets
    cargo fmt --check
    cargo clippy --all-targets -- -D warnings
    cargo clippy --all-targets -- -D clippy::pedantic
    cargo nextest run

kafka_fixtures topic file:
    cat {{ file }} | kcat -b localhost:9092 -t {{ topic }} -P

# read the schemas topic. Append `-e` to stop after reading the last message
kcat_schemas *args:
    kcat -b localhost:9092 -t schemas -o-1 -C {{ args }}

kafka_topic_recreate:
    docker-compose exec kafka /opt/kafka/bin/kafka-topics.sh --bootstrap-server localhost:9092 --delete --topic schemas --if-exists
    docker-compose exec kafka /opt/kafka/bin/kafka-topics.sh --bootstrap-server localhost:9092 --create --partitions 12 --topic schemas --config cleanup.policy=compact
