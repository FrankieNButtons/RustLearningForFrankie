fn main() {                             // There are totally 5 warnings in it, if not, I should update my note.
    println!("Lesson 2.1");
    code2_1();                          // There should be 2 warnings in this, don't worry.
    println!("########################\n\n");
    println!("Lesson 2.2");
    code2_2();                          // There should be no warnings in this.
    println!("########################\n\n");
    println!("Lesson 2.3");
    code2_3();                          // There should be 3 warnings in this, don't worry.
    println!("########################\n\n");
    println!("Lesson 2.4");             // There should be no warnings in this.
    code2_4();
}
fn code2_1(){                           // About Variables
        let num: i32 = 100;             // [Warning]unused var without "_" will cause compiler's warning.
    let _mun: i64 = 54;                 // unused var with "_" won't cause compiler's warning.
    //num = 13;                         // defaultly immutable variable.

    let mut tum: i32 = 20;
    tum = tum + 1;                      // [Warning]variable initialized that receives excessive assignment will cause warning.
    // println!("tum={tum}");           // Uncomment the line to remove the warning.

    let x: i32 = 35;
    {
        let x: i32 = 10;
        println!("x={}", x);            // variable can be define multitimes in different effective realm.
    }
    println!("x={}", x);                // but only shadow the original variable in that realm, new variable destroied when out.

    let x: &str = "Hello";              // However, you can reclaim the value(even dtype) in the same realm.
    println!("The new `x` is '{}'", x);

    let mut x: u16 = 30;                // Even the immutability of a variable can be reclaimed.
    println!("Mutable `x` is {}", x);
    x = 45;
    println!("Changed `x` is {}", x);
}



static MY_STA: i32 = 42;
static mut MY_MUT_STA:i8 = 42;
fn code2_2(){
    const _SECONDS_PER_HOUR: usize = 3_600;              // Integer can be defined like this.
    const MONTH_PER_YEAR: usize = 12;
    const MONTH_PER_DECADE: usize = 10 * MONTH_PER_YEAR; // Computed out when compiling

    println!("A year has {} months.", MONTH_PER_YEAR);
    println!("A decade has {} months.", MONTH_PER_DECADE);
    {
        const TST:u32 = 1_000;
        println!("inner TST={}", TST);
    }
    // println!("outer TST={}", TST);                    // const only accessible in its defining realm.


    println!("MY_STA={}", MY_STA);
    unsafe {                                            // `Unsafe code` is not recommended in Rust.
        MY_MUT_STA = 37;
        println!("inner MY_MUT_STA={}", MY_MUT_STA);
        println!("inner MY_STA={}", MY_STA);            // Immutable static variables can be used in `unsafe code`
    }
    // println!("outer MY_MUT_STA={}", MY_MUT_STA);     // mutable static variables cannot be used in `safe code`
    println!("outer MY_STA={}", MY_STA);
}
fn code2_3(){
    let a1 = 233;                                       // Defaultly int32
    let a2: i64 = 0xFFFF;                               // start with '0x' to define a hex
    let a4: i16 = 0o666;                                // start with '0o' to define an octal
    let a3: i8 = 0b1111;                                // start with '0b' to define a binary
    println!("a1 = {a1}\na2 = {a2}\na3 = {a3}\na4 = {a4}");

    println!("Max of u32 is {}", u32::MAX);
    println!("Min of u32 is {}", u32::MIN);
    println!("Max of i64 is {}", i64::MAX);
    println!("Min of i64 is {}", i64::MIN);
    println!("Max of usize is {}", usize::MAX);

    println!("u32 ocps {} bytes", std::mem::size_of::<u32>()); // This is a Generics, explain in section
    println!("i8 ocps {} bytes", std::mem::size_of::<i8>());
    println!("isize ocps {} bytes", std::mem::size_of::<isize>());


    let f1: f32 = 3.1415926;
    let f2: f64 = 6.71828;
    // println!("Max of f32 is {}", f32::MAX);
    // println!("Min of f64 is {}", f64::MIN);
    println!("f1 = {:.2}\nf2 = {:.4}", f1, f2);                 // Float formatting in Rust need no sign of 'f'


    let True = true;                                            // [warning] Snake case naming is recommanded
                                                                // [warning] Capitalized reserved word not recommanded for variable, if necessary,  use 'raw identifier'
    let mut False: bool = false;                                // [Warning] Unchanged variable need no 'mut' declaration
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
    let char_emoji: char = '❤';                                     // Unicode Emoji is available for Rust.
    println!("I got {char_e}velyn and I feel {char_emoji}.");       // char can be directly printed.
}

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
