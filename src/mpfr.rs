use gmp::mpf::{Mpf, mpf_ptr, mpf_srcptr};
use gmp::mpq::{Mpq, mpq_srcptr};
use gmp::mpz::{Mpz, mpz_ptr, mpz_srcptr};
use libc::{c_char, c_int, c_ulong, c_long, c_double, c_void, size_t};
use rustc_serialize::{Decodable, Decoder, Encodable, Encoder};
use std::ffi::CStr;
use std::cmp::{Eq, PartialEq, Ord, PartialOrd, Ordering};
use std::cmp;
use std::convert::{From, Into};
use std::ffi::CString;
use std::fmt;
use std::mem::uninitialized;
use std::ops::{Add, Sub, Mul, Div, Neg};
use std::str;
use std::ptr;

type mpfr_prec_t = c_long;
type mpfr_sign_t = c_int;
type mpfr_exp_t = c_long;

#[repr(C)]
pub enum mpfr_rnd_t {
    MPFR_RNDN = 0, // round to nearest, with ties to even
    MPFR_RNDZ, // round toward zero
    MPFR_RNDU, // round toward +Inf
    MPFR_RNDD, // round toward -Inf
    MPFR_RNDA, // round away from zero
    MPFR_RNDF, // faithful rounding (not implemented yet)
    MPFR_RNDNA = -1, // round to nearest, with ties away from zero (mpfr_rouund)
}

#[repr(C)]
pub struct mpfr_struct {
    _mpfr_prec: mpfr_prec_t,
    _mpfr_sign: mpfr_sign_t,
    _mpfr_exp: mpfr_exp_t,
    _mpfr_d: *mut c_void,
}

pub type mpfr_srcptr = *const mpfr_struct;
pub type mpfr_ptr = *mut mpfr_struct;

