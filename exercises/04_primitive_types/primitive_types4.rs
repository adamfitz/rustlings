// primitive_types4.rs
//
// Get a slice out of Array a where the ??? is so that the test passes.
//
// Execute `rustlings hint primitive_types4` or use the `hint` watch subcommand
// for a hint.


#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    // borrow / point (&) to the array in memory to access elements in it
    // the start element needs to be BEFORE the element you want to return eg: here it starts at 1 and returns elements 
    // from 2 to 4
    let nice_slice = &a[1..4];

    assert_eq!([2, 3, 4], nice_slice)
}
