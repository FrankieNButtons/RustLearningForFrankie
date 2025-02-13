fn main() {
    println!("#############################Code3_0######################################");
    code3_0();                                      // There is no Warnings in this case
    println!("#############################Code3_1######################################");
    code3_1();                                      // There is no Warnings in this case
    println!("#############################Code3_2######################################");
    code3_2();                                      // There are 4 Warnings in this case, don't worry
    println!("#############################Code3_3######################################");
    code3_3();                                      // There is 1 Warning in this case, don't worry
    println!("#############################Code3_4######################################");
    code3_4();                                      // There is no Warnings in this case
    println!("#############################Code3_5######################################");
    code3_5();                                      // There is no Warnings in this case
    println!("#############################Code3_6######################################");
    code3_6();                                      // There are 3 Warnings in this case, don't worry
}
fn code3_0(){
    // For thouse variables that are not that complex, Rust use `Copy` Strategy
    let arr = [1, 3, 5];
    let tup = (15, "ok", 42, 666);
    println!("arr: {:?}", arr);
    println!("tup: {:?}", tup);

    let arr_ownership = arr;                                                           // ownership transfer
    let tup_ownership = tup;
    println!("arr_ownership: {:?}",  arr_ownership);                                           // the memory has been copied.(Not only apply the pointer to the variable)
    println!("tup_ownership: {:?}", tup_ownership);

    let a = 3;
    let b = a;                                                                           // Variable `a` is not covered by `b`
    println!("a: {}, b: {}", a, b);                                                           // Thus `a` can still be printed

    // But for those variables that are more complex, Rust use `Move` Strategy
    let str_1 = String::from("hello");
    let str_2 = str_1;
//  println!("str_1: {}", str_1);                                                          // [Error]The value of `str_1` has been moved to `str_2`, which will cause BorrowOfMovedValueError
    println!("str_2: {}", str_2);
}

