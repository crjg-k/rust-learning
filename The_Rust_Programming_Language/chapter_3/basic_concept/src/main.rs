fn main() {
    let a = 8964;
    if (a > 0x1989u64) {
        println!("a = {}", a);
    }
    let mut idx = 2952;
    while (idx > 2900) {
        println!("idx = {}", idx);
        idx -= 1;
    }
    for number in (1..4) {
        println!("{number}!");
    }
}
