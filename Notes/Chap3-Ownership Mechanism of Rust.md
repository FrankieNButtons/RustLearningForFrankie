# <center> Chapter2: Ownership Machanism of Rust </center>
## 3.0 General Previews of Ownership
### Demo Codes
```rust
// For thouse variables that are not that complex, Rust use `Copy` Strategy
let arr = [1, 3, 5];
let tup = (15, "ok", 42, 666);
println!("arr: {:?}", arr);
println!("tup: {:?}", tup);

let arr_ownership = arr;                                                                  // ownership transfer
let tup_ownership = tup;
println!("arr_ownership: {:?}", arr_ownership);                                           // the memory has been copied.(Not only apply the pointer to the variable)
println!("tup_ownership: {:?}", tup_ownership);

let a = 3;
let b = a;                                                                                // Variable `a` is not covered by `b` 
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
1. **Memory Leak**: Memory that is allocated but not freed (e.g., `int* str = new int;`).
   - Solution: `delete str;`
   
2. **Dangling Pointer**: A pointer that points to memory that has been freed (e.g., `int* str = new int; delete str; int* str2 = str;`).
   - Solution: Avoid using the pointer after memory is freed. Ensure `delete` is done only once.

3. **Double Free**: Memory that is freed twice (e.g., `int* str = new int; delete str; delete str;`).
   - Solution: Make sure to `delete` the memory only once.

4. **Array out of bounds**: Trying to access an array outside of its bounds (e.g., `int* str = new int[10]; str[10] = 1;`).
   - Solution: Ensure that array access is within bounds (e.g., `str[9] = 1;`).

5. **Use after free**: Memory that is freed but still used (e.g., `int* str = new int; delete str; str[0] = 1;`).
   - Solution: Avoid using a pointer after `delete` is called.

6. **Stack overflow**: The stack overflows due to excessive memory allocation (e.g., `int* str = new int[100000000];`).
   - Solution: Use smaller allocations (e.g., `int* str = new int[100];`).

7. **Wild pointer**: A pointer that is not initialized (e.g., `int* str; str[0] = 1;`).
   - Solution: Always initialize pointers (e.g., `int* str = new int; str[0] = 1;`).

8. **Ambiguous usage of `new/delete` and `malloc/free`**: Mixing memory management methods, such as using `new/delete` with `malloc/free` (e.g., `int* str = new int; free(str);`).
   - Solution: Stick to one method of memory management. If using `malloc`, use `free`, and if using `new`, use `delete`.

### 3.1.2 Memory Management Mechanism in Rust
1. **Borrowing**: Rust uses borrowing to allow multiple references to a value without taking ownership. There are two types of borrowing:
   - **Immutable Borrowing**: `&T`, which allows shared access to a value without taking ownership.
   - **Mutable Borrowing**: `&mut T`, which allows exclusive access to modify a value.
   
2. **Lifetime**: Rust enforces lifetimes to ensure that memory is properly freed when no longer in use. When a variable goes out of scope, its memory is freed.

3. **Reference Counting**: Rust uses reference counting through `Rc` and `Arc` for memory management. The memory is freed when the reference count drops to zero. This happens at compile time.

### 3.1.3 Demo Codes
```rust
fn code3_1() {
    // `Copy` on basic types
    let c1 = 1;
    let _c2 = c1; // `Copy` on basic types
    println!("_c1={}", c1);

    // `Ownership borrowed` on complex types
    let s1 = String::from("hello");
    let _s2 = s1; // Ownership of `s1` moved to `_s2`
    // println!("_s1={}", s1); // [Error] Ownership moved, cannot use `s1`
    println!("_s2={}", _s2);

    let s2 = _s2.clone(); // `Clone` on complex types
    println!("s2={}", s2);
    println!("_s2={}", _s2);

    fn get_length(s: String) -> usize {
        println!("Length of {}, is {}", s, s.len());
        s.len() // No `;` to return and claim the dtype on annotation
    }

    let s3 = String::from("hello3");
    let ln = get_length(s3);
    // println!("s3 '{}' is not kept.", s3); // Ownership moved to function
    println!("length {} of s3 is returned.", ln);

    // Solution 1: Returning ownership
    fn print_string(s: String) -> String {
        println!("{}", s);
        s.to_owned() // Return the ownership of the String
    }

    let s4 = String::from("hello4");
    let s4_copy = print_string(s4);
    println!("s4_copy={}", s4_copy);

    // Solution 2: Static pointer (not recommended)
    fn print_string_static(s: String) -> &'static str {
        println!("{}", s);
        "hello5_returned" // Static return value, not recommended
    }

    let s5 = String::from("hello5");
    let s5_returned = print_string_static(s5);
    println!("s5_returned={}", s5_returned);

    // Solution 3: Cloning the value
    fn print_string_static_2(s: String) -> String {
        println!("{}", s);
        s // Returning ownership back
    }

    let s6 = String::from("hello6");
    let s6_returned = print_string_static_2(s6.clone()); // Clone to preserve original
    println!("s6_returned={}", s6_returned);
    println!("s6={}", s6);

    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        &s[..]
    }

    let back = first_word("Hello World");
    println!("back={}", back);
}
```

### 3.1.4 Notes on Codes
**Warnings 2B Concerned**
1. **Ownership Move**: When ownership of a value is moved (like `s1` to `s2`), the original variable is no longer accessible, which will lead to a **BorrowOfMovedValueError**.
   - Ensure that you don't try to use a variable after its ownership has been moved.

2. **Cloning**: When you need to keep a copy of a value, use `.clone()` to avoid ownership transfer. However, cloning can affect performance, especially for large data structures.

3. **Static Return Values**: Returning static values (`'static str`) is not recommended because it can lead to memory management issues as the string is stored in global memory.

