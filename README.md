# mongo-perf

**mongo-perf** is a Rust utility designed to stress test MongoDB by generating and executing a high volume of read and write operations. Its primary purpose is to help measure the performance of your MongoDB instance under various workloads. Please note that this project itself does not provide fully functional instrumentation of MongoDB but can be used in conjunction with other tools to analyze MongoDB performance.

## Installation

You can install **mongo-perf** by cloning this repository and building it using Cargo, Rust's package manager:

```sh
git clone git@github.com:mrdck/mongo-perf.git
cd mongo-perf
cargo build --release
```

## Usage

To use **mongo-perf**, follow the provided command-line syntax:

```sh
mongo-perf --uri <mongodb-uri> --write-iterations <writes> --read-iterations <reads>
```

- `--uri <mongodb-uri>`: Specifies the MongoDB connection URI, including the protocol, host, port, and authentication credentials if required.

- `--write-iterations <writes>`: Defines the number of write operations to perform on the MongoDB instance.

- `--read-iterations <reads>`: Sets the number of read operations to execute against the MongoDB instance.

### Example

```sh
mongo-perf --uri mongodb://localhost:27017 --write-iterations 1000 --read-iterations 500
```

This command will execute multiple read and write operations against the MongoDB instance located at `localhost:27017`. It will perform 1000 write operations and 500 read operations.

## Disclaimer

**mongo-perf** is intended for use in performance testing and benchmarking scenarios. Be cautious when using it in a production environment, as it generates a high volume of traffic and may impact the stability and performance of your MongoDB server.

## Contributing

If you would like to contribute to this project, please feel free to fork the repository and submit a pull request with your improvements or bug fixes.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
