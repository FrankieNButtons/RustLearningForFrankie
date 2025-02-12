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