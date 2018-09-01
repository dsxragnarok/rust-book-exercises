// fn main() {
//     let v = vec![1,2,3,4,5];

//     // let does_not_exist = &v[100];
//     let does_not_exist = v.get(100);
// }

// #[derive(Debug)]
// enum SpreadsheetCell {
//     Int(i32),
//     Float(f64),
//     Text(String),
// }

// fn main() {
//     let row = vec![
//         SpreadsheetCell::Int(3),
//         SpreadsheetCell::Text(String::from("blue")),
//         SpreadsheetCell::Float(10.20)
//     ];

//     println!("{:#?}", row);
// }

// fn main() {
//     let s1 = String::from("Hello, ");
//     let s2 = String::from("world!");
//     let s3 = s1 + &s2; // s1 has been moved here so can no longer be used

//     // println!("s1 {}", s1); // s1 moved to s3 so is no longer valid
//     println!("s2 {}", s2);
//     println!("s3 {}", s3);

//     // format!
//     let s4 = String::from("tic");
//     let s5 = String::from("tac");
//     let s6 = String::from("toe");

//     let ttt = format!("{}-{}-{}", s4, s5, s6);

//     println!("{}", ttt);
// }

// use std::collections::HashMap;

// fn main() {
//     let mut scores = HashMap::new();

//     scores.insert(String::from("Blue"), 10);
//     scores.insert(String::from("Yellow"), 50);

//     println!("{:#?}", scores);

//     ///////////////////////

//     let teams = vec![String::from("Green"), String::from("Red")];
//     let initial_scores = vec![33, 55];

//     let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

//     println!("{:#?}", scores);

//     let team_name = String::from("Red");
//     let score = scores.get(&team_name);
//     match score {
//         None => (),
//         Some(i) => println!("{} score is {}", team_name, i)
//     };

//     // unwrapped is discouraged because it can be None
//     // The preferred method of handling is to use match like above
//     let sc = score.unwrap();
//     println!("unwrapped score : {}", sc);

//     ////////
//     let mut scores = HashMap::new();

//     scores.insert(String::from("Avengers"), 55);
//     scores.insert(String::from("Justice League"), 10);

//     for (key, value) in &scores {
//         println!("{}: {}", key, value);
//     }

//     scores.insert(String::from("Avengers"), 100);
//     println!("{:#?}", scores);

//     scores.entry(String::from("Justice League")).or_insert(30);
//     scores.entry(String::from("X-Men")).or_insert(70);

//     println!("{:#?}", scores);

//     //////////////

//     let text = "hello world wonderful world";
//     let mut map = HashMap::new();

//     for word in text.split_whitespace() {
//         let count = map.entry(word).or_insert(0);
//         *count += 1;
//     }
//     println!("{:?}", map);
// }


fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    // let mut largest = number_list[0];

    // for number in number_list {
    //     if number > largest {
    //         largest = number;
    //     }
    // }
    // println!("The largest number is {}", largest);

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("The largest char is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);

}

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}
fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}
