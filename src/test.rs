use super::mpfr::Mpfr;

#[test]
fn test_set() {
	let mut a : Mpfr = From::<i64>::from(1000);
	let b : Mpfr = From::<i64>::from(5000);
	assert!(a != b);
	a.set(&b);
	assert!(a == b);
}

#[test]
fn test_eq() {
    let x: Mpfr = From::<f64>::from(1.234567);
    let y: Mpfr = From::<f64>::from(1.234567);
    let z: Mpfr = From::<f64>::from(1.234568);

    assert!(x == y);
    assert!(x != z);
    assert!(y != z);
}

#[test]
fn test_ord() {
    let x: Mpfr = From::<i64>::from(-1048576);
    let y: Mpfr = From::<i64>::from(2);
    let z: Mpfr = From::<i64>::from(1048576);

    assert!(x < y && x < z && y < z);
    assert!(x <= x && x <= y && x <= z && y <= z);
    assert!(z > y && z > x && y > x);
    assert!(z >= z && z >= y && z >= x && y >= x);
}

#[test]
#[should_panic]
fn test_div_zero() {
    let x: Mpfr = From::<i64>::from(1);
    let y = Mpfr::zero(1);
    x / y;
}

#[test]
fn test_clone() {
    let a: Mpfr = From::<i64>::from(100);
    let b = a.clone();
    assert!(b == a);
}

#[test]
fn test_add() {
	let a: Mpfr = From::<i64>::from(15);
	let b: Mpfr = From::<i64>::from(20);
	let result: Mpfr = From::<i64>::from(35);

	assert!(a + b == result);
}

#[test]
fn test_sub() {
	let a: Mpfr = From::<i64>::from(15);
	let b: Mpfr = From::<i64>::from(20);
	let result: Mpfr = From::<i64>::from(-5);

	assert!(a - b == result);
}

#[test]
fn test_mul() {
	let a: Mpfr = From::<i64>::from(15);
	let b: Mpfr = From::<i64>::from(20);
	let result: Mpfr = From::<i64>::from(300);

	assert!(a * b == result);
}

#[test]
fn test_div() {
	let a: Mpfr = From::<i64>::from(15);
	let b: Mpfr = From::<i64>::from(20);
	let result: Mpfr = From::<f64>::from(0.75);

	assert!(a / b == result);
}