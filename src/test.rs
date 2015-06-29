use super::mpfr::Mpfr;

#[test]
fn test_init() {
	let mpfr1 : Mpfr = From::<i64>::from(1000);
	let mpfr2 : Mpfr = From::<i64>::from(5000);
	assert!(mpfr1 != mpfr2);
}