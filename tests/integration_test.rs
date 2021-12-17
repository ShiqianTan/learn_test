use learn_test;
mod commmon;

#[test]
fn it_adds_two() {
	assert_eq!(4, learn_test::add_two(2));
}

#[test]
fn it_really_adds_two() {
	commmon::setup();
	assert_eq!(5, learn_test::add_two(3));
}