**Points 2B Remembered**
1. **Borrowing**: Rust allows both immutable and mutable borrowing of values. With borrowing, you can have multiple references to a value, but only one mutable reference at a time.
   
2. **Ownership and Cloning**: Ownership of data is transferred when it is moved (e.g., `let s2 = s1;`). You can prevent this by cloning the data if you need to keep a copy (e.g., `let s2 = s1.clone();`).
   
3. **Function Ownership**: Functions can take ownership of parameters (e.g., `fn get_length(s: String)`), and Rust will automatically manage the cleanup when the function exits, preventing memory leaks.

4. **Lifetimes**: Lifetimes in Rust ensure that references are valid for the correct duration, preventing dangling pointers and other memory issues.

5. **Static References**: Use caution when returning references with `'static` lifetime as it can cause issues with memory management. Consider using ownership transfer or cloning instead.

6. **Memory Safety**: Rust's ownership model ensures memory safety without the need for a garbage collector.

## 3.2 Ownership of `String` & `&str`
### 3.2.1 `String` and `&str` Explanation
1. `String` is a **Heap Allocated Changable struct**, which is a **Dynamic Array**, whose essence is:
```rust
pub struct String {
    vec: Vec<u8>,
}
```
2. `&str` is a **Stack Allocated Unchangable Slice**, whose essence is a `borrow pointer`, pointing to the UTF-8 encoded `String` stored in memory, whose essence is a `pointer` and a `length`.
3. `String` has an ownership, while `&str` does not.
4. Using `String`: When you need to occupy memory for a new String (like a property of a `struct`)
    - You can't use `&str` unless you claim a lifetime to it.
    - There will be other memory risks when you create multiple structs that contain the same `&str`
5. Using `&str`: When you need to pass the value of a **created** `String` and no need to pass the ownership to something else (like passing the value of a string to a function)
    - use `&str`, `&str` and `&String` can both be passed.
    - use `&String` only `&String` can be passed.