#[link(name = "mpfr")]
extern {
    // Initialization
    fn mpfr_init(x: mpfr_ptr);
    fn mpfr_init2(x: mpfr_ptr, prec: mpfr_prec_t);
    fn mpfr_clear(x: mpfr_ptr);
    fn mpfr_set_default_prec(prec: mpfr_prec_t);
    fn mpfr_get_default_prec() -> mpfr_prec_t;
    fn mpfr_set_prec(x: mpfr_ptr, prec: mpfr_prec_t);
    fn mpfr_get_prec(x: mpfr_srcptr) -> mpfr_prec_t;

    // Assignment
    fn mpfr_set(rop: mpfr_ptr, op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    fn mpfr_set_ui(rop: mpfr_ptr, op: c_ulong, rnd: mpfr_rnd_t) -> c_int;
    fn mpfr_set_si(rop: mpfr_ptr, op: c_long, rnd: mpfr_rnd_t) -> c_int;
    fn mpfr_set_d(rop: mpfr_ptr, op: c_double, rnd: mpfr_rnd_t) -> c_int;
    fn mpfr_set_z(rop: mpfr_ptr, op: mpz_srcptr, rnd: mpfr_rnd_t) -> c_int;
    fn mpfr_set_q(rop: mpfr_ptr, op: mpq_srcptr, rnd: mpfr_rnd_t) -> c_int;
    fn mpfr_set_f(rop: mpfr_ptr, op: mpf_srcptr, rnd: mpfr_rnd_t) -> c_int;
    fn mpfr_set_ui_2exp(rop: mpfr_ptr, op: c_ulong, e: mpfr_exp_t, rnd: mpfr_rnd_t) -> c_int;
    fn mpfr_set_si_2exp(rop: mpfr_ptr, op: c_long, e: mpfr_exp_t, rnd: mpfr_rnd_t) -> c_int;
    fn mpfr_set_z_2exp(rop: mpfr_ptr, op: mpz_srcptr, e: mpfr_exp_t, rnd: mpfr_rnd_t) -> c_int;
    fn mpfr_set_nan(x: mpfr_ptr);
    fn mpfr_set_inf(x: mpfr_ptr, sign: c_int);
    fn mpfr_set_zero(x: mpfr_ptr, sign: c_int);
    fn mpfr_set_str(rop: mpfr_ptr, s: *const c_char, base: c_int, rnd: mpfr_rnd_t) -> c_int;

    // Conversion
    fn mpfr_get_ui(op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_ulong;
    fn mpfr_get_si(op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_long;
    fn mpfr_get_d(op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_double;
    fn mpfr_get_z(rop: mpz_ptr, op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    fn mpfr_get_f(rop: mpf_ptr, op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;

    // Comparison
    fn mpfr_cmp(op1: mpfr_srcptr, op2: mpfr_srcptr) -> c_int;
    fn mpfr_cmp_ui(op1: mpfr_srcptr, op2: c_ulong) -> c_int;

    // Arithmetic
    fn mpfr_add(rop: mpfr_ptr, op1: mpfr_srcptr, op2: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    fn mpfr_add_d(rop: mpfr_ptr, op1: mpfr_srcptr, op2: c_double, rnd: mpfr_rnd_t) -> c_int;
    fn mpfr_add_si(rop: mpfr_ptr, op1: mpfr_srcptr, op2: c_long, rnd: mpfr_rnd_t) -> c_int;
    fn mpfr_sub(rop: mpfr_ptr, op1: mpfr_srcptr, op2: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    fn mpfr_sub_d(rop: mpfr_ptr, op1: mpfr_srcptr, op2: c_double, rnd: mpfr_rnd_t) -> c_int;
    fn mpfr_sub_si(rop: mpfr_ptr, op1: mpfr_srcptr, op2: c_long, rnd: mpfr_rnd_t) -> c_int;
    fn mpfr_d_sub(rop: mpfr_ptr, op1: c_double, op2: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    fn mpfr_si_sub(rop: mpfr_ptr, op1: c_long, op2: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    fn mpfr_mul(rop: mpfr_ptr, op1: mpfr_srcptr, op2: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    fn mpfr_mul_d(rop: mpfr_ptr, op1: mpfr_srcptr, op2: c_double, rnd: mpfr_rnd_t) -> c_int;
    fn mpfr_mul_si(rop: mpfr_ptr, op1: mpfr_srcptr, op2: c_long, rnd: mpfr_rnd_t) -> c_int;
    fn mpfr_div(rop: mpfr_ptr, op1: mpfr_srcptr, op2: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    fn mpfr_div_d(rop: mpfr_ptr, op1: mpfr_srcptr, op2: c_double, rnd: mpfr_rnd_t) -> c_int;
    fn mpfr_div_si(rop: mpfr_ptr, op1: mpfr_srcptr, op2: c_long, rnd: mpfr_rnd_t) -> c_int;
    fn mpfr_d_div(rop: mpfr_ptr, op1: c_double, op2: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    fn mpfr_si_div(rop: mpfr_ptr, op1: c_long, op2: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    fn mpfr_neg(rop: mpfr_ptr, op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;

    // Rounding
    fn mpfr_floor(rop: mpfr_ptr, op: mpfr_srcptr) -> c_int;
    fn mpfr_ceil(rop: mpfr_ptr, op: mpfr_srcptr) -> c_int;
    fn mpfr_round(rop: mpfr_ptr, op: mpfr_srcptr) -> c_int;

    // Functions
    fn mpfr_sqrt(rop: mpfr_ptr, op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    fn mpfr_cbrt(rop: mpfr_ptr, op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    fn mpfr_root(rop: mpfr_ptr, op: mpfr_srcptr, k: c_ulong, rnd: mpfr_rnd_t) -> c_int;
    fn mpfr_pow(rop: mpfr_ptr, op1: mpfr_srcptr, op2: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    fn mpfr_abs(rop: mpfr_ptr, op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    fn mpfr_exp(rop: mpfr_ptr, op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    fn mpfr_log(rop: mpfr_ptr, op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    fn mpfr_gamma(rop: mpfr_ptr, op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    fn mpfr_lngamma(rop: mpfr_ptr, op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
    fn mpfr_lgamma(rop: mpfr_ptr, op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;

    // Formatted output
    fn mpfr_snprintf(buffer: *const c_char, length: size_t, string: *const u8, ...) -> c_int;
}

pub struct Mpfr {
    pub mpfr: mpfr_struct,
}

impl fmt::Debug for Mpfr {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self, fmt)
    }
}

impl fmt::Display for Mpfr {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            let length = mpfr_snprintf(ptr::null(), 0, b"%.Re\0".as_ptr(), &self.mpfr);
            if length < 0 {
                // Maybe fmt.write_str("@uninnitialized@")
                return Ok(())
            }
            let buff: Vec<c_char> = Vec::with_capacity((length + 1) as usize);
            mpfr_snprintf(buff.as_ptr(),
                          (length + 1) as size_t,
                          b"%.Re\0".as_ptr(),
                          &self.mpfr);
            let s = CStr::from_ptr(buff.as_ptr());
            fmt.write_str(str::from_utf8_unchecked(s.to_bytes()))
        }
    }
}

unsafe impl Send for Mpfr { }

impl Drop for Mpfr {
    fn drop(&mut self) {
        unsafe { mpfr_clear(&mut self.mpfr) }
    }
}

impl Clone for Mpfr {
    fn clone(&self) -> Mpfr {
        let mut mpfr = Mpfr::new2(self.get_prec());
        mpfr.set(self);
        mpfr
    }
}

impl Mpfr {
    pub fn new() -> Mpfr {
        unsafe {
            let mut mpfr = uninitialized();
            mpfr_init(&mut mpfr);
            Mpfr { mpfr: mpfr }
        }
    }

    pub fn new2(precision: usize) -> Mpfr {
        unsafe {
            let mut mpfr = uninitialized();
            mpfr_init2(&mut mpfr, precision as mpfr_prec_t);
            Mpfr { mpfr: mpfr }
        }
    }

    pub fn new_from_str<T: Into<Vec<u8>>>(s: T, base: usize) -> Option<Mpfr> {
        let c_string = match CString::new(s) {
            Ok(c_string) => c_string,
            Err(..) => return None,
        };
        unsafe {
            let mut mpfr = Mpfr::new();
            if mpfr_set_str(&mut mpfr.mpfr,
                            c_string.as_ptr(),
                            base as c_int,
                            mpfr_rnd_t::MPFR_RNDN) == 0 {
                Some(mpfr)
            } else {
                None
            }
        }
    }

    pub fn new2_from_str<T: Into<Vec<u8>>>(precision: usize, s: T, base: usize) -> Option<Mpfr> {
        let c_string = match CString::new(s) {
            Ok(c_string) => c_string,
            Err(..) => return None,
        };
        unsafe {
            let mut mpfr = Mpfr::new2(precision);
            if mpfr_set_str(&mut mpfr.mpfr,
                            c_string.as_ptr(),
                            base as c_int,
                            mpfr_rnd_t::MPFR_RNDN) == 0 {
                Some(mpfr)
            } else {
                None
            }
        }
    }

    pub fn set(&mut self, other: &Mpfr) {
        unsafe {
            mpfr_set(&mut self.mpfr, &other.mpfr, mpfr_rnd_t::MPFR_RNDN);
        }
    }

    pub fn new_u64_2exp(base: u64, exp: i32) -> Mpfr {
        unsafe {
            let mut mpfr = Mpfr::new();
            mpfr_set_ui_2exp(&mut mpfr.mpfr,
                             base as c_ulong,
                             exp as mpfr_exp_t,
                             mpfr_rnd_t::MPFR_RNDN);
            mpfr
        }
    }

    pub fn new_i64_2exp(base: i64, exp: i32) -> Mpfr {
        unsafe {
            let mut mpfr = Mpfr::new();
            mpfr_set_si_2exp(&mut mpfr.mpfr,
                             base as c_long,
                             exp as mpfr_exp_t,
                             mpfr_rnd_t::MPFR_RNDN);
            mpfr
        }
    }

    pub fn new_mpz_2exp(base: &Mpz, exp: i32) -> Mpfr {
        unsafe {
            let mut mpfr = Mpfr::new();
            mpfr_set_z_2exp(&mut mpfr.mpfr,
                            &base.mpz,
                            exp as mpfr_exp_t,
                            mpfr_rnd_t::MPFR_RNDN);
            mpfr
        }
    }

    pub fn zero(sign: i32) -> Mpfr {
        unsafe {
            let mut mpfr = Mpfr::new();
            mpfr_set_zero(&mut mpfr.mpfr, sign as c_int);
            mpfr
        }
    }

    pub fn inf(sign: i32) -> Mpfr {
        unsafe {
            let mut mpfr = Mpfr::new();
            mpfr_set_inf(&mut mpfr.mpfr, sign as c_int);
            mpfr
        }
    }

    pub fn nan() -> Mpfr {
        unsafe {
            let mut mpfr = Mpfr::new();
            mpfr_set_nan(&mut mpfr.mpfr);
            mpfr
        }
    }

    pub fn get_default_prec() -> usize {
        unsafe { mpfr_get_default_prec() as usize }
    }

    pub fn set_default_prec(precision: usize) {
        unsafe {
            mpfr_set_default_prec(precision as mpfr_prec_t);
        }
    }

    pub fn get_prec(&self) -> usize {
        unsafe { mpfr_get_prec(&self.mpfr) as usize }
    }

    pub fn set_prec(&mut self, precision: usize) {
        unsafe {
            mpfr_set_prec(&mut self.mpfr, precision as mpfr_prec_t);
        }
    }

    // Rounding

    pub fn floor(&self) -> Mpfr {
        unsafe {
            let mut res = Mpfr::new2(self.get_prec());
            mpfr_floor(&mut res.mpfr, &self.mpfr);
            res
        }
    }

    pub fn ceil(&self) -> Mpfr {
        unsafe {
            let mut res = Mpfr::new2(self.get_prec());
            mpfr_ceil(&mut res.mpfr, &self.mpfr);
            res
        }
    }

    pub fn round(&self) -> Mpfr {
        unsafe {
            let mut res = Mpfr::new2(self.get_prec());
            mpfr_round(&mut res.mpfr, &self.mpfr);
            res
        }
    }

    // Mathematical functions

    pub fn sqrt(&self) -> Mpfr {
        unsafe {
            let mut res = Mpfr::new2(self.get_prec());
            mpfr_sqrt(&mut res.mpfr, &self.mpfr, mpfr_rnd_t::MPFR_RNDN);
            res
        }
    }

    pub fn cbrt(&self) -> Mpfr {
        unsafe {
            let mut res = Mpfr::new2(self.get_prec());
            mpfr_cbrt(&mut res.mpfr, &self.mpfr, mpfr_rnd_t::MPFR_RNDN);
            res
        }
    }

    pub fn root(&self, k: u64) -> Mpfr {
        unsafe {
            let mut res = Mpfr::new2(self.get_prec());
            mpfr_root(&mut res.mpfr,
                      &self.mpfr,
                      k as c_ulong,
                      mpfr_rnd_t::MPFR_RNDN);
            res
        }
    }

    pub fn pow(&self, other: &Mpfr) -> Mpfr {
        unsafe {
            let mut res = Mpfr::new2(self.get_prec());
            mpfr_pow(&mut res.mpfr,
                     &self.mpfr,
                     &other.mpfr,
                     mpfr_rnd_t::MPFR_RNDN);
            res
        }
    }

    pub fn abs(&self) -> Mpfr {
        unsafe {
            let mut res = Mpfr::new2(self.get_prec());
            mpfr_abs(&mut res.mpfr, &self.mpfr, mpfr_rnd_t::MPFR_RNDN);
            res
        }
    }

    pub fn exp(&self) -> Mpfr {
        unsafe {
            let mut res = Mpfr::new2(self.get_prec());
            mpfr_exp(&mut res.mpfr, &self.mpfr, mpfr_rnd_t::MPFR_RNDN);
            res
        }
    }

    pub fn log(&self) -> Mpfr {
        unsafe {
            let mut res = Mpfr::new2(self.get_prec());
            mpfr_log(&mut res.mpfr, &self.mpfr, mpfr_rnd_t::MPFR_RNDN);
            res
        }
    }

    pub fn gamma(&self) -> Mpfr {
        unsafe {
            let mut res = Mpfr::new2(self.get_prec());
            mpfr_gamma(&mut res.mpfr, &self.mpfr, mpfr_rnd_t::MPFR_RNDN);
            res
        }
    }

    pub fn lngamma(&self) -> Mpfr {
        unsafe {
            let mut res = Mpfr::new2(self.get_prec());
            mpfr_lngamma(&mut res.mpfr, &self.mpfr, mpfr_rnd_t::MPFR_RNDN);
            res
        }
    }

    pub fn lgamma(&self) -> Mpfr {
        unsafe {
            let mut res = Mpfr::new2(self.get_prec());
            mpfr_lgamma(&mut res.mpfr, &self.mpfr, mpfr_rnd_t::MPFR_RNDN);
            res
        }
    }
}

impl Eq for Mpfr { }
impl PartialEq for Mpfr {
    fn eq(&self, other: &Mpfr) -> bool {
        unsafe { mpfr_cmp(&self.mpfr, &other.mpfr) == 0 }
    }
}

impl Ord for Mpfr {
    fn cmp(&self, other: &Mpfr) -> Ordering {
        let cmp = unsafe { mpfr_cmp(&self.mpfr, &other.mpfr) };
        if cmp == 0 {
            Ordering::Equal
        } else if cmp > 0 {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}

impl PartialOrd for Mpfr {
    fn partial_cmp(&self, other: &Mpfr) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Conversions

impl From<i64> for Mpfr {
    fn from(x: i64) -> Mpfr {
        unsafe {
            let mut mpfr = Mpfr::new();
            mpfr_set_si(&mut mpfr.mpfr, x as c_long, mpfr_rnd_t::MPFR_RNDN);
            mpfr
        }
    }
}

impl From<u64> for Mpfr {
    fn from(x: u64) -> Mpfr {
        unsafe {
            let mut mpfr = Mpfr::new();
            mpfr_set_ui(&mut mpfr.mpfr, x as c_ulong, mpfr_rnd_t::MPFR_RNDN);
            mpfr
        }
    }
}

impl From<f64> for Mpfr {
    fn from(x: f64) -> Mpfr {
        unsafe {
            let mut mpfr = Mpfr::new();
            mpfr_set_d(&mut mpfr.mpfr, x as c_double, mpfr_rnd_t::MPFR_RNDN);
            mpfr
        }
    }
}

impl From<Mpz> for Mpfr {
    fn from(x: Mpz) -> Mpfr {
        unsafe {
            let mut mpfr = Mpfr::new();
            mpfr_set_z(&mut mpfr.mpfr, &x.mpz, mpfr_rnd_t::MPFR_RNDN);
            mpfr
        }
    }
}

impl From<Mpq> for Mpfr {
    fn from(x: Mpq) -> Mpfr {
        unsafe {
            let mut mpfr = Mpfr::new();
            mpfr_set_q(&mut mpfr.mpfr, &x.mpq, mpfr_rnd_t::MPFR_RNDN);
            mpfr
        }
    }
}

impl From<Mpf> for Mpfr {
    fn from(x: Mpf) -> Mpfr {
        unsafe {
            let mut mpfr = Mpfr::new();
            mpfr_set_f(&mut mpfr.mpfr, &x.mpf, mpfr_rnd_t::MPFR_RNDN);
            mpfr
        }
    }
}

impl<'a> Into<i64> for &'a Mpfr {
    fn into(self) -> i64 {
        unsafe { mpfr_get_si(&self.mpfr, mpfr_rnd_t::MPFR_RNDN) as i64 }
    }
}

impl<'a> Into<u64> for &'a Mpfr {
    fn into(self) -> u64 {
        unsafe { mpfr_get_ui(&self.mpfr, mpfr_rnd_t::MPFR_RNDN) as u64 }
    }
}

impl<'a> Into<f64> for &'a Mpfr {
    fn into(self) -> f64 {
        unsafe { mpfr_get_d(&self.mpfr, mpfr_rnd_t::MPFR_RNDN) as f64 }
    }
}

impl<'a> Into<Mpz> for &'a Mpfr {
    fn into(self) -> Mpz {
        unsafe {
            let mut result = Mpz::new();
            mpfr_get_z(&mut result.mpz, &self.mpfr, mpfr_rnd_t::MPFR_RNDN);
            result
        }
    }
}

impl<'a> Into<Mpf> for &'a Mpfr {
    fn into(self) -> Mpf {
        unsafe {
            let mut result = Mpf::new(self.get_prec());
            mpfr_get_f(&mut result.mpf, &self.mpfr, mpfr_rnd_t::MPFR_RNDN);
            result
        }
    }
}

//
// Addition
//
// Supports:
// Mpfr + Mpfr
// Mpfr + f64, f64 + Mpfr
// Mpfr + i64, i64 + Mpfr
//
//

impl<'a, 'b> Add<&'a Mpfr> for &'b Mpfr {
    type Output = Mpfr;
    fn add(self, other: &Mpfr) -> Mpfr {
        unsafe {
            let mut res = Mpfr::new2(cmp::max(self.get_prec(), other.get_prec()));
            mpfr_add(&mut res.mpfr,
                     &self.mpfr,
                     &other.mpfr,
                     mpfr_rnd_t::MPFR_RNDN);
            res
        }
    }
}

impl<'a> Add<&'a Mpfr> for Mpfr {
    type Output = Mpfr;
    #[inline]
    fn add(mut self, other: &Mpfr) -> Mpfr {
        unsafe {
            mpfr_add(&mut self.mpfr,
                     &self.mpfr,
                     &other.mpfr,
                     mpfr_rnd_t::MPFR_RNDN);
            self
        }
    }
}

impl Add<Mpfr> for f64 {
	type Output = Mpfr;
    fn add(self, other: Mpfr) -> Mpfr {
        unsafe {
            let mut res = Mpfr::new2(other.get_prec());
            mpfr_add_d(&mut res.mpfr,
                       &other.mpfr,
                       self as c_double,
                       mpfr_rnd_t::MPFR_RNDN);
            res
        }
    }
}

impl<'a> Add<&'a Mpfr> for f64 {
	type Output = Mpfr;
    fn add(self, other: &'a Mpfr) -> Mpfr {
        unsafe {
            let mut res = Mpfr::new2(other.get_prec());
            mpfr_add_d(&mut res.mpfr,
                       &other.mpfr,
                       self as c_double,
                       mpfr_rnd_t::MPFR_RNDN);
            res
        }
    }
}

impl Add<f64> for Mpfr {
	type Output = Mpfr;
    #[inline]
    fn add(mut self, other: f64) -> Mpfr {
        unsafe {
            mpfr_add_d(&mut self.mpfr,
                       &self.mpfr,
                       other as c_double,
                       mpfr_rnd_t::MPFR_RNDN);
            self
        }
    }
}

impl<'a> Add<f64> for &'a Mpfr {
	type Output = Mpfr;
    fn add(self, other: f64) -> Mpfr {
        unsafe {
            let mut res = Mpfr::new2(self.get_prec());
            mpfr_add_d(&mut res.mpfr,
                       &self.mpfr,
                       other as c_double,
                       mpfr_rnd_t::MPFR_RNDN);
            res
        }
    }
}

impl Add<Mpfr> for i64 {
	type Output = Mpfr;
    fn add(self, other: Mpfr) -> Mpfr {
        unsafe {
            let mut res = Mpfr::new2(other.get_prec());
            mpfr_add_si(&mut res.mpfr,
                        &other.mpfr,
                        self as c_long,
                        mpfr_rnd_t::MPFR_RNDN);
            res
        }
    }
}

impl<'a> Add<&'a Mpfr> for i64 {
	type Output = Mpfr;
    fn add(self, other: &'a Mpfr) -> Mpfr {
        unsafe {
            let mut res = Mpfr::new2(other.get_prec());
            mpfr_add_si(&mut res.mpfr,
                        &other.mpfr,
                        self as c_long,
                        mpfr_rnd_t::MPFR_RNDN);
            res
        }
    }
}

impl Add<i64> for Mpfr {
	type Output = Mpfr;
    #[inline]
    fn add(mut self, other: i64) -> Mpfr {
        unsafe {
            mpfr_add_si(&mut self.mpfr,
                        &self.mpfr,
                        other as c_long,
                        mpfr_rnd_t::MPFR_RNDN);
            self
        }
    }
}

impl<'a> Add<i64> for &'a Mpfr {
	type Output = Mpfr;
    fn add(self, other: i64) -> Mpfr {
        unsafe {
            let mut res = Mpfr::new2(self.get_prec());
            mpfr_add_si(&mut res.mpfr,
                        &self.mpfr,
                        other as c_long,
                        mpfr_rnd_t::MPFR_RNDN);
            res
        }
    }
}

//
// Subtraction
//
// Supports:
// Mpfr - Mpfr
// Mpfr - f64, f64 - Mpfr
// Mpfr - i64, i64 - Mpfr
//
//

impl<'a, 'b> Sub<&'a Mpfr> for &'b Mpfr {
    type Output = Mpfr;
    fn sub(self, other: &Mpfr) -> Mpfr {
        unsafe {
            let mut res = Mpfr::new2(cmp::max(self.get_prec(), other.get_prec()));
            mpfr_sub(&mut res.mpfr,
                     &self.mpfr,
                     &other.mpfr,
                     mpfr_rnd_t::MPFR_RNDN);
            res
        }
    }
}

impl<'a> Sub<&'a Mpfr> for Mpfr {
    type Output = Mpfr;
    #[inline]
    fn sub(mut self, other: &Mpfr) -> Mpfr {
        unsafe {
            mpfr_sub(&mut self.mpfr,
                     &self.mpfr,
                     &other.mpfr,
                     mpfr_rnd_t::MPFR_RNDN);
            self
        }
    }
}

impl Sub<Mpfr> for f64 {
	type Output = Mpfr;
    fn sub(self, other: Mpfr) -> Mpfr {
        unsafe {
            let mut res = Mpfr::new2(other.get_prec());
            mpfr_d_sub(&mut res.mpfr,
                       self as c_double,
                       &other.mpfr,
                       mpfr_rnd_t::MPFR_RNDN);
            res
        }
    }
}

impl<'a> Sub<&'a Mpfr> for f64 {
	type Output = Mpfr;
    fn sub(self, other: &'a Mpfr) -> Mpfr {
        unsafe {
            let mut res = Mpfr::new2(other.get_prec());
            mpfr_d_sub(&mut res.mpfr,
                       self as c_double,
                       &other.mpfr,
                       mpfr_rnd_t::MPFR_RNDN);
            res
        }
    }
}

impl Sub<f64> for Mpfr {
	type Output = Mpfr;
    #[inline]
    fn sub(mut self, other: f64) -> Mpfr {
        unsafe {
            mpfr_sub_d(&mut self.mpfr,
                       &self.mpfr,
                       other as c_double,
                       mpfr_rnd_t::MPFR_RNDN);
            self
        }
    }
}

impl<'a> Sub<f64> for &'a Mpfr {
	type Output = Mpfr;
    fn sub(self, other: f64) -> Mpfr {
        unsafe {
            let mut res = Mpfr::new2(self.get_prec());
            mpfr_sub_d(&mut res.mpfr,
                       &self.mpfr,
                       other as c_double,
                       mpfr_rnd_t::MPFR_RNDN);
            res
        }
    }
}

impl Sub<Mpfr> for i64 {
	type Output = Mpfr;
    fn sub(self, other: Mpfr) -> Mpfr {
        unsafe {
            let mut res = Mpfr::new2(other.get_prec());
            mpfr_si_sub(&mut res.mpfr,
                        self as c_long,
                        &other.mpfr,
                        mpfr_rnd_t::MPFR_RNDN);
            res
        }
    }
}

impl<'a> Sub<&'a Mpfr> for i64 {
	type Output = Mpfr;
    fn sub(self, other: &'a Mpfr) -> Mpfr {
        unsafe {
            let mut res = Mpfr::new2(other.get_prec());
            mpfr_si_sub(&mut res.mpfr,
                        self as c_long,
                        &other.mpfr,
                        mpfr_rnd_t::MPFR_RNDN);
            res
        }
    }
}

impl Sub<i64> for Mpfr {
	type Output = Mpfr;
    #[inline]
    fn sub(mut self, other: i64) -> Mpfr {
        unsafe {
            mpfr_sub_si(&mut self.mpfr,
                        &self.mpfr,
                        other as c_long,
                        mpfr_rnd_t::MPFR_RNDN);
            self
        }
    }
}

impl<'a> Sub<i64> for &'a Mpfr {
	type Output = Mpfr;
    fn sub(self, other: i64) -> Mpfr {
        unsafe {
            let mut res = Mpfr::new2(self.get_prec());
            mpfr_sub_si(&mut res.mpfr,
                        &self.mpfr,
                        other as c_long,
                        mpfr_rnd_t::MPFR_RNDN);
            res
        }
    }
}

//
// Multiplication
//
// Supports:
// Mpfr * Mpfr
// Mpfr * f64, f64 * Mpfr
// Mpfr * i64, i64 * Mpfr
//
//

impl<'a, 'b> Mul<&'a Mpfr> for &'b Mpfr {
    type Output = Mpfr;
    fn mul(self, other: &Mpfr) -> Mpfr {
        unsafe {
            let mut res = Mpfr::new2(cmp::max(self.get_prec(), other.get_prec()));
            mpfr_mul(&mut res.mpfr,
                     &self.mpfr,
                     &other.mpfr,
                     mpfr_rnd_t::MPFR_RNDN);
            res
        }
    }
}

impl<'a> Mul<&'a Mpfr> for Mpfr {
    type Output = Mpfr;

    #[inline]
    fn mul(mut self, other: &Mpfr) -> Mpfr {
        unsafe {
            mpfr_mul(&mut self.mpfr,
                     &self.mpfr,
                     &other.mpfr,
                     mpfr_rnd_t::MPFR_RNDN);
            self
        }
    }
}

impl Mul<Mpfr> for f64 {
	type Output = Mpfr;
    fn mul(self, other: Mpfr) -> Mpfr {
        unsafe {
            let mut res = Mpfr::new2(other.get_prec());
            mpfr_mul_d(&mut res.mpfr,
                       &other.mpfr,
                       self as c_double,
                       mpfr_rnd_t::MPFR_RNDN);
            res
        }
    }
}

impl<'a> Mul<&'a Mpfr> for f64 {
	type Output = Mpfr;
    fn mul(self, other: &'a Mpfr) -> Mpfr {
        unsafe {
            let mut res = Mpfr::new2(other.get_prec());
            mpfr_mul_d(&mut res.mpfr,
                       &other.mpfr,
                       self as c_double,
                       mpfr_rnd_t::MPFR_RNDN);
            res
        }
    }
}

impl Mul<f64> for Mpfr {
	type Output = Mpfr;
    #[inline]
    fn mul(mut self, other: f64) -> Mpfr {
        unsafe {
            mpfr_mul_d(&mut self.mpfr,
                       &self.mpfr,
                       other as c_double,
                       mpfr_rnd_t::MPFR_RNDN);
            self
        }
    }
}

impl<'a> Mul<f64> for &'a Mpfr {
	type Output = Mpfr;
    fn mul(self, other: f64) -> Mpfr {
        unsafe {
            let mut res = Mpfr::new2(self.get_prec());
            mpfr_mul_d(&mut res.mpfr,
                       &self.mpfr,
                       other as c_double,
                       mpfr_rnd_t::MPFR_RNDN);
            res
        }
    }
}

impl Mul<Mpfr> for i64 {
	type Output = Mpfr;
    fn mul(self, other: Mpfr) -> Mpfr {
        unsafe {
            let mut res = Mpfr::new2(other.get_prec());
            mpfr_mul_si(&mut res.mpfr,
                        &other.mpfr,
                        self as c_long,
                        mpfr_rnd_t::MPFR_RNDN);
            res
        }
    }
}

impl<'a> Mul<&'a Mpfr> for i64 {
	type Output = Mpfr;
    fn mul(self, other: &'a Mpfr) -> Mpfr {
        unsafe {
            let mut res = Mpfr::new2(other.get_prec());
            mpfr_mul_si(&mut res.mpfr,
                        &other.mpfr,
                        self as c_long,
                        mpfr_rnd_t::MPFR_RNDN);
            res
        }
    }
}

impl Mul<i64> for Mpfr {
	type Output = Mpfr;
    #[inline]
    fn mul(mut self, other: i64) -> Mpfr {
        unsafe {
            mpfr_mul_si(&mut self.mpfr,
                        &self.mpfr,
                        other as c_long,
                        mpfr_rnd_t::MPFR_RNDN);
            self
        }
    }
}

impl<'a> Mul<i64> for &'a Mpfr {
	type Output = Mpfr;
    fn mul(self, other: i64) -> Mpfr {
        unsafe {
            let mut res = Mpfr::new2(self.get_prec());
            mpfr_mul_si(&mut res.mpfr,
                        &self.mpfr,
                        other as c_long,
                        mpfr_rnd_t::MPFR_RNDN);
            res
        }
    }
}

//
// Division
//
// Supports:
// Mpfr / Mpfr
// Mpfr / f64, f64 / Mpfr
// Mpfr / i64, i64 / Mpfr
//
//

impl<'a, 'b> Div<&'a Mpfr> for &'b Mpfr {
    type Output = Mpfr;
    fn div(self, other: &Mpfr) -> Mpfr {
        unsafe {
            if mpfr_cmp_ui(&other.mpfr, 0) == 0 {
                panic!("divide by zero")
            }

            let mut res = Mpfr::new2(cmp::max(self.get_prec(), other.get_prec()));
            mpfr_div(&mut res.mpfr,
                     &self.mpfr,
                     &other.mpfr,
                     mpfr_rnd_t::MPFR_RNDN);
            res
        }
    }
}

impl<'a> Div<&'a Mpfr> for Mpfr {
    type Output = Mpfr;
    #[inline]
    fn div(mut self, other: &Mpfr) -> Mpfr {
        unsafe {
            if mpfr_cmp_ui(&other.mpfr, 0) == 0 {
                panic!("divide by zero")
            }

            mpfr_div(&mut self.mpfr,
                     &self.mpfr,
                     &other.mpfr,
                     mpfr_rnd_t::MPFR_RNDN);
            self
        }
    }
}

impl Div<Mpfr> for f64 {
	type Output = Mpfr;
    fn div(self, other: Mpfr) -> Mpfr {
        unsafe {
            if mpfr_cmp_ui(&other.mpfr, 0) == 0 {
                panic!("divide by zero")
            }

            let mut res = Mpfr::new2(other.get_prec());
            mpfr_d_div(&mut res.mpfr,
                       self as c_double,
                       &other.mpfr,
                       mpfr_rnd_t::MPFR_RNDN);
            res
        }
    }
}

impl<'a> Div<&'a Mpfr> for f64 {
	type Output = Mpfr;
    fn div(self, other: &'a Mpfr) -> Mpfr {
        unsafe {
            if mpfr_cmp_ui(&other.mpfr, 0) == 0 {
                panic!("divide by zero")
            }

            let mut res = Mpfr::new2(other.get_prec());
            mpfr_d_div(&mut res.mpfr,
                       self as c_double,
                       &other.mpfr,
                       mpfr_rnd_t::MPFR_RNDN);
            res
        }
    }
}

impl Div<f64> for Mpfr {
	type Output = Mpfr;
    #[inline]
    fn div(mut self, other: f64) -> Mpfr {
        unsafe {
            if other == 0.0 {
                panic!("divide by zero")
            }

            mpfr_div_d(&mut self.mpfr,
                       &self.mpfr,
                       other as c_double,
                       mpfr_rnd_t::MPFR_RNDN);
            self
        }
    }
}

impl<'a> Div<f64> for &'a Mpfr {
	type Output = Mpfr;
    fn div(self, other: f64) -> Mpfr {
        unsafe {
            if other == 0.0 {
                panic!("divide by zero")
            }

            let mut res = Mpfr::new2(self.get_prec());
            mpfr_div_d(&mut res.mpfr,
                       &self.mpfr,
                       other as c_double,
                       mpfr_rnd_t::MPFR_RNDN);
            res
        }
    }
}

impl Div<Mpfr> for i64 {
	type Output = Mpfr;
    fn div(self, other: Mpfr) -> Mpfr {
        unsafe {
            if mpfr_cmp_ui(&other.mpfr, 0) == 0 {
                panic!("divide by zero")
            }

            let mut res = Mpfr::new2(other.get_prec());
            mpfr_si_div(&mut res.mpfr,
                        self as c_long,
                        &other.mpfr,
                        mpfr_rnd_t::MPFR_RNDN);
            res
        }
    }
}

impl<'a> Div<&'a Mpfr> for i64 {
	type Output = Mpfr;
    fn div(self, other: &'a Mpfr) -> Mpfr {
        unsafe {
            if mpfr_cmp_ui(&other.mpfr, 0) == 0 {
                panic!("divide by zero")
            }

            let mut res = Mpfr::new2(other.get_prec());
            mpfr_si_div(&mut res.mpfr,
                        self as c_long,
                        &other.mpfr,
                        mpfr_rnd_t::MPFR_RNDN);
            res
        }
    }
}

impl Div<i64> for Mpfr {
	type Output = Mpfr;
    #[inline]
    fn div(mut self, other: i64) -> Mpfr {
        unsafe {
            if other == 0 {
                panic!("divide by zero")
            }

            mpfr_div_si(&mut self.mpfr,
                        &self.mpfr,
                        other as c_long,
                        mpfr_rnd_t::MPFR_RNDN);
            self
        }
    }
}

impl<'a> Div<i64> for &'a Mpfr {
	type Output = Mpfr;
    fn div(self, other: i64) -> Mpfr {
        unsafe {
            if other == 0 {
                panic!("divide by zero")
            }

            let mut res = Mpfr::new2(self.get_prec());
            mpfr_div_si(&mut res.mpfr,
                        &self.mpfr,
                        other as c_long,
                        mpfr_rnd_t::MPFR_RNDN);
            res
        }
    }
}

// Negation

impl<'b> Neg for &'b Mpfr {
    type Output = Mpfr;
    fn neg(self) -> Mpfr {
        unsafe {
            let mut res = Mpfr::new2(self.get_prec());
            mpfr_neg(&mut res.mpfr, &self.mpfr, mpfr_rnd_t::MPFR_RNDN);
            res
        }
    }
}

impl Neg for Mpfr {
    type Output = Mpfr;
    #[inline]
    fn neg(mut self) -> Mpfr {
        unsafe {
            mpfr_neg(&mut self.mpfr, &self.mpfr, mpfr_rnd_t::MPFR_RNDN);
            self
        }
    }
}

gen_overloads!(Mpfr);

// rustc_serialize support
impl Decodable for Mpfr {
    fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
        let s = try!(d.read_str());
        match Mpfr::new_from_str(s, 10) {
            Some(val) => Ok(val),
            None => Err(d.error("Cannot parse decimal float")),
        }
    }
}

impl Encodable for Mpfr {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        s.emit_str(&self.to_string())
    }
}
