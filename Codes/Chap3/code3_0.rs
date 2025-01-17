// For thouse variables that are not that complex, Rust use `Copy` Strategy
let arr = [1, 3, 5];
let tup = (15, "ok", 42, 666);
println!("arr: {:?}", arr);
println!("tup: {:?}", tup);

let arr_ownership = arr;                                                                  // ownership transfer
let tup_ownership = tup;
println!("arr_ownership: {:?}",  arr_ownership);                                          // the memory has been copied.(Not only apply the pointer to the variable)
println!("tup_ownership: {:?}", tup_ownership);

let a = 3;
let b = a;                                                                                // Variable `a` is not covered by `b`
println!("a: {}, b: {}", a, b);                                                           // Thus `a` can still be printed

// But for those variables that are more complex, Rust use `Move` Strategy
let str_1 = String::from("hello");
let str_2 = str_1;
//  println!("str_1: {}", str_1);                                                         // [Error]The value of `str_1` has been moved to `str_2`, which will cause BorrowOfMovedValueError
println!("str_2: {}", str_2);