# <center> Chapter2: Ownership Machanism of Rust </center>
## 3.0 General Previews of Ownership
### Demo Codes
```rust
// For thouse variables that are not that complex, Rust use `Copy` Strategy
let arr = [1, 3, 5];
let tup = (15, "ok", 42, 666);
println!("arr: {:?}", arr);
println!("tup: {:?}", tup);

let arr_ownership = arr;                                                        // ownership transfer
let tup_ownership = tup;
println!("arr_ownership: {:?}", arr_ownership);                                           // the memory has been copied.(Not only apply the pointer to the variable)
println!("tup_ownership: {:?}", tup_ownership);

let a = 3;
let b = a;                                                                           // Variable `a` is not covered by `b` 
println!("a: {}, b: {}", a, b);                                                           // Thus `a` can still be printed

// But for those variables that are more complex, Rust use `Move` Strategy
let str_1 = String::from("hello");
let str_2 = str_1;
// println!("str_1: {}", str_1);                                                          // [Error]The value of `str_1` has been moved to `str_2`, which will cause BorrowOfMovedValueError
println!("str_2: {}", str_2);
```

### Notes on Codes
1. `Copy` Strategy: The variables that are not that complex, Rust will use `Copy` Strategy to transfer ownership. For example, `arr` and `tup` are both arrays and tuples, which are not that complex. Thus, Rust will use `Copy` Strategy to transfer ownership.
2. `Move` Strategy: The variables that are more complex, Rust will use `Move` Strategy to transfer ownership. For example, `str_1` is a string, which is more complex than arrays and tuples. Thus, Rust will use `Move` Strategy to transfer ownership.


## 3.1 Memory Management Mechanism of Rust
### 3.1.1 Errors in regards of memory mechanism in C/C++
1. Memory Leak: The memory that is allocated but not freed(e.g. `int* str = new int` Solution: `delete str`)
2. Dangling Pointer: The pointer that points to the memory that has been freed(e.g. `int* str = new int; delete str; int* str2 = str;` Solution: `delete str2`)
3. Double Free: The memory that is freed twice(e.g. `int* str = new int; delete str; delete str;` Solution: `delete str`only once.)
4. Array out of bounds: The array that is out of bounds(e.g. `int* str = new int[10]; str[10] = 1;` Solution: `str[9] = 1;`)
5. Use after free: The memory that is freed but still used(e.g. `int* str = new int; delete str; str[0] = 1;` Solution: no `str` usage after `delete str`)
6. Stack overflow: The stack that is overflowed(e.g. `int* str = new int[100000000];` Solution: `int* str = new int[100];`)
7. Wild pointer: The pointer that is not initialized(e.g. `int* str; str[0] = 1;` Solution: `int* str = new int; str[0] = 1;`)
8. Ambiguous usage of `new/delete` and `mealoc/free`(): The usage of `new/delete` and `mealoc/free`() is ambiguous(e.g. `int* str = new int; free(str);` Solution:`int* str = (int*)malloc(sizeof(int)); free)
### 3.1.2 Memory Management Mechanism in Rust
 - Borrowing: `Variable` or `Invariable`
 - Lifetime: The memory that’s been used passed as `ownership` 
 - Reference Counting: Calculated while compiling, freed when the reference count is 0.
### 3.1.3 Demo Codes
```rust
// `Copy` on baic type
let c1 = 1;
let _c2 = c1;                                                                          // `Copy` on basic type
println!("_c1={}", c1);
// `Ownership borrowed`` on complex type
let s1 = String::from("hello");
let _s2 = s1;                                                                          // `Move` the ownership of `s1` to `s2`
// println!("_s1={}", s1);                                                             // [Error]The ownership of value in `s1` has been moved to `s2`, which will cause BorrowOfMovedValueError
println!("_s2={}", _s2);
let s2 = _s2.clone();                                                                  //  `Clone` on complex type
println!("s2={}", s2);
println!("_s2={}", _s2);

fn get_length(s: String)-> usize{      
    println!("Length of {}, is {}", s, s.len());
    s.len()                                                                            // No `;` to return and claim the dtype on annotation
}

let s3 = String::from("hello3");
let ln = get_length(s3);
// println!("s3 '{}'is not kept.", s3);                                                // The ownership of value in `s3` has been moved to function `get_length`, which will cause BorrowOfMovedValueError
println!("length {} of s3 is returned.", ln);


// Solution 1
fn print_string(s: String) -> String{                                                  // return the Struct as the input variable
    println!("{}", s);
    s.to_owned()
}
let s4 = String::from("hello4");
let s4_copy = print_string(s4);
println!("s4_copy={}", s4_copy);


// Solution 2
fn print_string_static(s: String) -> &'static str{                                     // return the static pointer as the input, but will occupy the global memory all the time.[NOT RECOMMENDED]
    println!("{}", s);
    "hello5_returned"
}
let s5 = String::from("hello5");
let s5_returned = print_string_static(s5);
println!("s5_returned={}", s5_returned);


