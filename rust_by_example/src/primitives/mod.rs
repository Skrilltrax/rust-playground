use std::fmt;
use std::mem;

#[allow(dead_code)]
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
#[allow(dead_code)]
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // `let` can be used to bind the members of a tuple to variables
    let (integer, boolean) = pair;

    (boolean, integer)
}

#[allow(dead_code)]
fn transpose(matrix: Matrix) -> Matrix {
    let new_matrix = Matrix(matrix.0, matrix.2, matrix.1, matrix.3);

    return new_matrix;
}

#[allow(dead_code)]
pub fn tuples() {
    let mat = Matrix(1.1, 1.2, 2.1, 2.2);

    println!("Debug Matrix: {:?}", mat);
    println!("Debug TransposeMatrix: {:?}", transpose(mat));

    // Previous matrix is out of scope now
    let mat = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("Display Matrix: \n{}", mat);
    println!("Display TransposeMatrix: \n{}", transpose(mat));
}

// This function borrows a slice
#[allow(dead_code)]
fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

#[allow(dead_code)]
pub fn arrays() {
    // Fixed-size array (type signature is superfluous)
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // All elements can be initialized to the same value
    let ys: [i32; 500] = [0; 500];

    // Indexing starts at 0
    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    // `len` returns the count of elements in the array
    println!("number of elements in array: {}", xs.len());

    // Arrays are stack allocated
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    // Arrays can be automatically borrowed as slices
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    // Slices can point to a section of an array
    // They are of the form [starting_index..ending_index]
    // starting_index is the first position in the slice
    // ending_index is one more than the last position in the slice
    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1..4]);

    // Out of bound indexing causes compile error
    // println!("{}", xs[5]);
}