fn code3_1(){
    // `Copy` on baic type
    let c1 = 1;
    let _c2 = c1;                                                                     // `Copy` on basic type
    println!("_c1={}", c1);
    // `Ownership borrowed`` on complex type
    let s1 = String::from("hello");
    let _s2 = s1;                                                                  // `Move` the ownership of `s1` to `s2`
    // println!("_s1={}", s1);                                                             // [Error]The ownership of value in `s1` has been moved to `s2`, which will cause BorrowOfMovedValueError
    println!("_s2={}", _s2);
    let s2 = _s2.clone();                                                          //  `Clone` on complex type
    println!("s2={}", s2);
    println!("_s2={}", _s2);

    fn get_length(s: String)-> usize{      
        println!("Length of {}, is {}", s, s.len());
        s.len()                                                                               // No `;` to return and claim the dtype on annotation
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
    let s6_returned = print_string_static_2(s6.clone());                           // Whatever the ownership of the variable is, it can be cloned and put in as other languages, while the original one preserved, though it hurt the performance sometimes.
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
}

fn code3_2(){
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
}

fn code3_3(){
    enum Color{                                                                            // Type unclaimed `enum`
        Red,
        Blue,
        Green,
        Black
    }
    fn print_color(my_color: Color) {
        match my_color {
        Color::Red => println!("Red"),                                                     // [Warning]Used variants will cause Warning
        Color::Blue => println!("Blue"),
        Color::Green => println!("Green"),
        _ => println!("Others")                                                            // All variants need to be covered, comment this line out to cause error.
        }
    }
    print_color(Color::Blue);
    print_color(Color::Black);

    enum BuildingLocation {
        Number(i32),
        // Name(&str),                                                                     // `&str` is not recommended, for it need to have a lifetime specifier.
        Name(String),
        Unknown                                                                            // `Unknown` can 
    }

    // `impl` is used to implement a function for a type( function oriented, not object oriented), and with `&self` param to declare the function for an instance
    impl BuildingLocation {
        fn get_location(&self) -> String {
            match self {
                BuildingLocation::Number(n) => format!("Number {}", n),
                BuildingLocation::Name(s) => format!("Name {}", s),
                BuildingLocation::Unknown => format!("Unknown")
            }
        }
    }

    let location1 = BuildingLocation::Number(1);
    let location2 = BuildingLocation::Name("Test Place".to_string());
    let location3 = BuildingLocation::Unknown;
    println!("location1 = {}", location1.get_location());                                  // call the function with `.` on the instance
    println!("location2 = {}", location2.get_location());
    println!("location3 = {}", location3.get_location());

    enum BuildingLocation2 {
        Number(i32),
        Name(String),
        Unknown
        }
    
    // `impl` is used to implement a function for a type( function oriented, not object oriented), and with any other param to declare the function for the type itself
    impl BuildingLocation2 {
        fn get_location2(location: &BuildingLocation2) -> String {
            match location {
                BuildingLocation2::Number(n) => format!("Number {}", n),
                BuildingLocation2::Name(s) => format!("Name {}", s),
                BuildingLocation2::Unknown => format!("Unknown Location")
            }
        }
    }

    let location1 = BuildingLocation2::Number(2);
    let location2 = BuildingLocation2::Name("Test Place".to_string());
    let location3 = BuildingLocation2::Unknown;
    println!("location1 = {}", BuildingLocation2::get_location2(&location1));                                                            // call the function with `::` on the type
    println!("location2 = {}", BuildingLocation2::get_location2(&location2));
    println!("location3 = {}", BuildingLocation2::get_location2(&location3));
}
fn code3_4(){
    struct Person {
        name: String,
        age: u8
    }
    impl Person {
        fn new(name: String, age: u8) -> Self {
            Person{
                name,
                age
            }
        }
        fn is_older_than(&self, other: &Person) -> bool {                                                                                // `Associated function` for instance
            self.age > other.age
        }
        fn is_older_than_static(person1: &Person, person2: &Person) -> bool {                                                            // `Associated function` for type
            person1.age > person2.age
        }

        fn is_adult(&self) -> bool {
            self.age >= 18
        }

        fn introduce(&self) {
            println!("My name is {} and I am {} years old.", self.name, self.age);
        }
    }

    let person1 = Person{                                                                                                        // create an instance of `Person` in struct style
        name: "John".to_string(),
        age: 17
    };
    let person2 = Person::new("Nancy".to_string(), 20);                                                                // create an instance of `Person` in function style
    person1.introduce();
    person2.introduce();
    println!("Is {} older than {}? {}", person1.name, person2.name, person1.is_older_than(&person2));                                    // `.` is used to call the function on the instance
    println!("Is {} older than {}? {}", person2.name, person1.name, Person::is_older_than_static(&person2, &person1));                   // `::` is used to call the function on the type
    println!("Is {} an adult? {}", person1.name, person1.is_adult());
    println!("Is {} an adult? {}", person2.name, person2.is_adult());
}

fn code3_5(){
    struct Counter {
        count: u32
    }

    impl Counter {
        fn new(init: u32) -> Counter {
            println!("Create counter at {} by `new`", init);
            Counter{
                count: init
            }
        }
        fn get_count(&self) -> u32 {                                                                                                     // Immutable borrowing
            self.count
        }
        fn add(&mut self, num: u32) {                                                                                                    // Mutable borrowing
            println!("Add {} to counter at {} by `add`", num, self.count);
            self.count += num;
        }
        fn combine(self: Self, other: Self) -> Self {                                                                                    // Move Ownership
            println!("Combine counter at {} and {} by `combine`", self.count, other.count);
            Counter{
                count: self.count + other.count
            }
        }
        fn free_counter(self) {                                                                                                          // Muve Ownership
            println!("Free counter at {} by `free_counter`", self.count);
        }
    }

    let mut counter1 = Counter::new(0);
    println!("Counter1: {}", counter1.get_count());
    let counter2 = Counter::new(counter1.get_count());
    counter1.add(1);
    println!("Counter1: {}", counter1.get_count());
    println!("Counter2: {}", counter2.get_count());
    counter1 = counter1.combine(counter2);
    counter1.free_counter();
    // println!("Counter1: {}", counter1.get_count());                                                                                   // [Error]counter1's ownership has been moved to counter1.free_counter() and ropped, uncomment to see the error
    // println!("Counter2: {}", counter2.get_count());                                                                                   // counter2's ownership has been moved to counter1.combine() and ropped, uncomment to see the error
}

fn code3_6(){
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
}
