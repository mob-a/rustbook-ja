extern crate adder;

#[test]
fn it_adds_two_integ() {
    assert_eq!(4, adder::add_two(2));
}
