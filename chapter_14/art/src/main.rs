fn main() {
    use art::kinds::PrimaryColor;
    use art::utils::mix;

    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}

fn _test1() {
    use art::PrimaryColor;
    use art::mix;

    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}
