# Command Parsing Module

This module provides a way to parse command-line-like input into a structured format for easier handling.

## `Parsing` Struct

The core data structure is the `Parsing` struct, which holds the components of a parsed command:

| Field   | Type          | Description                          |
|---------|---------------|------------------------------------|
| command | `String`      | The main command name               |
| arg     | `Vec<String>` | A vector of arguments for the command |
| flag    | `Vec<String>` | A vector of flags (options)         |

### Definition

```rust
pub struct Parsing {
    pub command: String,
    pub arg: Vec<String>,
    pub flag: Vec<String>,
}
```
# Command Parsing Module

This module provides a way to parse command-line-like input into a structured format for easier handling.

## `Parsing` Struct

The core data structure is the `Parsing` struct, which holds the components of a parsed command:

| Field   | Type          | Description                          |
|---------|---------------|------------------------------------|
| command | `String`      | The main command name               |
| arg     | `Vec<String>` | A vector of arguments for the command |
| flag    | `Vec<String>` | A vector of flags (options)         |

### Definition

```rust
pub struct Parsing {
    pub command: String,
    pub arg: Vec<String>,
    pub flag: Vec<String>,
}
```
