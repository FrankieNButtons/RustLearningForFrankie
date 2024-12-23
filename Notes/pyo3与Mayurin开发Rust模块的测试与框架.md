# pyo3与Mayurin开发Rust模块的测试与框架
## 基于pyo3的Rust加速Python模块开发流程
该流程偏向于Rust主导的项目开发流程，主要包含以下步骤：
### 总体流程
#### 1. 创建Rust项目
```bash
cargo new --lib pyo3_test
cd pyo3_test
```
#### 2. 修改`cargo.toml`并添加`pyo3`依赖包
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
### 将`src`目录中的`main.rs`调整为`lib.rs`并将结构调整如下
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
#### 使用`cargo`构建项目
```bash
cargo build --release
```
#### 将`target/release/`中的编译结果文件复制到python项目目录下
 - Linux/MacOS: `rust_core.so`
 - Windows: `rust_core.dll`
#### 将编译结果文件导入到python项目中并改为`rust_core.pyd`
#### 将在使用`rust_core`模块的python项目路径中添加如下`rust_core.pyi`模块存根文件以增加代码提示
```python
def add(a: int, b: int) -> int: ...   # or `pass`
```








## 基于maturin的Rust加速Python模块开发流程
### 总体流程
#### 新建一个空白项目（也可用cargo，但可清空）
```bash
mkdir rust_module
cd rust_module
```
#### 创建使用该模块的虚拟环境
```bash
python -m venv .venv
./.venv/Scripts/activate
```
#### 安装`maturin`模块
```bash
pip install maturin
```
#### 创建maturin项目
```bash
maturin init
```
init后需要选择顶上的`pyo3`项目，最终得到一个如下结构的Python项目：
```plaintext
rust_module/
├── .venv/                # 虚拟环境
├── Cargo.toml
├── src/
│   └── lib.rs
├── target/               # 编译输出目录
│   └── debug/
├── pyproject.toml        # 用于Maturin设置Python项目的配置文件
└── .gitignore            # Git忽略文件
```
