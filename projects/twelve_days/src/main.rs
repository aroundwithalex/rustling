fn main() {
    twelve_days();
}

fn twelve_days() {
    for a in 1..13 {
        let day_of_christmas = match a {
            1 => "first",
            2 => "second",
            3 => "third",
            4 => "fourth",
            5 => "fifth",
            6 => "sixth",
            7 => "seventh",
            8 => "eighth",
            9 => "ninth",
            10 => "tenth",
            11 => "eleventh",
            12 => "twelfth",
            _ => ""
        };

        println!("\nOn the {day_of_christmas} of Christmas, my true love sent to me");
        
        if a >= 12 {
            println!("Twelve drummers drumming");
        }

        if a >= 11 {
            println!("I sent elven pipers piping");
        }

        if a >= 10 {
            println!("Ten lords a-leaping");
        }

        if a >= 9 {
            println!("Nine ladies dancing");
        }

        if a >= 8 {
            println!("Eight maids a-milking");
        }

        if a >= 7 {
            println!("Seven swans a-swimming");
        }

        if a >= 6 {
            println!("Six geese a-laying");
        }

        if a >= 5 {
            println!("Five gold rings");
        }

        if a >= 4 {
            println!("Four calling birds");
        }

        if a >= 3 {
            println!("Three french hens");
        }

        if a >= 2 {
            println!("Two turtledoves");
        }

        if a == 1 {
            println!("A partridge in a pear tree");
        } else if a > 1 {
            println!("And a partridge in a pear tree");
        }
    }
}
