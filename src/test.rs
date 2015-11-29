use rustc_serialize::json;

use super::mpfr::Mpfr;

#[test]
fn test_set() {
    let mut a: Mpfr = From::<i64>::from(1000);
    let b: Mpfr = From::<i64>::from(5000);
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

    assert!(&a + b == result);
    assert!(&a + 20 == result);
    assert!(20 + &a == result);
    assert!(&a + 20.0 == result);
    assert!(20.0 + &a == result);
}

#[test]
fn test_sub() {
    let a: Mpfr = From::<i64>::from(15);
    let b: Mpfr = From::<i64>::from(20);
    let result: Mpfr = From::<i64>::from(-5);

    assert!(&a - &b == result);
    assert!(&a - 20 == result);
    assert!(15 - &b == result);
    assert!(&a - 20.0 == result);
    assert!(15.0 - &b == result);
}

#[test]
fn test_mul() {
    let a: Mpfr = From::<i64>::from(15);
    let b: Mpfr = From::<i64>::from(20);
    let result: Mpfr = From::<i64>::from(300);

    assert!(&a * b == result);
    assert!(&a * 20 == result);
    assert!(20 * &a == result);
    assert!(&a * 20.0 == result);
    assert!(20.0 * &a == result);
}

#[test]
fn test_div() {
    let a: Mpfr = From::<i64>::from(15);
    let b: Mpfr = From::<i64>::from(20);
    let result: Mpfr = From::<f64>::from(0.75);

    assert!(&a / &b == result);
    assert!(&a / 20 == result);
    assert!(15 / &b == result);
    assert!(&a / 20.0 == result);
    assert!(15.0 / &b == result);
}

#[test]
fn test_rounding() {
    let a: Mpfr = From::<f64>::from(2.4999);
    let b: Mpfr = From::<f64>::from(2.5);
    let two: Mpfr = From::<i64>::from(2);
    let three: Mpfr = From::<i64>::from(3);

    assert!(a.floor() == two);
    assert!(a.round() == two);
    assert!(a.ceil() == three);

    assert!(b.floor() == two);
    assert!(b.round() == three);
    assert!(b.ceil() == three);
}

#[test]
fn test_pow_root() {
    let a: Mpfr = From::<f64>::from(2.654);
    let two: Mpfr = From::<i64>::from(2);
    let three: Mpfr = From::<i64>::from(3);
    let asq = &a * &a;
    let acb = &a * &a * &a;

    assert!(a.pow(&two) == asq);
    assert!(a.pow(&three) == acb);
    assert!(asq.sqrt() == a);
    assert!(acb.cbrt() == a);
    assert!(asq.root(2) == a);
    assert!(acb.root(3) == a);
}

#[test]
fn test_exp_log() {
    let a: Mpfr = From::<i64>::from(1);
    let b: Mpfr = From::<f64>::from(2.718281828459045);

    assert!(a.exp() == b);
    assert!(b.log() == a);
}

#[test]
fn test_new_from_str() {
    let a: Mpfr = From::<i64>::from(1);
    let b: Mpfr = Mpfr::new_from_str("1", 10).unwrap();

    assert!(a == b);

    let a: Mpfr = From::<f64>::from(1.234);
    let b: Mpfr = Mpfr::new_from_str("1.234", 10).unwrap();

    assert!(a == b);
}

#[test]
fn test_new2_from_str() {
    let epsilon = 1e-100f64;

    {
        let a: Mpfr = From::<i64>::from(1);
        let b: Mpfr = Mpfr::new2_from_str(200, "1", 10).unwrap();
        let c = a.clone();
        let d = b.clone();

        assert!(a - b < From::from(epsilon) || c - d > From::from(-epsilon));
    }

    {
        let a: Mpfr = From::<f64>::from(1.234);
        let b: Mpfr = Mpfr::new2_from_str(200, "1.234", 10).unwrap();
        let c = a.clone();
        let d = b.clone();

        assert!(a - b < From::from(epsilon) || c - d > From::from(-epsilon));
    }
}

#[test]
fn test_debug() {
    let a: Mpfr = Mpfr::new2_from_str(128, "1.23456789123456789123456789123456789e5", 10).unwrap();

    assert_eq!(format!("{:?}", a), "1.23456789123456789123456789123456789e+05");
}

#[test]
fn test_display() {
    let a: Mpfr = Mpfr::new2_from_str(128, "1.23456789123456789123456789123456789e5", 10).unwrap();

    assert_eq!(format!("{}", a), "1.23456789123456789123456789123456789e+05");
}

#[test]
fn test_to_string() {
    let a: Mpfr = Mpfr::new2_from_str(128, "1.23456789123456789123456789123456789e5", 10).unwrap();

    assert_eq!(a.to_string(), "1.23456789123456789123456789123456789e+05");
}

#[test]
fn test_to_string_new2_from_str() {
    let a: Mpfr = Mpfr::new2_from_str(128, "1.23456789123456789123456789123456789e5", 10).unwrap();

    assert!(a == Mpfr::new2_from_str(128, a.to_string(), 10).unwrap());
}

#[test]
fn test_abs() {
    let a: Mpfr = From::<i64>::from(1);
    let b: Mpfr = From::<i64>::from(-1);

    assert!(a.abs() == b.abs());
}

#[test]
fn test_rustc_serialize() {
    #[derive(RustcDecodable, RustcEncodable, PartialEq)]
    struct Test {
        price: Mpfr,
    }
    let a: Test = Test { price: From::<f64>::from(0.75) };
    assert_eq!(json::encode(&a).unwrap(), "{\"price\":\"7.5e-01\"}");
    let b: Test = json::decode("{\"price\": \"0.75\"}").unwrap();
    assert!(a == b);
}
