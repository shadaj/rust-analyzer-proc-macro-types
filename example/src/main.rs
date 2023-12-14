use std::vec::IntoIter;

use passthrough::{passthrough, passthrough_wrapped};

fn main() {
    let test1 = 123;
    passthrough!({
        let test2 = 123;
    });

    let my_iter: IntoIter<Result<i32, ()>> = vec![Ok(1), Ok(2), Ok(3)].into_iter();
    let _ = my_iter.map(passthrough_wrapped!(|x: Result<i32, ()>| x.unwrap().abs()));
}
