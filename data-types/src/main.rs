use std::io;

/*  
*  Data Types
*/

fn main() {


    /* The Tuple Type */
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    println!("The value of z is: {z}");


    /* The Array Type */
    let a = [1, 2, 3, 4, 5];
    let mut index = String::new();

    println!("Please enter an array index.");
    io::stdin()
    .read_line(&mut index)
    .expect("Failed to read line");

    let index: usize = index
    .trim()
    .parse()
    .expect("Index entered was not a number");

    let element = a[index];
    println!("The value of the element at index {index} is: {element}");


    /* The Scalar Type */
    let mut xyz: i32 = "32".parse().expect("Not a number!");
    // Function Calling
    xyz = plus_one(xyz);
    println!("XYZ Value after function and return: {xyz}");

}


fn plus_one(x: i32) -> i32 {
    return x + 1;
}