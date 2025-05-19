Here is an explanation of commonly used words in this open source project:

### AST (Abstract Syntax Tree)
An AST is a tree-like data structure that represents the hierarchical syntactic structure of source code or expressions. In this project, the [parser](#parser) builds an AST from [tokens](#token), and the [evaluator](#evaluator) processes the AST to compute results.

### Binary Crate
A [binary crate](#library-crate) is a Rust project that compiles to an executable program. In this project, the terminal calculator application is a binary crate.

### Branch
A branch is a separate environment of changes, diverging from the other branches of the project. This is to avoid conflicts while multiple people are working on a project. Conflicts are instead handled when branches are [merged](#merging) with a [pull request (pr)](#pull-request-pr)

### Cargo
Cargo is Rust's build system and package manager. It handles building, running, testing, and managing [dependencies](#dependency) for Rust projects.

### Commit
A commit is a part of [git](#git), where the changes you have made locally are saved.

### Dependency
A dependency is an external library or crate that your project uses. Dependencies are managed in Rust with the `Cargo.toml` file.

### Enum
An enum (enumeration) is a type that can be one of several defined variants. Enums are commonly used for [tokens](#token), [AST](#ast-abstract-syntax-tree) nodes, and error types in Rust.

### Error Handling
Error handling is the process of responding to and managing errors in code. Rust uses the `Result` and `Option` types for error handling, often with custom error enums.

### Evaluator
The evaluator processes the [AST](#ast-abstract-syntax-tree) to compute the final result of the [expression](#expression), handling [functions](#function), [variables](#variable), and operators.

### Expression
An expression is a combination of values, [variables](#variable), operators, and [functions](#function) that evaluates to a result.

### Function
A function is a reusable block of code that performs a specific task. In this project, mathematical functions like `sin`, `cos`, and user-defined functions are supported and handled by the [evaluator](#evaluator).

### Git
Git is the version control software this project uses, it is also the software which this website is based upon [*Git*Hub](#GitHub).

### Integration Test
An integration test checks how multiple parts of the codebase work together. In Rust, integration tests are placed in the `tests/` directory and test the public API of the crate, often involving multiple [modules](#module).

### Issue
An issue is the way we report [bugs](#bug), [request features](#feature-request), and track feature progess. Issues are closed by [maintainers](#maintainer) or [authors](#author).

### Lexer
The lexer (lexical analyzer) converts raw input text into a sequence of [tokens](#token) for the [parser](#parser) to process.

### Library Crate
A library crate is a Rust project that compiles to reusable code (a library) which can be used by other crates. Shared logic for the calculator can be placed in a library crate, and it can be used by a [binary crate](#binary-crate).

### main
`main` is the name for the central [branch](#branch) of this project.

### Maintainer
A maintainer is someone who has the rights to approve [pull requests (PR)](#pull-request-pr), create branches, directly commit to the [repository](#repository), and close [issues](#issue).

### Module
A module is a way to organize code within a crate. Modules help group related [functions](#function), [structs](#struct), and other items together.

### Parser
The parser takes a sequence of [tokens](#token) and builds an [abstract syntax tree (AST)](#ast-abstract-syntax-tree) representing the structure of the [expression](#expression).

### Plotting
Plotting refers to generating visual graphs of mathematical [functions](#function) or data. This project may use a Rust plotting library (like `plotters`) for graphing features.

### Pull Request (PR)
A pull request is a request to add the changes from your [commits](#commit) to the [main branch](#main).

### Repository
A repository is a cloud-based location where code is stored, managed by [git](#git).

### Struct
A struct is a custom data type that groups together related values. Structs are used for things like [tokens](#token), [AST](#ast-abstract-syntax-tree) nodes, and evaluation contexts.

### Token
A token is a basic unit of meaning in the input, such as a number, operator, or identifier. The [lexer](#lexer) splits input into tokens.

### Unit Test
A unit test checks the correctness of a small, isolated piece of code (such as a [function](#function) or method) within a single [module](#module).

### Variable
A variable is a named value that can be used and changed in [expressions](#expression). The calculator supports variables for storing and reusing values.

