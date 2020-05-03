fn add_two_nums(n1: i32, n2: i32) -> i32{
    n1 + n2
}

fn main() {
    println!("{}", "2 + 2 is ".to_string() + &add_two_nums(2, 2).to_string());
}