6. `&str` is a `Slice Borrow Pointer` and can pass the actual value of the String it points to (using `.to_owned()`), while `&String` is a `String Borrow Pointer` that can only pass the pointer of the String it points to.

### 3.2.2 Demo Codes
```rust
fn code3_2(){
    // `Copy` on basic type
    let c1 = 1;
    let _c2 = c1;                                     
    println!("_c1={}", c1);

    // `Ownership borrowed` on complex type
    let s1 = String::from("hello");
    let _s2 = s1;                               
    println!("_s2={}", _s2);
    let s2 = _s2.clone();                      
    println!("s2={}", s2);
    println!("_s2={}", _s2);

    fn get_length(s: String)-> usize{      
        println!("Length of {}, is {}", s, s.len());
        s.len()
    }

    let s3 = String::from("hello3");
    let ln = get_length(s3);
    println!("length {} of s3 is returned.", ln);

    // Solution 1
    fn print_string(s: String) -> String{                                              
        println!("{}", s);
        s.to_owned()
    }
    let s4 = String::from("hello4");
    let s4_copy = print_string(s4);
    println!("s4_copy={}", s4_copy);

    // Solution 2
    fn print_string_static(s: String) -> &'static str{                                     
        println!("{}", s);
        "hello5_returned"
    }
    let s5 = String::from("hello5");
    let s5_returned = print_string_static(s5);
    println!("s5_returned={}", s5_returned);

    // Solution 3
    fn print_string_static_2(s: String) -> String{                                         
        println!("{}", s);
        s
    }
    let s6 = String::from("hello6");
    let s6_returned = print_string_static_2(s6.clone());                           
    println!("s6_returned={}", s6_returned);
    println!("s6={}", s6);

    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        &s[..]
    }
    let back = first_word("Hello World");
    println!("back={}", back);

    // Convert between `&str` and `String`
    let s1 = String::from("Vlue C++");
    let s2 = "Rust".to_owned();
    let s1_replaced = s1.replace("C++", "Rust");
    println!("{s1},{s2},{s1_replaced}");

    let rust = "\x52\x75\x73\x74";
    println!("`rust` = {:?}", rust);

    let color = "black".to_string();
    let name = "Vlue".to_string();

    struct Person {
        name: String,
        color: String,
        age: u8,
    }
    let _people = Person {
        name,
        color,
        age: 18,
    };

    struct Person2<'a> {
        name: &'a str,
        color: &'a str,
        age: u8,
    }
    let _people2 = Person2 {
        name: "John",
        color: "black",
        age: 19,
    };
}
```

### 3.2.3 Notes on Codes
**Warnings 2B Concerned**
1. Moving ownership from one variable to another makes the original variable inaccessible. For example:
   ```rust
   let s1 = String::from("hello");
   let _s2 = s1; // Ownership of `s1` is moved to `_s2`.
   // println!("{}", s1); // [Error] Borrow of moved value
   ```
2. Structs containing `&str` require a lifetime specifier to avoid compile-time errors. For example:
   ```rust
   struct Person2<'a> {
       name: &'a str,
       color: &'a str,
       age: u8,
   }
   ```

**Points 2B Remembered**
1. `String` can be cloned using `.clone()`, and the cloned variable retains the original data. For example:
   ```rust
   let s2 = _s2.clone();
   println!("s2={}", s2);
   println!("_s2={}", _s2);
   ```

2. Ownership can be passed to functions and returned to maintain control of the value. For example:
   ```rust
   fn print_string(s: String) -> String {
       println!("{}", s);
       s.to_owned()
   }
   let s4_copy = print_string(s4);
   println!("s4_copy={}", s4_copy);
   ```

3. Functions can return a static reference when global memory is required, but this is not recommended. For example:
   ```rust
   fn print_string_static(s: String) -> &'static str {
       println!("{}", s);
       "hello5_returned"
   }
   ```

