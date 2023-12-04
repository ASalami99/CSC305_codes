use core::cmp::Ordering;

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}
#[derive(Debug)]
struct Rectangle {
    top_right: Point,
    bottom_left: Point,
}

struct Pair(u8, f32);

fn ru() {
    //declare a variable of type Person and assign values.
    let var_declaration = Person {
        name: String::from("Abdullah"),
        age: 19,
    };
    println!("{:#?}", var_declaration); //{:#?} implies pretty debug print person. :? is for debug print and :#? is for pretty debug print

    let declared_point = Point { x: 4.6, y: 6.7 };
    println!("{:#?}", declared_point);
    //println!("The x coordinate of the point is {:#?} and the y coordinate of the point  is {:#?}", declared_point.x, declared_point.y);

    // Make a new point by using struct update syntax to use the fields of our other one
    //let bottom_left = Point{x: 3.5, y: 5.4};
    //println!("The coordinates of the bottom left point are {:?} ", bottom_left)
    let top_right = Point {
        x: 5.2,
        ..declared_point
    };
    let bottom_left = Point {
        x: (5.6),
        ..declared_point
    };
    println!("Top Right: {:#?}", top_right);

    // Destructure the point using a let binding.
    let Point {
        x: left_edge,
        y: top_edge,
    } = declared_point;
    dbg!(&left_edge, &top_edge);

    let _rectangle = Rectangle {
        top_right: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_left,
    };
    println!("destructred rectangle {:#?}", _rectangle);

    let pair = Pair(1, 0.1);
    //Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    //Destructure a tuple struct
    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);
}

trait Shape {
    fn new(length: f32, width: f32, name: &'static str) -> Self;  //Changed all datatypes from i32 to f32 so that
    fn area(&self) -> f32;                                        //the functions can be implemented when finding the areas
    fn set_length(&mut self, length: f32);                        //of circle and triangle
    fn get_length(&self) -> f32;
    fn set_width(&mut self, width: f32);
    fn get_width(&self) -> f32;
    fn set_name(&mut self, name: &'static str);
    fn get_name(&self) -> &str;
}

struct Rect {
    //// CHANGED i32's to f32
    length: f32,
    width: f32,
    name: &'static str,
}

impl Rect {
    fn default() -> Self {      
        Rect {
            length: 1.0,
            width: 1.0,
            name: "default_name",
        }
    }
}

impl Shape for Rect {
    //// CHANGED i32 to f32
    fn new(length: f32, width: f32, name: &'static str) -> Self {
        Rect {
            length,
            width,
            name,
        }
    }

    fn area(&self) -> f32 {
        (self.length * self.width) as f32
    }

    fn set_length(&mut self, length: f32) {
        self.length = length
    }

    fn get_length(&self) -> f32 {
        self.length
    }

    fn set_width(&mut self, width: f32) {
        self.width = width
    }

    fn get_width(&self) -> f32 {
        self.width
    }

    fn set_name(&mut self, name: &'static str) {
        self.name = name
    }

    fn get_name(&self) -> &str {
        self.name
    }
}

impl PartialEq for Rect {
    fn eq(&self, other: &Self) -> bool {
        self.area() == other.area()
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialOrd for Rect {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.area().partial_cmp(&other.area())
    }

    fn lt(&self, other: &Rect) -> bool {
        println!("Taking Less Than");
        return self.area() < other.area();
    }

    fn gt(&self, other: &Self) -> bool {
        println!("Taking greater Than");
        return self.area() > other.area();
    }
    // Provided methods
    //fn lt(&self, other: &Rhs) -> bool { ... }
    //fn le(&self, other: &Rhs) -> bool { ... }
    //fn gt(&self, other: &Rhs) -> bool { ... }
    //fn ge(&self, other: &Rhs) -> bool { ... }
}
#[derive(Debug)]
struct Circle {         //Creating a new struct 'Circle' with the fields 'radius' and name
    radius: f32,
    name: &'static str,
}

impl Circle {               //Assigning values for default circle
    fn default() -> Self {
        Circle {
            radius: 3.5,
            name: "default_circle",
        }
    }
}
impl Circle {           //Implementing a perimeter functionality for our user defined type struct 'Circle'
    fn perimeter(&self) -> f32 {
        2.0 * 3.142 * self.radius
    }
}

impl Shape for Circle {
    fn new(length: f32, _width: f32, name: &'static str) -> Self {
        Circle {
            radius: length,
            name,
        }
    }

    fn area(&self) -> f32 {     
        3.142 * self.radius * self.radius
    }

    fn set_length(&mut self, length: f32) {
        self.radius = length as f32;
    }

    fn get_length(&self) -> f32 {
        self.radius
    }

    //My  attempt of implementnting Diameter because the diameter of a circle is double the radius
    fn get_width(&self) -> f32 {
        self.radius * 2.0
    }
    fn set_width(&mut self, width: f32) {
        self.radius = width / 2.0;
    }

    fn set_name(&mut self, name: &'static str) {
        self.name = name;
    }

    fn get_name(&self) -> &str {
        self.name
    }
}

//implement Partial Eq
impl PartialEq for Circle {
    fn eq(&self, other: &Self) -> bool {    //Compares two circles to see if their areas are equal
        self.area() == other.area()
    }

    fn ne(&self, other: &Self) -> bool {    //Compares two circles to see if they are not equal
        !self.eq(other)
    }
}

impl PartialOrd for Circle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.area().partial_cmp(&other.area())
    }

    fn lt(&self, other: &Circle) -> bool {      //Compares the circles to see if one area is less than the other
        println!("Taking Less Than");
        return self.area() < other.area();
    }

    fn gt(&self, other: &Self) -> bool {    //Compares the circles to see if one area is greater than the other
        println!("Taking greater Than");
        return self.area() > other.area();
    }
}


//A conversion implementation into String
//Expects a string slice with radius and name, separated by commas
impl From<&'static str> for Circle {
    fn from(s: &'static str) -> Circle {
        let mut parts = s.split(',');
        let radius = match parts.next() {
            Some(val) => val.parse::<f32>().unwrap(),
            None => 0.0,
        };
        let name = match parts.next() {
            Some(val) => val,
            None => "",
        };
        Circle {
            radius,
            name: &name,
        }
    }
}

