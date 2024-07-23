# <center> Chapter2: Basics of RustLanguage
## 2.1: Variability and Immutability
### 2.1.1: About Variables
1. use `let` to define a variable.  
**Note:** variables are defaultly immutable in Rust language. 
2. use `mut` to claim a mutable variable.
    - `let mut x: u32 = 35;` 
3. Rust supports **auto-detect** on datatype, but can also claim explicitly, e.g.
    - `let x: i32 = 5;` then `x` will be defined in `int32` dtype.
4. Some naming conventions for Rust:
    - `variables`: Snake Case normally(`my_method`, `my_var`······).
    - `enum`/`struct`: Pascal Case normally(`MyEnum`, `myStruct`······).
    - unused `variable`: start with "_" to avoid warning.
5. Dtype Conversion(Casting):
    - `let newVar = oldVal as {newDtype}`, then we got a new variable `newzVar` that is of `newDtype` dtype.

### 2.1.2 Demo Code 
```rust
fn code2_1(){
    let num: i32 = 100;
    let _mun: i64 = 54;
    //num = 13;

    let mut tum: i32 = 20;
    tum = tum + 1;
    // println!("tum={tum}");

    let x: i32 = 35;
    {
        let x: i32 = 10;
        println!("x={}", x);
    }
    println!("x={}", x);

    let x: &str = "Hello";
    println!("The new `x` is '{}'", x);

    let mut x: u16 = 30;
    println!("Mutable `x` is {}", x);
    x = 45;
    println!("Changed `x` is {}", x);
}
```
### 2.1.3 Notes on Code
**Warnings 2B Concerned**
1. Unused variables without a preceding underscore (`_`) will cause a compiler warning. For example, `let num: i32 = 100;`.
2. Variable initialized that receives excessive assignment will cause warning.

**Points 2B Remembered**
1. Using an underscore (`_`) before a variable name will avoid compiler warnings for unused variables. For example, `let _mun: i64 = 54;`.
2. Variables are immutable by default. Attempting to reassign an immutable variable will result in a compile-time error.
3. Variables can be shadowed within a new scope, and their values can be changed or even have their type altered. For example:
   ```rust
   let x: i32 = 35;
   {
       let x: i32 = 10;
       println!("x={}", x);
   }
   println!("x={}", x);
   let x: &str = "Hello";
   println!("The new `x` is '{}'", x);
   ```
4. Even immutable variables can be redeclared as mutable in the same scope. For example:
   ```rust
   let mut x: u16 = 30;
   println!("Mutable `x` is {}", x);
   x = 45;
   println!("Changed `x` is {}", x);
   ```
5. Variable shadowing does not extend beyond the scope in which the variable is shadowed. When the inner scope ends, the original variable is accessible again.


## 2.2 Constant & Static Variable
### 
