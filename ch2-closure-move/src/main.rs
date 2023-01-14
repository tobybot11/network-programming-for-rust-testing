// chapter2 / closure-move.rs
//

fn main() {
    let mut times = 2  ;
    {
        let mut borrow = |x| times += x ;
        borrow(5) ;
    }
    assert_eq!(7, times) ;
    assert_eq!(times, 7) ;

    let mut own = move |x| times += x;
    own(5);
    assert_eq!(times,7) ;

    println!("times = {}", times) ;
}
