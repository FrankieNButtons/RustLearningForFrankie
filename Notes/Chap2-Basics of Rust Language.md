# <center> Chapter2: Basics of RustLanguage </center>

## 2.1: Variability and Immutability

### 2.1.1: About Variables
1. use `let` to define a variable.  
**Note:** variables are defaultly immutable in Rust language. 
2. use `mut` to claim a mutable variable.
    - Usage: `let mut x: u32 = 35;` 
3. Rust supports **auto-detect** on datatype, but can also claim explicitly, e.g.
    - Usage: `let x: i32 = 5;` then `x` will be defined in `int32` dtype.
4. Some naming conventions for Rust:
    - `variables`: Snake Case normally(`my_method`, `my_var`······).
    - `enum`/`struct`: Pascal Case normally(`MyEnum`, `myStruct`······).
    - unused `variable`: start with "_" to avoid warning.
5. Dtype Conversion(Casting):
    - `let newVar = oldVal as {newDtype}`, then we got a new variable `newVar` that is of `newDtype` dtype.

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
- - -
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


## 2.2: Constant & Static Variable

### 2.2.1: Const vs Static
#### 2.2.1.1: What is a constant in Rust
1. Value confirmed when compiling, must have dtype and value declared.
2. Compiled solidly into machine code.
3. normally named in full capitalized style(e.g. `MY_CONSTANT`).
    - Usage: `const CST: u32 = 42`
4. Constant is only accessible to the realm that defining it.
#### 2.2.1.2: What is a static variable in Rust
1. Not solidified, a certain scale of RAM will be allocated to it when running.
2. normally named in full capitalized style(e.g. `MY_CONSTANT`).
    - Usage: `static STA: i32 = -42`
3. Not completely unchangable, can be changed with `unsafe code`.
4. Has a lifetime of whole running time.  

### 2.2.2: Const (in `Rust`) vs Macro (in `C`)
1. Safety: Constants are type-safe; macros are not.
2. Scope: Constants have clear module scope; macros can pollute namespaces.
3. Ease of Use: Constants are straightforward; macros are flexible but complex.
4. Debugging: Constants are easier to debug; macros can be challenging.

### 2.2.3: Demo Code
```rust
static MY_STA: i32 = 42;
static mut MY_MUT_STA:i8 = 42;

fn code2_2(){
    const _SECONDS_PER_HOUR: usize = 3_600;
    const MONTH_PER_YEAR: usize = 12;
    const MONTH_PER_DECADE: usize = 10 * MONTH_PER_YEAR;

    println!("A year has {} months.", MONTH_PER_YEAR);
    println!("A decade has {} months.", MONTH_PER_DECADE);
    {
        const TST:u32 = 1_000;
        println!("inner TST={}", TST);
    }
    // println!("outer TST={}", TST);

    println!("MY_STA={}", MY_STA);
    unsafe {
        MY_MUT_STA = 37;
        println!("inner MY_MUT_STA={}", MY_MUT_STA);
        println!("inner MY_STA={}", MY_STA);
    }
    println!("outer MY_STA={}", MY_STA);
}
```
### 2.2.4: Notes on Code
**Warnings 2B Concerned**
1. Constants are only accessible within the scope that defines them. For example:
   ```rust
   {
       const TST:u32 = 1_000;
       println!("inner TST={}", TST);
   }
   ```
2. Unsafe code is not recommended in Rust. Mutable static variables require `unsafe` blocks to be modified.
- - -
**Points 2B Remembered**
1. Constants can be defined and used within their defining scope.
2. Constants  be computed at compile time.
3. Immutable static variables can be used in `unsafe` code blocks.
4. Mutable static variables can only be modified within `unsafe` code blocks. 

## 2.3: Basic Dtypes (Similar to C/C++)
### 2.3.1: Intro to those dtypes
- define: `let (mut) yourVar: dtype = val`
#### 2.3.1.1: Integer Types
- Integer Types
   1. `i8`
   2. `i16`
   3. `i32` (default)
   4. `i64`
   5. `i128`
- Unsigned Integer Types
   1. `u8`
   2. `u16`
   3. `u32`
   4. `u64`
   5. `u128`  
- Platform-Specific Integer Types (depending on your platform)
   1. `usize`
   2. `isize`
