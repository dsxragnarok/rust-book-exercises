// fn main() {
//     let number = 5;

//     // if number < 5 {
//     //     println!("condition was true");
//     // } else {
//     //     println!("condition was false");
//     // }

//     // if number {
//     //     println!("number was five");
//     // }

//     // if number != 0 {
//     //     println!("number was somehing other than zero");
//     // }

//     if number % 4 == 0 {
//         println!("divisible by 4");
//     } else if number % 3 == 0 {
//         println!("divisible by 3");
//     } else if number % 2 == 0 {
//         println!("divisible by 2");
//     } else {
//         println!("not divisible by 4, 3 or 2");
//     }
// }

////////////////////////////

fn main() {
    let condition = true;
    let number = if condition {
        5
    } else {
        "six"
    };

    println!("The value of number is: {}", number);
}