4. Slicing strings with `&str` is efficient and can return substrings using ranges. For example:
   ```rust
   fn first_word(s: &str) -> &str {
       let bytes = s.as_bytes();
       for (i, &item) in bytes.iter().enumerate() {
           if item == b' ' {
               return &s[0..i];
           }
       }
       &s[..]
   }
   ```

5. Structs can hold either `String` or `&str`, but `&str` requires careful handling with lifetime specifiers to avoid dangling references.

## 3.3 Ownership and Usage of `enum` & `match`
### 3.3.1 `enum` Explanation
1. `enum` is a data type that can be used to represent a set of possible discrete values, each named as a `variant`.
2. The calling of `enum` variants is done with `EnumName::VariantName`.
3. Using `enum` makes programs more robust and readable.
4. Example definition of `enum`:
   ```rust
   pub enum Shape{
     Square(f64),
     Rectangle(f64, f64),
     Circle(f64)
   }
   ```
5. Common `enum` types: `Option<T>` (function return) and `Result<T, E>`.
   ```rust
   pub enum Option<T>{
       None,
       Some(T)
   }
   pub enum Result<T, E>{
       Ok(T),
       Err(E)
   }
   ```

### 3.3.2 `match` Explanation
1. `match` is a powerful pattern matching tool to match the value of a variable.
2. It is defined using the `match` keyword.
3. All variants of an `enum` need to be covered, either explicitly or using a catch-all (`_`).
4. You can use various match patterns: `_`, `|`, `..=`, `==`, `=>`.
5. Example definition of `match`:
   ```rust
   match number{
       0 => println!("zero"),
       1 | 2 => println!("one or two"),
       3 ..= 9 => println!("three to nine"),
       n if n % 2 == 0 => println!("even"),
       _ => println!("something else")
   }
   ```

### 3.3.3 Little Tip on `impl`
1. `impl` is used to define `associated functions` or methods for a data type.
2. To define a function for a type, you need to declare it as a function on the type itself.
   - For a type function, use `type::func`.
3. To define a method for an instance of a type, use `&self`, and call it via `instance.func`.

### 3.3.4 Demo Codes
```rust
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
    println!("location1 = {}", location1.get_location());
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
    println!("location1 = {}", BuildingLocation2::get_location2(&location1));
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
        fn is_older_than(&self, other: &Person) -> bool {
            self.age > other.age
        }
        fn is_older_than_static(person1: &Person, person2: &Person) -> bool {                                                            
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
    let person2 = Person::new("Nancy".to_string(), 20);
    person1.introduce();
    person2.introduce();
    println!("Is {} older than {}? {}", person1.name, person2.name, person1.is_older_than(&person2));
    println!("Is {} older than {}? {}", person2.name, person1.name, Person::is_older_than_static(&person2, &person1));
    println!("Is {} an adult? {}", person1.name, person1.is_adult());
    println!("Is {} an adult? {}", person2.name, person2.is_adult());
}
```

### 3.3.5 Note on Codes
**Warnings 2B Concerned**
1. When using `match`, make sure to cover all variants of the `enum`. If you miss a variant, the compiler will warn or error.
   - For example, the `_` catch-all ensures that unmatched variants are handled.
   - If `_` is omitted, it can lead to errors or unhandled cases.
2. Using `&str` in `enum` variants requires careful attention to lifetimes. It’s better to use `String` instead of `&str` to avoid lifetime management issues.
   - Example:
   ```rust
   enum BuildingLocation {
       Name(String),
       // Name(&str), // `&str` would require a lifetime specifier
   }
   ```

**Points 2B Remembered**
1. `match` allows flexible and powerful pattern matching, including ranges (`..=`), multiple patterns (`|`), and guards (`if` conditions).
   - Example:
   ```rust
   match number {
       1 | 2 => println!("one or two"),
       3 ..= 9 => println!("three to nine"),
       n if n % 2 == 0 => println!("even"),
       _ => println!("something else"),
   }
   ```
