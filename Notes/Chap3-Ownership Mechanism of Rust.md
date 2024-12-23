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


