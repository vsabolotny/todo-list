# Simple To-Do List Application

This is a simple command-line based To-Do List application that allows users to manage their tasks efficiently.

## Basic Features

- **Add tasks**: Accept a task from the user and save it.
- **List tasks**: Show all added tasks.
- **Mark tasks as done**: Update the status of a task.
- **Save tasks to a file**: Make tasks persistent across runs.

## Usage

1. **Add a Task**
    ```sh
    ./todo add "Your task description"
    ```

2. **List All Tasks**
    ```sh
    ./todo list
    ```

3. **Mark a Task as Done**
    ```sh
    ./todo done <task_id>
    ```

4. **Save Tasks to a File**
    Tasks are automatically saved to a file to ensure persistence across runs.

## Setting Up the Rust Environment

To build and run this application, you need to have Rust installed on your system. Follow these steps to set up the Rust environment:

1. **Install Rust**: Use the following command to install Rust using `rustup` (the recommended way):
    ```sh
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

2. **Configure your current shell**: After installation, configure your current shell to use Rust:
    ```sh
    source $HOME/.cargo/env
    ```

3. **Verify the installation**: Check if Rust is installed correctly by running:
    ```sh
    rustc --version
    ```

4. **Build the project**: Navigate to the project directory and build the project using Cargo:
    ```sh
    cd /Users/vsabolotny/Projects/todo-list
    cargo build
    ```

5. **Run the application**: After building the project, you can run the application using:
    ```sh
    cargo run
    ```

## Installation

Clone the repository and navigate to the project directory:
```sh
git clone https://github.com/vsabolotny/todo-list.git
cd /Users/vsabolotny/Projects/todo-list
```

Make the script executable:
```sh
chmod +x todo
```

## Contributing

Contributions are welcome! Please fork the repository and submit a pull request.

## License

This project is licensed under the MIT License.