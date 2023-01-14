/// function: main
fn main() {
    let s = String::from("Text");  
    heap_example(&s);
}

fn heap_example(input: &String) {
    let mystr = input;
    let _otherstr = mystr;
    println!("Heap Example Input: {}", mystr);
}
