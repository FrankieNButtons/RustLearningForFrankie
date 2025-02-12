// convert between `&str` and `String`
// and there are 3 ways to create a string: `String::from("")`, `"".to_owned()`, `"".to_string()`
let s1 = String::from("Vlue C++");
let s2 = "Rust".to_owned();
let s1_replaced = s1.replace("C++", "Rust");
println!("{s1},{s2},{s1_replaced}");



let rust = "\x52\x75\x73\x74";
println!("`rust` = {:?}", rust);

let color = "black".to_string();
let name = "Vlue".to_string();

struct Person{
    name: String,
    color: String,
    age: u8
};
let _people = Person{
    name: name,
    color: color,
    age: 18
};

// struct Person2{
//     name: &str,                                                                     // `Missing lifetime Specifier` Error
//     color: String,
//     age: u8
// };

struct Person2<'a>{                                                                    // `Lifetime Specifier` claimed for `&str` in `struct`
    name: &'a str,                                                                     // Unused properties in `struct` will cause ``
    color: &'a str,
    age: u8
};
let _people2 = Person2{
    name: "John",
    color: "black",
    age: 19
};