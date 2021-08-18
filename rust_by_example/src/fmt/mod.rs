use std::fmt;

#[derive(Debug)]
struct Structure(i32);

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Structure({})", self.0)
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hex_code = format!("0x{:0>2X}{:0>2X}{:0>2X}", self.red, self.green, self.blue);

        write!(
            f,
            "RGB ({}, {}, {}) {}",
            self.red, self.green, self.blue, hex_code
        )
    }
}

#[derive(Debug)]
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Extract the value using tuple indexing, and create a reference to `vec`.
        let vec = &self.0;

        // Try `write!` to see if it errors. If it errors, return
        // the error. Otherwise continue.
        write!(f, "[")?;

        // Iterate over `v` in `vec` while enumerating the iteration
        // count in `count`.
        for (count, v) in vec.iter().enumerate() {
            write!(f, "{}: {}", count, v)?;

            if count != vec.len() - 1 {
                write!(f, ", ")?;
            }
        }

        write!(f, "]")
    }
}

#[allow(dead_code)]
pub fn display() {
    let str = Structure(23);
    println!("{}", str);

    let color = Color {
        red: 255,
        green: 00,
        blue: 128,
    };

    println!("Debug: {:?}", color);
    println!("Display: {:}", color);

    let v = List(vec![100, 200, 300]);
    println!("{}", v);
}