2. `impl` is used to define methods for a type. Methods that depend on the instance of a type use `&self`, whereas functions that depend on the type itself use the type name.
   - Example of instance method:
   ```rust
   fn get_location(&self) -> String { ... }
   ```
   - Example of type function:
   ```rust
   fn get_location2(location: &BuildingLocation2) -> String { ... }
   ```
3. Using `enum` with `match` ensures that all possible cases are covered, making your code more predictable and less error-prone.
4. `String` is preferred over `&str` in `enum` variants when there’s no need for complex lifetime management.

## 3.4 Usage of `struct`
### 3.4.1 `struct` Explanation
1. `struct` is a user-defined data type that groups together multiple values of different types, which are called `fields`.
2. Values in a `struct` can be accessed using the `.` operator.
   Example:
   ```rust
   struct Point {
     x: f64,
     y: f64
   }
   ```
3. Rust is primarily a **function-oriented** language, and the functions you define for a `struct` are called **associated functions**. While Rust doesn't define a `method` as in object-oriented languages, it is common to refer to functions defined on instances of structs as methods.
4. **Method**: A method is an associated function that is defined for an instance with `&self` as a parameter. You can call it via `instance.method`.
   Example:
   ```rust
   impl Point {
     fn get_distance(&self, other: Point) -> f64 {
       // some logic here
     }
   }
   // Calling method
   point1.get_distance(point2);
   ```
5. **Associated Function**: A function that is defined for a `struct` type and doesn't use `&self`. You call it using `Type::func`.
   Example:
   ```rust
   impl Point {
     fn new(x: f64, y: f64) -> Self {
       Point { x, y }
     }
   }
   // Calling associated function
   Point::new(1.0, 2.0);
   ```
6. **Associated Variable**: You can define constants or static variables inside an `impl` block for a type, and access them via `Type::var`.
   Example:
   ```rust
   impl Point {
     const PI: f64 = 3.1415926;
   }
   // Accessing associated variable
   Point::PI;
   ```
7. **`self` vs `Self`**:
   - `self` is a reference to the instance of the type and is used when defining methods.
   - `Self` refers to the type itself (the struct) and is used when defining associated functions or variables.

### 3.4.2 Demo Codes
```rust
struct Person {
    name: String,
    age: u8
}

impl Person {
    fn new(name: String, age: u8) -> Self {
        Person {
            name,
            age
        }
    }

    fn is_older_than(&self, other: &Person) -> bool { // `Method` for instance
        self.age > other.age
    }

    fn is_older_than_static(person1: &Person, person2: &Person) -> bool { // `Associated function` for type
        person1.age > person2.age
    }

    fn is_adult(&self) -> bool {
        self.age >= 18
    }

    fn introduce(&self) {
        println!("My name is {} and I am {} years old.", self.name, self.age);
    }
}

let person1 = Person { // Creating an instance in struct style
    name: "John".to_string(),
    age: 17
};
let person2 = Person::new("Nancy".to_string(), 20); // Creating an instance in function style

person1.introduce();
person2.introduce();

println!("Is {} older than {}? {}", person1.name, person2.name, person1.is_older_than(&person2)); // Calling instance method
println!("Is {} older than {}? {}", person2.name, person1.name, Person::is_older_than_static(&person2, &person1)); // Calling type function
println!("Is {} an adult? {}", person1.name, person1.is_adult());
println!("Is {} an adult? {}", person2.name, person2.is_adult());
```

### 3.4.3 Note on Codes
**Points 2B Remembered**
1. **Methods** are defined with `&self` as the first parameter, allowing them to operate on instances of the `struct`. These are called via `instance.method()`.
   - Example: `person1.introduce()`
   
2. **Associated functions** are defined without `&self` and are typically used to operate on the type itself rather than its instances. These are called via `Type::func()`.
   - Example: `Person::new("Nancy", 20)`
   
3. **Associated variables** like constants can also be defined inside the `impl` block and accessed with `Type::var`.
   - Example: `Point::PI`

