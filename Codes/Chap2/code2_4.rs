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
