// fn main() {
//     another_function(5, 6);
// }

// fn another_function(x: i32, y: i32) {
//     println!("The value of x is: {}", x);
//     println!("The value of y is: {}", y);
// }

/////////////////////////////

// expression returns a value
// statement does not return a value

// fn main() {
//     let x = 5;

//     let y = {
//         let x = 3;
//         x + 1 // expressions do not end with semi-colon
//     };

//     println!("y = {}", y);
// }

///////////////////////////

// fn five() -> i32 {
//     5
// }

// fn main() {
//     let x = five();

//     println!("x = {}", x);
// }

////////////////////////////

fn main() {
    let x = plus_one(5);

    println!(" x = {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
