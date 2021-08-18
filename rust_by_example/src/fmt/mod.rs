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
}
