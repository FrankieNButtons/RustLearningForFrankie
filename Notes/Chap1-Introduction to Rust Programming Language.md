# <center>Chapter1: Introduction to Rust Language 
## 1.1: Basic Components:
 - Language Manager: `Rustup`
 - Compiler: `rustc`
 - Package Manager: `cargo`
### 1.1.1: rustup usage
 - `rustup update`: Update Rust language.
 - `rustup --version`: Check the version of Rust language.
 - `rustup self uninstall`: uninstall Rust anguage.
 - `rustup component add rustfmt`: Add component to Rust.

### 1.1.2: rustc usage
 - `rustc --version`: Check the version of rust compiler.
 - `rustc yourSourceCode.rs`: then you will get your executive filein your pwd/cwd.
 - `rustc --crate-type lib yourCrateCode.rs`: building a `cteate`(package of rust language)  

### 1.1.3: cargo usage
 - `cargo new yourProjectName`: Create a new directory and necessary conponents for a new rust project named after your command in your pwd/cwd.  
 - `cargo check`: Check errors and warnings in a rust project's directory.
 - `cargo build`: Compiling the whole project with all the dependencies.
      - `--`: No certain attachment for default compiling, debugging oriented.
      - `--release`: **Optimized** compiling process for production Projects, releasing oriented.
 - `cargo run`: Defaultly building an executive file and run instantly.
 - `cargo test`: Defaultly building an executive file with debugging infobation. 


## 1.2: Project's Structure
```markdown
my_rust_project/
├── Cargo.toml
├── Cargo.lock
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── bin/
│   │   └── other_binary.rs
│   ├── tests/
│   │   └── integration_test.rs
│   ├── examples/
│   │   └── example.rs
│   └── modules/
│       └── mod1.rs
│       └── mod2.rs
├── target/
├── .gitignore
└── README.md
```

### 1.2.1: Insight into `Cargo.toml`(extended by GPT-4o)
1. `[package]`: A section for the Rust project's configuration.
    - `name`: The name of the Rust project.
    - `version`: The version of the Rust project.
    - `edition`: The Rust edition being used, typically "2018" or "2021".
    - `description`: A brief description of your project.
    - `authors`: The authors of the project.
    - `license`: The license for the project.
    - `repository`: The URL of the project's repository.
    - `documentation`: The URL for the project's documentation.
    - `readme`: The README file for the project.
    - `keywords`: Keywords that describe the project.
    - `categories`: Categories that classify the project.
    - `workspace`: The name of the workspace the project belongs to.

2. `[dependencies]`: Lists the project's dependencies.
    - `dependency_name`: The name and version of a dependency.

3. `[dev-dependencies]`: Lists dependencies required for development.
    - `dependency_name`: The name and version of a development dependency.

4. `[build-dependencies]`: Lists dependencies required for building the project.
    - `dependency_name`: The name and version of a build dependency.

5. `[target.'cfg(target_os = "os_name")'.dependencies]`: Lists dependencies for specific target platforms.
    - `dependency_name`: The name and version of a platform-specific dependency.

6. `[features]`: Defines features for the project.
    - `default`: The default features enabled for the project.
    - `feature_name`: Additional features that can be enabled.

7. `[badges]`: Configuration for project badges.
    - `badge_name`: The configuration for a specific badge, such as CI status.

8. `[[bin]]`: Configuration for custom binary files.
    - `name`: The name of the binary.
    - `path`: The path to the binary file.

9. `[[example]]`: Configuration for example files.
    - `name`: The name of the example.
    - `path`: The path to the example file.

10. `[[test]]`: Configuration for test files.
    - `name`: The name of the test.
    - `path`: The path to the test file.

11. `[[bench]]`: Configuration for benchmark files.
    - `name`: The name of the benchmark.
    - `path`: The path to the benchmark file.  
### 1.2.2: A Demo of `Cargo.toml`
```toml
[package]
name = "my_rust_project"
version = "0.1.0"
edition = "2021"
description = "A sample Rust project to demonstrate Cargo.toml configuration."
authors = ["Your Name <your.email@example.com>"]
license = "MIT"
repository = "https://github.com/yourusername/my_rust_project"
documentation = "https://docs.rs/my_rust_project"
readme = "README.md"
keywords = ["rust", "example", "cargo"]
categories = ["command-line-utilities"]
workspace = "my_workspace"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
reqwest = "0.11"

[dev-dependencies]
tokio = { version = "1.0", features = ["full"] }
assert_cmd = "1.0"

[build-dependencies]
cc = "1.0"

[target.'cfg(target_os = "windows")'.dependencies]
winapi = { version = "0.3", features = ["winuser"] }

[features]
default = ["serde"]
special_feature = ["serde", "reqwest"]

[badges]
travis-ci = { repository = "yourusername/my_rust_project", branch = "main" }

[[bin]]
name = "my_rust_project"
path = "src/main.rs"

[[example]]
name = "example1"
path = "examples/example1.rs"

[[test]]
name = "integration_test"
path = "tests/integration_test.rs"

[[bench]]
name = "benchmark1"
path = "benches/benchmark1.rs"
```
