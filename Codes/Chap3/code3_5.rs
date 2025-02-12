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