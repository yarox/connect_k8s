# Connect_k8s
This is a command-line tool written in Rust that allows you to easily connect to a Kubernetes cluster, namespace, and container of your choice.

## Dependencies
+ Rust
+ kubectl

## Installation
1. Clone the repository to your local machine:

```sh
git clone https://github.com/your-username/connect_k8s.git
```

2. Compile the source code using Rust:
```sh
cargo build --release
```

3. Add the compiled binary to your system's PATH:
```sh
export PATH=$PATH:/path/to/connect_k8s/target/release
```

# Usage
1. Run the connect_k8s command:

```sh
connect_k8s
```

2. Follow the prompts to select the cluster, namespace, review app, and container you want to connect to.

3. Once you've selected all the options, the tool will automatically connect you to the chosen container.

# License
This project is licensed under the MIT License. 
