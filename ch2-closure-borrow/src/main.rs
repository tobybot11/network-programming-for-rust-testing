// chapter2/closure-borrow.rs

fn main() {
    // closure with two parameters
    let add = | a, b | a + b;
    assert_eq!(add(1, 2), 3);
    assert_eq!(add(2, 3), 5);
    println!("add(1, 2) = {}", add(1, 2));

    // common use cases are on iterators
    println!("{:?}", (1..10).filter(|x| x % 2 == 0).collect::<Vec<u32>>());

    // using a variable from enclosing scope
    let times = 2;
    println!("{:?}", (1..10).map(|x| x * times).collect::<Vec<i32>>());

}
