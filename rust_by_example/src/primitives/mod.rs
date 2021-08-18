#[allow(dead_code)]
use std::fmt;

pub fn literals_operators() {
    // Integer addition
    println!("1 + 2 = {}", 1u32 + 2);

    // Integer subtraction
    println!("1 - 2 = {}", 1i32 - 2);
    // TODO ^ Try changing `1i32` to `1u32` to see why the type is important

    // Short-circuiting boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // Use underscores to improve readability!
    println!("One million is written as {}", 1_000_000u32);
}

// The following struct is for the activity.
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {})\n", self.0, self.1)?;
        write!(f, "({} {})", self.2, self.3)
    }
}

// Tuples can be used as function arguments and as return values
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // `let` can be used to bind the members of a tuple to variables
    let (integer, boolean) = pair;

    (boolean, integer)
}

fn transpose(matrix: Matrix) -> Matrix {
    let new_matrix = Matrix(matrix.0, matrix.2, matrix.1, matrix.3);

    return new_matrix;
}

pub fn tuples() {
    let mat = Matrix(1.1, 1.2, 2.1, 2.2);

    println!("Debug Matrix: {:?}", mat);
    println!("Debug TransposeMatrix: {:?}", transpose(mat));

    // Previous matrix is out of scope now
    let mat = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("Display Matrix: \n{}", mat);
    println!("Display TransposeMatrix: \n{}", transpose(mat));
}
