# Rust Learning Challende

# Day1
## Description
This Rust project interfaces with the Cosmos-sdk REST endpoint to fetch transaction data based on a user-specified block height and dumps the transaction data onto the console. The primary objective is to delve deep into the following Rust concepts:
- Ownership and borrowing mechanics.
- Structs and Enums: Usage and best practices.
- Error handling techniques.
- Modular organization of code.

## Challenges Encountered

### Ownership & Borrowing Mechanics
Coming from languages that utilize garbage collection (GC), understanding how Rust eliminates the need for GC and ensures safe memory management was initially a hurdle. It demanded revisiting some foundational C programming concepts, particularly those related to pointers.

### Idiomatic Rust
Grasping the idiomatic practices of Rust presented its own set of challenges. Writing in a manner that's true to the Rust philosophy took some time and a lot of practice.

### Serialization with Serde
Using the Serde library for serialization and deserialization brought its own nuances to the forefront, especially when handling complex data structures and different types of messages coming from the blockchain.

### Debugging Techniques
Understanding the debugging flow in Rust, especially when juxtaposed against languages like Go, required some acclimation. Figuring out how the Rust debugger operates and how to efficiently troubleshoot issues was a journey on its own.

## Overcoming Challenges

The key to navigating through these challenges was a combination of:

- **Official Documentation**: Continuously referring to the official documentation provided insights into best practices.

- **Standard Library Dive**: Spending quality time with the Rust standard library gave a better understanding of the language's constructs and their idiomatic use.

- **Basic I/O Coding Problems**: Working on elementary input/output coding problems in Rust played a pivotal role in grasping the language's flow and syntax. It was a hands-on way to test and reinforce new knowledge.

- **Design Patterns**: Exploring how traditional design patterns translate into Rust was enlightening. Implemented the singleton pattern.

- **Debugging Techniques**: Figuring out how the Rust debugger operates and how to efficiently troubleshoot issues became crucial. Once the debugging techniques were familiar, it significantly eased the process of writing and refining the code.

## Lessons Learned

- **Expression-Based Language**: One of the standout features of Rust is that almost everything is an expression, which can sometimes be both surprising and powerful.

- **Memory Management**: Rust's unique approach to memory management, especially how it differentiates between heap and stack using ownership principles, was a refreshing and enlightening experience. It underscores the language's focus on safety and performance.