#### Float Types
- `f32` (float)
- `f64` (double, default)  
**Note:** Use `f64` if you're not sure about your demands.
#### Boolean Values (`bool`)
- `true`
- `false`
#### Character Type(`char`)
1. Unicode characters in Rust  
2. declared with `'yourChar'` (just like C/C++)

### 2.3.2: Demo Codes
```rust
fn code2_3(){
    let a1 = 233;
    let a2: i64 = 0xFFFF;
    let a4: i16 = 0o666;
    let a3: i8 = 0b1111;
    println!("a1 = {a1}\na2 = {a2}\na3 = {a3}\na4 = {a4}");

    println!("Max of u32 is {}", u32::MAX);
    println!("Min of u32 is {}", u32::MIN);
    println!("Max of i64 is {}", i64::MAX);
    println!("Min of i64 is {}", i64::MIN);
    println!("Max of usize is {}", usize::MAX);

    println!("u32 ocps {} bytes", std::mem::size_of::<u32>());
    println!("i8 ocps {} bytes", std::mem::size_of::<i8>());
    println!("isize ocps {} bytes", std::mem::size_of::<isize>());


    let f1: f32 = 3.1415926;
    let f2: f64 = 6.71828;
    // println!("Max of f32 is {}", f32::MAX);
    // println!("Min of f64 is {}", f64::MIN);
    println!("f1 = {:.2}\nf2 = {:.4}", f1, f2);


    let True = true;
    let mut False: bool = false;
    if True{
        println!("True = {}", True);
    }
    if !False{
        println!("False = {}", False);
    }

    println!("True && False = {}\nTrue || False = {}",
             True && False,
             True || False);


    let char_e: char = 'E';
    let char_emoji: char = '❤';
    println!("I got {char_e}velyn and I feel {char_emoji}.");      
}
```
### 2.3.3: Notes on Code
**Warnings 2B Concerned**
1. Snake case naming is recommended for variables.  
2. Using capitalized reserved words for variable names is not recommended.  
3. For example:
   ```rust
   let True = true;
   ```
4. Unchanged variables need no `mut` declaration. For example:
   ```rust
   let mut False: bool = false;
   // and no more set value for False
   ```

- - -

**Points 2B Remembered**
1. Default integer type is `i32`. For example:
   ```rust
   let a1 = 233;
   ```
2. Different ways to define integer literals:
   - Hexadecimal: 
     ```rust
     let a2: i64 = 0xFFFF;
     ```
   - Octal:
     ```rust
     let a4: i16 = 0o666;
     ```
   - Binary:
     ```rust
     let a3: i8 = 0b1111;
     ```
3. Checking the size of data types in bytes. For example:
   ```rust
   println!("u32 ocps {} bytes", std::mem::size_of::<u32>()); 
   ```
4. Formatting floating-point numbers in Rust requires no 'f' sign. For example:
   ```rust
   println!("f1 = {:.2}\nf2 = {:.4}", f1, f2); 
   ```
5. Boolean values and logical operations:
   ```rust
   let True = true;
   let False = false;
   println!("True && False = {}\nTrue || False = {}", True && False, True || False);
   ```
6. Unicode characters, including emojis, are available in Rust. For example:
   ```rust
   let char_e: char = 'E';
   let char_emoji: char = '❤';
   println!("I got {char_e}velyn and I feel {char_emoji}.");
   ```

