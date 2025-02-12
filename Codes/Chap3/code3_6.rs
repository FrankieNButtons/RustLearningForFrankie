// Box pointer
struct Point {
    x: i32,
    y: i32
}

let boxed_point = Box::new(Point{x: 1, y: 2});
println!("Point is at ({}, {}", boxed_point.x, boxed_point.y);                                                                       // Though Box is a pointer, it will automatically dereference when accessing its fields.

let mut boxed_num = Box::new(5);
println!("Boxed number is {}", *boxed_num);                                                                                          // Dereference the pointer manually with `*` like C/C++ to access the value.
// boxed_num += 1;                                                                                                                   // [Error] cannot assign to `*boxed_num` directly because it is a `&mut` reference
*boxed_num += 1;                                                                                                                     // Dereference the pointer manually with `*` to change the value.
println!("Boxed number is {}", *boxed_num);


// Copy/Move
let x = vec![1, 2, 3, 4];
let y = x.clone();                                                                                                         // Copy the value of a `heap-stored` variable with `.clone()`
println!("x is {:?}", x);
println!("y is {:?}", y);

struct Book1 {
    page: i32,
    price: f64
}

let str = "Hello, world!".to_string();
let str_copied = str.clone();
println!("string is {:?}", str);
println!("string copied is {:?}", str_copied);

let book = Book1{page: 100,price: 10.5};
// println!("book is {:?}", book);                                                                                                   // [Error] `Book1` doesn't implement the `Debug` trait
// let book_copied = book.clone();                                                                                                   // [Error] No method named `clone` found for struct `Book` in the current scope

#[derive(Clone, Debug)]                                                                                                              // Add `#[derive(Clone, Debug)]` to `Book2` to make it `Clone` and `Debug`
struct Book2 {
    page: i32,                                                                                                                       // [Warning] Struct printed with `{:?}` as `Debug`` is not considered as Fields Usage
    price: f64
}

let book = Book2{page: 100,price: 10.5};
println!("book is {:?}", book);
let book_copied = book.clone();
println!("book copied is {:?}", book_copied);