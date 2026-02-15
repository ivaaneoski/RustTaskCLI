# TODO List (Rust)

Simple command-line TODO list application written in Rust with file persistence.

## Requirements

- Rust 1.x or higher
- Cargo (comes with Rust)

## Installation

```bash
git clone <repository-url>
cd todo-list-rust
```

## Run

```bash
cargo run
```

## Build

```bash
cargo build --release
./target/release/todo-list-rust
```

## Usage

The app presents a menu-driven interface:

```
To-Do List:
1. Buy groceries
2. Finish homework

Options:
1. Add Task
2. Remove Task
3. Exit

Choose an option:
```

### Add Task

```
Choose an option: 1
Enter a new task: Call dentist
Task added!
```

### Remove Task

```
Choose an option: 2
Enter task number to remove: 1
Task removed!
```

### Exit

```
Choose an option: 3
Goodbye!
```

## Implementation Details

- Tasks are stored persistently in `todo.txt` file
- File is created automatically if it doesn't exist
- Tasks persist between program runs
- Simple numbered list interface
- Uses Rust standard library for file operations

## Features

- Persistent storage using text file
- Add tasks
- Remove tasks by number
- List all tasks
- Automatic file creation
- Error handling for invalid inputs

## File Structure

All tasks are stored in `todo.txt` in the current directory, with one task per line.