4. **`self` vs `Self`**:
   - `self`: Used in instance methods to refer to the current instance of the type.
   - `Self`: Used to refer to the type itself in associated functions or variables.
   
5. When creating instances of `struct`:
   - You can use direct assignment (e.g., `let person1 = Person { name: "John".to_string(), age: 17 };`).
   - Alternatively, you can use associated functions like `Person::new()` to create new instances.
   
6. Methods and associated functions can be used to organize logic and behavior around your data types, improving modularity and readability in your Rust programs.

## 3.5 Ownerhip of `struct`
### 3.5.1 General Knowledges
1. **Ownership Rules**:
    - Each value in Rust has an owner.
    - There can only be one owner at a time.
    - Values are automatically dropped when the owner goes out of scope.

2. **Value Passing Semantics**:
    - **Immutable Borrow**: The ownership remains with the original variable, and the receiver gets a reference to the value (`borrow`) rather than a copy.
    - **Mutable Borrow**: The ownership is still with the original variable, but the receiver can modify the value through the borrow. A mutable borrow must be unique.
    - **Move**: Ownership is transferred to the receiver variable, and the original variable no longer has access to the value.

3. **Borrowing in Structs**:
    - **Immutable Borrow**: `&self` (e.g., `self: &self`).
    - **Mutable Borrow**: `&mut self` (e.g., `self: &mut self`).
    - **Move**: Ownership is transferred via `self` (e.g., `self: self`).
    - Example of move in implementation:
      ```rust
      impl MyStruct {
          fn func(self: Self) -> DType {
              // Function logic here
          }
      }
      ```

4. Once ownership is used in a function (e.g., `MyStruct::func(instance)`), the `self` parameter loses ownership, and the instance can no longer be used.

### 3.5.2 Demo Codes
```rust
fn code3_5(){
    struct Counter {
        count: u32
    }

    impl Counter {
        fn new(init: u32) -> Counter {
            println!("Create counter at {} by `new`", init);
            Counter {
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
            Counter {
                count: self.count + other.count
            }
        }

        fn free_counter(self) {                                                                                                          // Move Ownership
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

    // println!("Counter1: {}", counter1.get_count());                                                                                   // [Error] Ownership moved to `free_counter` and dropped.
    // println!("Counter2: {}", counter2.get_count());                                                                                   // Ownership moved to `combine` and dropped.
}
```

### 3.5.3 Note on Codes
**Points 2B Remembered**
1. **Immutable Borrow**: Use `&self` to borrow a reference without transferring ownership.
   - Example:
     ```rust
     fn get_count(&self) -> u32 {
         self.count
     }
     ```
   - This allows access to the instance without changing its ownership.

2. **Mutable Borrow**: Use `&mut self` to borrow a mutable reference, enabling modifications to the instance.
   - Example:
     ```rust
     fn add(&mut self, num: u32) {
         self.count += num;
     }
     ```
   - Only one mutable borrow is allowed at a time.

3. **Move Ownership**: Use `self` to transfer ownership of the instance. Once moved, the instance cannot be used again.
   - Example:
     ```rust
     fn free_counter(self) {
         println!("Free counter at {}", self.count);
     }
     ```

4. **Ownership Transfer in Combine**: When combining two instances, ownership of both instances is transferred, and a new instance is returned.
   - Example:
     ```rust
     fn combine(self: Self, other: Self) -> Self {
         Counter { count: self.count + other.count }
     }
     ```

5. **Creating Struct Instances**:
   - Instances can be created with either a constructor function (`Counter::new`) or directly using the struct definition.
   - Example:
     ```rust
     let counter1 = Counter { count: 0 };
     let counter2 = Counter::new(10);
     ```

6. **Dropped Ownership**:
   - Ownership of an instance is dropped after being passed to a function that takes ownership (e.g., `self` parameter).
   - Uncommenting lines attempting to use dropped instances will result in compiler errors:
     ```rust
     // println!("Counter1: {}", counter1.get_count()); // Error: Ownership moved and dropped
     ```
