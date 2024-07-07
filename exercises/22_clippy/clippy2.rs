fn main() {
    let mut res: i32 = 42;
    let option: Option<i32> = Some(12);
    if let Some(x) = option {
        res += x;
    }

    println!("{res}");
}
