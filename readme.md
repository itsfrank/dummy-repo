# CLI Project

A simple Rust CLI application with commands: foo, bar, baz, and animal.

## Usage

```bash
# Run the foo command
cargo run -- foo

# Run the bar command
cargo run -- bar

# Run the baz command
cargo run -- baz

# Run the animal command
cargo run -- animal <animal-type>
```

## Commands

- **foo**: Prints "foo command was run"
- **bar**: Prints "bar command was run"
- **baz**: Prints "baz command was run"
- **animal <type>**: Prints a cute ASCII art animal. Available types:
  - **cat**: Prints a cute cat
  - **bird**: Prints a cute bird
  - **snake**: Prints a cute snake

