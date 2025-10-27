# 0-Shell 

[![Rust](https://img.shields.io/badge/Rust-1.72.0-orange?logo=rust)](https://www.rust-lang.org/)  
[![License](https://img.shields.io/badge/License-MIT-blue)](LICENSE)

A **minimalist Unix-like shell implemented in Rust**, designed to execute core Unix commands using system-level Rust abstractions—without relying on external binaries or built-in shells like bash/sh.  

Inspired by [BusyBox](https://busybox.net/), this shell helps you learn Unix system programming, process management, and file system operations.

---

## Features

- Interactive shell with `$ ` prompt  
- Parses and executes commands: `echo`, `cd`, `ls`, `pwd`, `cat`, `cp`, `rm`, `mv`, `mkdir`, `exit`  
- Supports `ls -l`, `-a`, `-F` and `rm -r`  
- Graceful handling of Ctrl+D (EOF)  
- Prints error for unrecognized commands:  
  `Command '<name>' not found`  
- Lightweight and standalone—no external binaries required  

---

## Installation

```bash
git clone <https://learn.zone01oujda.ma/git/ihajji/0-shell>
cd 0-shell
cargo r 

```
## Usage

```bash
$ cd dev
$ pwd
/dev
$ ls -l
total 0
crw-------  1 root root 10, 58 Feb 5 09:21 acpi_thermal_rel
drwxr-xr-x  2 root root    540 Feb 5 09:21 block
$ echo "Hello There"
Hello There
$ something
Command 'something' not found
$ exit
```
## Learning Objectives

- File & directory operations
- Command parsing & shell loops
- Robust error handling
- Unix process & system call APIs
- Executing commands without external binaries

## Resources

- [Rust std::fs Documentation](https://doc.rust-lang.org/std/fs/)
- [Rust std::process Documentation](https://doc.rust-lang.org/std/process/)
- [Unix Shell - Wikipedia](https://en.wikipedia.org/wiki/Unix_shell)
- [BusyBox](https://busybox.net/)
- `man` pages: `man 2 open`, `man 2 execve`
