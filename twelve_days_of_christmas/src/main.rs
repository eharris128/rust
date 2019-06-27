fn main() {
    let mut index = 0;
    let gifts = [
        "a partridge in a pear tree",
        "two turtle doves",
        "three french hens",
        "four calling birds",
        "five gold rings",
        "six geese a-layings",
        "seven swans a-swimming",
        "eight maids a-milking",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twleve drummers drumming"
    ];
    while index < 12 {
        println!("On the {} day of Christmas my true love sent to me \n", index);
        for day in (0..(index+1)).rev() {
            // display 'and' prior to patridge except for the initial verse
            let and_or_blank =
                if index != 1 && day == 0 {
                    "and "
                } else {
                    ""
                };
            println!("{}{} \n", and_or_blank, gifts[day]);
            
            // skip an additional line between verses
            if day == 0 {
                println!();
            }
        }
        index += 1;
    }
}
