fn main() {
    for day in 1..=12 {
        println!("{}", generate_verse(day));
    }
}

fn generate_verse(day: u32) -> String {
    let lyrics_by_day = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming"
    ];
    let suffix = match day {
        1 => "st",
        2 => "nd",
        3 => "rd",
        _ => "th"
    };

    let mut index = day;
    let mut verse = String::from("On the {} day of Christmas my true love gave to me");

    verse = verse.replace("{}", &(day.to_string() + suffix));
    verse.push_str("\n");

    for lyric in lyrics_by_day[0..day as usize].iter().rev() {
        if index == 1 && day > 1 {
            verse.push_str("And ");
        }
        verse.push_str(lyric);
        verse.push_str("\n");

        index -= 1;
    }

    return verse;
}
