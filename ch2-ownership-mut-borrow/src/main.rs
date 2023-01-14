/// function: main
fn main() {
    let mut s = String::from("Text");
    heap_example(&mut s);
    // println!("main : {}", s);
}

fn heap_example(input: &mut String) {
    let mystr = input;
    // let _otherstr = mystr.clone();
    let _otherstr = mystr.clone();
    // mystr.push_str(" more text");
    println!("{}", mystr);
}
