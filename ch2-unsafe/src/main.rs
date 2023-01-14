// chapter2 / unsafe.rs

fn main() {
    let num: u32 = 42;
    let p: *const u32 = &num;

    unsafe {
        assert_eq!(*p, num);
        assert_eq!(*p, 42);
    }
}
