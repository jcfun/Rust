// use cargo_crates_demo::kinds::PrimaryColor;
// use cargo_crates_demo::utils::mix;
use cargo_crates_demo::PrimaryColor;
use cargo_crates_demo::mix;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}
