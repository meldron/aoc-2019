mod lib;

use lib::{verify_password, verify_password_2};

const START: usize = 136_818;
const END: usize = 685_979;

fn main() {
    // let valid_passwords: usize = (START..END)
    //     .map(verify_password)
    //     .filter_map(Result::ok)
    //     .count();

    // println!("Valid passwords 1): {}", valid_passwords);

    let valid_passwords_2: usize = (START..END)
        .map(verify_password_2)
        .filter_map(Result::ok)
        .count();

    println!("Valid passwords 2): {}", valid_passwords_2)
}
