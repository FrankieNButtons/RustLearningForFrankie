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
