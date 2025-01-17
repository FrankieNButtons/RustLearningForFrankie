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