## 3.6 Stack/Heap & Copy/Move
### 3.6.1 Stack/Heap
1. **Stack**:
    - A data structure that follows the Last-In-First-Out (LIFO) principle.
    - Efficient for small data I/O.
    - Stores data in a contiguous block of memory.
    - All the data stored on the stack must a  Explicit size.
    - For example,  The function scope is stored on the stack.
2. **Heap**:
    - Less regular than stack, when putting data nto a heap, the memory allocator will return a pointer to the location where the data will be stored on the memory.
    - Inefficient for data I/O.
    - Stores data in a non-contiguous block of memory.
    - The size of the data is not known at compile time.
3. **Box Pointer**:
    - A **smart** pointer that points to a value on the `heap`, and is automatically freed when no longer in use,  And can avoid Mememory management problems like `Dangling Pointer` or `Doule Free`.
    - It is a `reference` to a value on the `heap` that is owned by the variable it is bound to, or concisely,  Provide the `ownership` to the data that are allocated on heap.
    -  It can keep the uniqueness of the ownership when copy or move a value across variables.
    - It controls:
       1.  Ownership Movement
       2.  Automatic Memory Management
       3.  Dereferencing
       4.   Construct Data Sturcture with Recursion
### 3.6.2 Copy/Move
1. **Clone** is a deepcopy operation, which means it will copy the data from the original variable to a new one,  and increases the memory occupation.
2. **Copy** is a marker trait based on `clone`, 
3. **Move**  Is the  transfer of the ownership of the variable,  Which do, which does not increase the memory occupation.
4. **trait** is is a mechanism for defining the operations of memory sharing, clone is a `trait` as well.
5. **marker trait** is a trait without any method, only passes a message to compiler to change he defalt operation of a type. (Similar to `inherit` in object oriented programming)
6. dtypes stored on stack:
    - basic dtypes:  `i32`, `f32`, `bool`, `char`, `str`, `()`, `[T; N]`, `&T`, `&mut T`, `*T`, `*const T`, `*mut T`, `Box<T>`
    - `tuple`/`array`
    - struct/enum that have no properties that stored on heap(like `String`)
7. dtypes stored on heap:
    - Box
    - String
    - Rc
    - Vec
    - ···
8. **Copy** strategy is defaultly applied to the `stack-stored dtypes`, but `struct` & `enum` are not.
9. And if we want to use `Copy` strategy for `struct` & `enum`, we need to implement the `Copy` trait manually like adding `#[derive(Copy, Clone)]` to the struct/enum definition or applied `.clone()` manually.
### 3.6.3 Demo Code
```rust
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
```
### 3.6.4 Notes on Codes
**Warnings 2B Concerned**
1. **Box Pointer Dereferencing**:
    - Boxed values are automatically dereferenced when accessing their fields, so you don't need to manually dereference unless needed (e.g., when performing mutable operations on a Boxed value).

2. **Cloning and Copying**:
    - `vec.clone()` performs a deep copy, creating a new vector on the heap.
    - Ensure that you implement `Clone` and `Debug` for custom types if you want to use methods like `clone()` and print them with `{:?}`.

**Points 2B Remembered**
1. **Box Pointer**:
   - A `Box` pointer manages ownership of heap-allocated data and automatically cleans up when it goes out of scope.
   - You can dereference a `Box` pointer using `*`, but remember, `Box<T>` only implements mutable references, so be cautious when trying to modify it directly.

2. **Copy vs. Clone**:
   - `Copy` applies to simple types that can be duplicated easily, like integers or booleans. For heap-allocated types, such as vectors, you must use `.clone()` to make a deep copy.
   - `Clone` is a trait that creates deep copies, while `Copy` provides a shallow copy.
   - Remember to derive `Copy` and `Clone` for structs/enums if needed, as they are not implemented by default.