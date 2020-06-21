pub fn verse(n: u32) -> String {
    if n > 2 {
        format!(
            "{0} bottles of beer on the wall, {0} bottles of beer.
Take one down and pass it around, {1} bottles of beer on the wall.
",
            n,
            n - 1
        )
    } else if n == 2 {
        format!(
            "2 bottles of beer on the wall, 2 bottles of beer.
Take one down and pass it around, 1 bottle of beer on the wall.
"
        )
    } else if n == 1 {
        format!(
            "1 bottle of beer on the wall, 1 bottle of beer.
Take it down and pass it around, no more bottles of beer on the wall.
"
        )
    } else {
        format!(
            "No more bottles of beer on the wall, no more bottles of beer.
Go to the store and buy some more, 99 bottles of beer on the wall.
"
        )
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let mut res = String::new();

    for i in (end..=start).rev() {
        res += verse(i).as_str();
        if i != end {
            res += "\n";
        }
    }

    res
}
