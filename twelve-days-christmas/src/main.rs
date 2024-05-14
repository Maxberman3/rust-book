

fn main() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth"
    ];
    let gifts = [
        "a partridge in a pear tree",
        "two turtle doves",
        "three French hens",
        "four calling birds",
        "five golden rings",
        "six geese a-laying"
    ];

    for i in 0..days.len() {
        println!("On the {} day of Christmas my true love sent to me:", days[i]);
        for j in (0..=i).rev() {
            if i != 0 && j == 0 {
                println!("And {}", gifts[j]);
            } else {
                println!("{}", gifts[j]);
            }
        }
        println!();
    }
}
