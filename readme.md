# Kafka Pub/Sub Demo 🚀🎉

`kafka-pub-sub-demo` is a simple yet powerful demonstration of a Kafka publisher in Python and a Kafka consumer in Rust, showcasing real-time message streaming capabilities. This project aims to provide a hands-on approach to understanding the seamless interaction between these languages and Kafka in a practical setting.

## Table of Contents 📖

- [Features](#features-)
- [Prerequisites](#prerequisites-)
- [Getting Started](#getting-started-)
- [Usage](#usage-)
- [Contributing](#contributing-)
- [Resources](#resources-)
- [License](#license-)

## Features 🌟

- 🐍 Python publisher sends messages to Kafka at a configured interval
- 🦀 Rust consumer subscribes to the Kafka topic and prints received messages
- 📦 Self-contained `docker-compose` configuration for easy deployment and testing
- 📚 Comprehensive documentation and usage instructions

## Prerequisites 🧰

- [Docker](https://www.docker.com/)
- [Docker Compose](https://docs.docker.com/compose/)
- [Python](https://www.python.org/)
- [Poetry](https://python-poetry.org/)
- [Rust](https://www.rust-lang.org/)
- [Kafka](https://kafka.apache.org/)

## Getting Started 🚀

1. Clone the repository:

```bash

git clone https://github.com/yourusername/kafka-pub-sub-demo.git

```

2. Change to the project directory:

```bash
cd kafka-pub-sub-demo

```

3. Install dependencies:

```bash
poetry install


```

4. Build the Docker containers:

```bash
docker-compose build

```

## Usage 🎮

1. Start the Kafka Pub/Sub demo:

```bash
docker-compose up



```

2. Observe the messages being published by the Python publisher and consumed by the Rust consumer in the terminal output.

3. To stop the demo, press `Ctrl+C`.

## Contributing 🤝

Contributions, issues, and feature requests are welcome! Feel free to check the [Issues](https://github.com/yourusername/kafka-pub-sub-demo/issues) page.

## Resources 📚

- [Kafka Documentation](https://kafka.apache.org/documentation/)
- [Python Kafka Client (Confluent-Kafka-Python)](https://github.com/confluentinc/confluent-kafka-python)
- [Rust Kafka Client (RDKafka)](https://github.com/fede1024/rust-rdkafka)

## License ⚖️

This project is licensed under the [MIT License](https://opensource.org/licenses/MIT). See the [LICENSE](https://github.com/yourusername/kafka-pub-sub-demo/blob/main/LICENSE) file for more details.
