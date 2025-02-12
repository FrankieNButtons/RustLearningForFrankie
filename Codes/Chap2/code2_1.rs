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