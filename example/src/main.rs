use passthrough::{passthrough, passthrough_wrapped};

fn main() {
    let _test1 = 123;
    passthrough!({
        let _test2 = 123;
    });

    let res = Result::<i32, ()>::Ok(123);
    let _: i32 = passthrough_wrapped!(res.unwrap().abs());
}
