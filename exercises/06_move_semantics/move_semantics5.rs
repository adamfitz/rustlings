// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.

#[test]
fn main() {
    let mut x = 100;
    // first borrow
    let y = &mut x;
    // perform action on the borrowed value and assign to new variable
    *y += 100;

    // second borrow, cannot occur before the first borrow action
    let z = &mut x;
    // perform action on borrow var and assign to new
    *z += 1000;
    assert_eq!(x, 1200);
}