// Solution 3
fn print_string_static_2(s: String) -> String{                                         // Whatever the function is
    println!("{}", s);
    s
}

let s6 = String::from("hello6");
let s6_returned = print_string_static_2(s6.clone());                                   // Whatever the ownership of the variable is, it can be cloned and put in as other languages, while the original one preserved, though it hurt the performance sometimes.
println!("s6_returned={}", s6_returned);
println!("s6={}", s6);

fn first_word(s:&str) -> &str{
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[0..i];
        }
    }
    &s[..]
}
let back = first_word("Hello World");
println!("back={}", back);
```
### 3.1.4 Notes on Codes
**Points to Remember**  
1. `Copy` strategy for simple dtypes(**Stack Allocated Memory**)
    - int
    - float
    - char
    - bool
    - enum(According to whether there are complex dtype in it)
    - struct(According to whether there are complex dtype in it)
    - tuple(According to whether there are complex dtype in it)
    - array(According to whether there are complex dtype in it)
2. `Move` strategy for complex dtypes(**Heap Allocated Memory**)
    - string
    - vector
    - hashmap
    - 
3. Ownership is transferred when the variable is passed to a function and will be destroied with the unction call.
```rust
let s3 = String::from("hello3");
let ln = get_length(s3);
println!("s3 '{}'is not kept.", s3);                                                   // This will cause BorrowOfMovedValueError
```
4. How to Properly pass the ownership of a variable to a function
    - Using `.clone()` strategy(Easy and intuitive)
    ```rust
    fn example_function(s: Any) -> Any{
        // anything
    }
    let s = Any;
    let s_copy = example_function(s.clone());                                           // Using `.clone()` to copy the value(Memory occupacy doubled)
    ```
    - Using `borrow pointer` to share the value(**MOST RECOMMENDED & EFFICIENT**)
    ```rust
    fn example_function(s: &Any) -> Any{
        /// anything
        let s_real = s.to_owned();                                                     // Using `.to_owned()` to fetch the value

    }

    ```
    - Using `&'static` lifetime(**NOT RECOMMENDED**)
    ```rust
    fn example_function(s: &'static Any) -> Any{                                       // `s` will never be destroyed and hurt the performance
        /// anything
    }
    ```
## 3.2 Ownership of `String` & `&str`
### 3.2.1 `String` and `&str` Explanation
1. `String` is a **Heap Allocated Changable struct**, which is a **Dynamic Array**, which can be resized dynamically, whose essence is:
```rust
pub struct String {
    vec: Vec<u8>,
}
```
2. `&str` is a **Stack Allocated Unchangable Slice**, whose essence is a `borrow pointer`, pointng to the UTF-8 encoded `String` stored in memory, whose essence is a `pointer` and a `length`.
3. `String` has an ownership, while `&str` does not.
4. Using o `String`: When you need to occupy memory for a new String(like a property of a `struct`)
    - You can't use `&str` unless you claim a lifetime to it.
    - There will be other memory risks when you create multiple structs that contain the same `&str`
5. Using `&str`: When you need to pass the value of a **created** `String` and no need to pass the ownershp to something else(lke pass the value of a string to a function)
    - use `&str`, `&str` and `&String` can both be passed.
    - use `&String` only `&String` can be passed.
6. `&str` is a `Slice Borrow Pointer` and can pass the actual value of the String it point to(using `.to_owned()`), while `&String` is a `String Borrow Pointer` that can only pass the pointer of the String it point to.

