    let a1 = 233;                                   // Defaultly int32
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


    let True = true;                                      // [warning] Snake case naming is recommanded
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
