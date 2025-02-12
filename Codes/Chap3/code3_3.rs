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