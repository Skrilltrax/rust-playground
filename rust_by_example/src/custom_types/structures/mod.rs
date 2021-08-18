#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

#[allow(dead_code)]
fn rect_area(rect: Rectangle) -> f32 {
    let Rectangle {
        top_left: Point { x: x1, y: y1 },
        bottom_right: Point { x: x2, y: y2 },
    } = rect;

    let height = y2 - y1;
    let base = x2 - x1;
    return height * base;
}

#[allow(dead_code)]
fn square(point: Point, size: f32) -> Rectangle {
    let Point { x: x1, y: y1 } = point;
    let point2 = Point {
        x: x1 + size,
        y: y1 + size,
    };

    Rectangle {
        top_left: point,
        bottom_right: point2,
    }
}

#[allow(dead_code)]
pub fn structures() {
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    // Instantiate a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 5.2, ..point };

    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point {
        x: left_edge,
        y: top_edge,
    } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right: bottom_right,
    };

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    let rect = Rectangle {
        top_left: Point { x: 1.0, y: 1.0 },

        bottom_right: Point { x: 11.0, y: 11.0 },
    };

    println!("Rectangle: {:?}", rect);
    println!("Rectangle area: {}", rect_area(rect));

    let point = Point { x: 0.0, y: 0.0 };
    let square = square(point, 10.0);

    println!("Square from point 0.0 with size 10.0 is: {:?}", square);
}
