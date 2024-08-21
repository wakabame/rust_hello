mod future;

fn main() {
    future::main();
}

pub fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

#[test]
fn test_add() {
    assert_eq!(0, add(0, 0));
}
