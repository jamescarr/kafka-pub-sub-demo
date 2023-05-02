from kafka import KafkaProducer
import time
import json

producer = KafkaProducer(
    bootstrap_servers=["kafka:9092"],
    value_serializer=lambda v: json.dumps(v).encode("utf-8"),
)

topic = "test-topic"
interval = 5

while True:
    message = {"message": "Hello from Python!", "timestamp": time.time()}
    producer.send(topic, message)
    print(f"Sent message: {message}")
    time.sleep(interval)
