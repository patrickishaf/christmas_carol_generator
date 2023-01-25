use std::io;

const NUMBER_OF_DAYS_OF_CHRISTMAS: usize = 12;

fn main() {
    println!("Hello, world! Welcome to The Christmas Carol Lyrics Game");
    println!("I will now sing you 'The Twelve Days Of Christmas");

    for number in 1..=NUMBER_OF_DAYS_OF_CHRISTMAS {
        generate_carol(number);
    }
}

fn generate_carol(day_of_christmas: usize) {
    let christmas_gifts = [
        "partridge in a pear tree",
        "turtle doves",
        "french hens",
        "calling birds",
        "golden riiiiiiiiings",
        "geese are laying",
        "swans are swimming",
        "maids are milking",
        "ladies dancing",
        "lords are leaping",
        "pipers piping",
        "drummers drumming",
    ];
    if day_of_christmas == 1 {
        println!("On the {day_of_christmas}st day of christmas, my true love gave to me...");
    } else if day_of_christmas == 2 {
        println!("On the {day_of_christmas}nd day of christmas, my true love gave to me...");
    } else if day_of_christmas == 3 {
        println!("On the {day_of_christmas}rd day of christmas, my true love gave to me...");
    } else {
        println!("On the {day_of_christmas}th day of christmas, my true love gave to me...");
    }
    for day in (0..day_of_christmas).rev() {
        if day == 0 {
            if day_of_christmas == 1 {
                println!("a {}", christmas_gifts[day]);
                break;
            }
            println!("and a {}", christmas_gifts[day]);
            break;
        }
        let day = day + 1;
        println!("{day} {}", christmas_gifts[day-1]);
    }
}
