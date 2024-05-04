use my_crate::kinds::PrimaryColor;
use my_crate::kinds::SecondaryColor;
use my_crate::utils::mix;


fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    let orange = mix(red, yellow);
}
