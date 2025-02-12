# Python Module Implemented by Rust with `pyo3` & `Maturin`
## Develop Python Module Implemented by Rust with `pyo3`
### Main Steps
#### 1. Create Rust Project
```bash
cargo new --lib pyo3_test
cd pyo3_test
```
#### 2. Edit `cargo.toml` by adding `pyo3` dependencies
以下是一个示例`cargo.toml`：
```TOML
[package]
name = "pyo3_test"
version = "0.1.0"
edition = "2021"

[lib]
name = "rust_core"                             # Your module name
crate-type = ["cdylib"]


[dependencies]

[dependencies.pyo3]                             # dependency for Python bindings
version = "0.23.3"                              # Latest version of course
features = ["extension-module"]                 # Enable extension module feature
```
### Rename `main.rs` from `src` to `lib.rs`
```rust
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;


/// a test Python module implementing a simple add function by rust
/// x: type of a int(i32 in rust)
/// y: type of a int(i32 in rust)
/// return: type of a int(i32 in rust)
#[pyfunction]                                   // the function in the module
fn add(x: i32, y: i32) -> PyResult<i32>{
    return Ok(x + y);
}

#[pymodule]                                    // the module to be import in python and the attributes under it need to be wrapped here 
fn rust_core(m: &Bound<'_, PyModule>) -> PyResult<()>{
    m.add_function(wrap_pyfunction!(add, m)?)?;
    return Ok(());
```
#### Build the `--release` Version with `cargo`
```bash
cargo build --release
```
#### Get the compiled ile in `target/release/`
 - Linux/MacOS: `rust_core.so`
 - Windows: `rust_core.dll`
#### Renamed the file into `rust_core.pyd` and move it to the python project
#### Add `rust_core.pyi` file in the same directory with `rust_core.pyd`
```python
def add(a: int, b: int) -> int: ...   # or `pass`
```








## Python Module Implemented by Rust with `maturin`
### Main Steps
#### Create a New Project
```bash
mkdir rust_module
cd rust_module
```
#### Create & Activate a Python Virtual Environment
```bash
python -m venv .venv
./.venv/Scripts/activate
```
#### Install `maturin` Module
```bash
pip install maturin
```
#### Create a `Maturin` Project
```bash
maturin init
```
Select `pyo3` project and initialize the project with the following content：
```plaintext
rust_module/
├── .venv/                # Virtual Environment
├── Cargo.toml
├── src/
│   └── lib.rs
├── target/               # Output directory of compiler
│   └── debug/
├── pyproject.toml        # Configuration of Python project over Maturin
└── .gitignore
```