/*for Circle*/
pub fn circles() {
    let circle1 = Circle::default();

    println!("{}", circle1.radius);
    //println!("{}", circle1.width);
    println!("{}", circle1.name);

    let circle2 = Circle::new(1.0, 2.0, "circle2");
    let circle3 = Circle::from("4,circle3");
    println!("Circle from String: {:#?}", circle3);

    //Compare using PartialOrd
    let result1 = circle1.partial_cmp(&circle2);
    println!(" result1 = {:?}", result1);

    let result2 = circle1.le(&circle2);
    println!("result2 = {:?}", result2);

    //Compare using PartialEq
    let result3 = circle2.eq(&circle3);
    println!("result3 = {:?}", result3);

    let result4 = circle2.ne(&circle3);
    println!("result4 = {:?}", result4);
}

struct Triangle {       //Creating a new struct 'Triangle' with the fields 'base', 'heigth', 'last side' and 'name'
    base: f32,
    height: f32,
    last_side: f32,
    name: &'static str,
}

impl Triangle {     //Implementing a perimeter functionality for our user defined type struct 'Triangle'
    fn perimeter(&self) -> f32 {
        self.base + self.height + self.last_side
    }
}

impl Triangle {     //Assigning values for default triangle
    fn default() -> Self {
        Triangle {
            height: 5.0,
            base: 4.0,
            last_side: 6.0, //Introduced a new field since a trianle has 3 sides
            name: "default_triangle",
        }
    }
}

impl Shape for Triangle {
    fn new(length: f32, width: f32, name: &'static str) -> Self {
        Triangle {
            height: length,
            base: width,
            last_side: width,
            name,
        }
    }
    fn area(&self) -> f32 {     
        0.5 * self.base * self.height
    }

    fn set_length(&mut self, length: f32) {
        self.height = length
    }
    fn get_length(&self) -> f32 {
        self.height
    }

    fn set_width(&mut self, width: f32) {
        self.base = width
    }
    fn get_width(&self) -> f32 {
        self.base
    }

    fn set_name(&mut self, name: &'static str) {
        self.name = name
    }

    fn get_name(&self) -> &str {
        self.name
    }
}

//implement Partial Eq
impl PartialEq for Triangle {
    fn eq(&self, other: &Self) -> bool {    //Compares two triangles to see if their areas are equal
        self.area() == other.area()
    }

    fn ne(&self, other: &Self) -> bool {    //Compares two triangles to see if they are not equal
        !self.eq(other)
    }
}

impl PartialOrd for Triangle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.area().partial_cmp(&other.area())
    }

    fn lt(&self, other: &Triangle) -> bool {        //Compares the triangles to see if one area is less than the other
        println!("Taking Less Than");
        return self.area() < other.area();
    }

    fn gt(&self, other: &Self) -> bool {        //Compares the triangles to see if one area is greater than the other
        println!("Taking greater Than");
        return self.area() > other.area();
    }
}

//A conversion implementation into String
//Expects a string slice with height, base, last_side and name, separated by commas
impl From<&'static str> for Triangle {
    fn from(s: &'static str) -> Triangle {
        let mut parts = s.split(',');
        let height = match parts.next() {
            Some(val) => val.parse::<f32>().unwrap(),
            None => 0.0,
        };
        let base = match parts.next() {
            Some(val) => val.parse::<f32>().unwrap(),
            None => 0.0,
        };
        let last_side = match parts.next() {
            Some(val) => val.parse::<f32>().unwrap(),
            None => 0.0,
        };
        let name = match parts.next() {
            Some(val) => val,
            None => "",
        };
        Triangle {
            height,
            base,
            last_side,
            name: &name,
        }
    }
}

pub fn triangles() {
    let Triangle = Triangle::default();
    
    println!("{}", Triangle.height);
    println!("{}", Triangle.base);
    println!("{}", Triangle.last_side);
    println!("{}", Triangle.name);


    let triangle2 = Rect::new(1, 3, "Triangle2");
    let triangle3 = Rect::from("4,5,Triangle3");

    //Compare using PartialOrd
    let result1 = Triangle.partial_cmp(&triangle2);
    println!(" result1 = {:?}", result1);

    let result2 = Triangle.le(&triangle2);
    println!("result2 = {:?}", result2);

    //Compare using PartialEq
    let result3 = rectangle2.eq(&triangle3);
    println!("result3 = {:?}", result3);

    let result4 = rectangle2.ne(&triangle3);
    println!("result4 = {:?}", result4);
}
fn main() {
    run2();
    circles();
    triangles();
}