## 2.4: Tuple & Array
### 2.4.1: Intro to those data types
#### General Preview
1. Both of them are Compound dypes, which is a combination of different dtypes of data;
2. Both of them are fixed-length;
3. Compared to Vectors & Maps: Those two are collection dtypes;
#### Tuple
1. Support different dtypes of data(Isomerous);
2. Defined with `let tup = (dtype1, dtype2, ...)`;
3. defaultly return `Empty Tuple()`;
4. No `tup.len()` method
5. fetch element with `tup.index`;
6. Directly support iteration(`for item in tup`);  
#### Array
1. Support only one dtype of data(Heterogenous);
2. Defined with `array = [a, b, c]` or `array = [value; length]`;
3. fetch element with `array[index]`;
4. `array.len()` to get the length of array;
5. Directly support slicing(`array[start:end]`);
6. Directly support iteration(`for item in array`);
### 2.4.2: Demo Codes
```rust
fn code2_4(){
    // Tuple Usage Test
    let a = (1, "hi", 6.7, 9);                                        // Immutable tuple can be defined like this.
    // println!("a = ({} {} {} {} {})", a.0, a.1, a.2, a.3, a.4);     // [Error]How to index a tuple(And index out of bounds will cause UnkwnownFieldError). 
    println!("a = ({} {} {} {})", a.0, a.1, a.2, a.3);
    println!("a = {:?}", a);                                          // Print the whole code form of a tuple with `:?`.
    // println!("a.len() = {}", a.len());                             // tuple has no len() method.
    
    let mut b = (1, "hi", 6.7, 9);
    // b.1 = 8;                                                       // [Error]Cross dtype change is not allowed ecen in tuple.
    b.0 = 2;
    b.1 = "hello";
    b.2 = 3.1415926;
    b.3 = 8;
    println!("b = {:?}", b);

    // Array Usage Test
    let arr1 = [1, 2, 3, 4];                                          // Immutable array can be defined like this with heterogeneous dtype.
    let arr2 = [7; 5];                                                // Define an array with same value.
    let arr3: [f32; 3] = [1.00, 2.71, 3.14];                          // Define an array with heterogeneous dtype.
    
    // println!("arr1[2] = {}", arr1[5]);                             // How to index an array(And index out of bounds will cause IndexOutOfBoundError).
    println!("arr1[2] = {}", arr1[2]);
    println!("arr1 = {:?}, arr1.len() = {}", arr1, arr1.len());       // Fetch the length of an array with `.len()` method.
    println!("arr2 = {:?}, arr2.len() = {}", arr2, arr2.len());
    println!("arr3 = {:?}, arr3.len() = {}", arr3, arr3.len());
    
    for i in 0..arr1.len(){                                           // Iterate over an array with index.
        println!("arr1[{}] = {}", i, arr1[i]);
    }
    
    for (i,item) in arr2.iter().enumerate(){                          // Iterate over an array with enumerator.
        println!("arr2[{}] = {} ", i, item);
    }
    
    print!("arr3 = [");
    for i in arr3{                                                    // Iterate over an array directly.
        print!("{:.2} ", i);
    }
    println!("]");
    
}
```

### 2.4.3: Notes on Code
**Warnings 2B Concerned**
1. Indexing a tuple or array out of bounds will result in an error. For example:
   ```rust
   // println!("a = ({} {} {} {} {})", a.0, a.1, a.2, a.3, a.4); // [Error] Index out of bounds causes UnkwnownFieldError
   // println!("arr1[2] = {}", arr1[5]);                        // [Error] Index out of bounds causes IndexOutOfBoundError
   ```

2. Cross dtype changes are not allowed even in tuples. For example:
   ```rust
   // b.1 = 8; // [Error] Cross dtype change is not allowed even in tuple.
   ```

- - -
*
**Points 2B Remembered**
1. Immutable tuples and arrays can be defined and printed directly. For example:
   ```rust
   let a = (1, "hi", 6.7, 9);
   println!("a = ({} {} {} {})", a.0, a.1, a.2, a.3);
   println!("a = {:?}", a); // Use `:?` to print the whole tuple
   ```

2. You can mutate values in a tuple as long as they stay within the same dtype. For example:
   ```rust
   let mut b = (1, "hi", 6.7, 9);
   b.0 = 2;
   b.1 = "hello";
   b.2 = 3.1415926;
   b.3 = 8;
   println!("b = {:?}", b);
   ```

3. Arrays in Rust can be defined with homogeneous dtypes and support several initialization methods:
   - Regular array definition:
     ```rust
     let arr1 = [1, 2, 3, 4]; // Immutable array with defined values
     ```
   - Repeated value array:
     ```rust
     let arr2 = [7; 5]; // Define an array with same value repeated 5 times
     ```

4. Arrays support fetching their length with `.len()`. For example:
   ```rust
   println!("arr1.len() = {}", arr1.len());
   ```

5. Arrays can be iterated using index, `.iter()` method, or directly. For example:
   - Using index:
     ```rust
     for i in 0..arr1.len(){
         println!("arr1[{}] = {}", i, arr1[i]);
     }
     ```
   - Using `.iter()` with `enumerate()`:
     ```rust
     for (i, item) in arr2.iter().enumerate(){
         println!("arr2[{}] = {}", i, item);
     }
     ```
   - Direct iteration:
     ```rust
     for i in arr3{
         print!("{:.2} ", i);
     }
     ```
