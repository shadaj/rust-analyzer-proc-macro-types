use passthrough::passthrough;

fn main() {
    let test1 = 123;
    passthrough!({
        let test2 = 123;
    })
}
