use libc::{c_int, c_void};
use std::mem::uninitialized;

type mpfr_prec_t = c_int;
type mpfr_sign_t = c_int;
type mpfr_exp_t = c_int;

#[repr(C)]
pub struct mpfr_struct {
    _mpfr_prec: mpfr_prec_t,
    _mpfr_sign: mpfr_sign_t,
    _mpfr_exp: mpfr_exp_t,
    _mpfr_d: *mut c_void
}

pub type mpfr_srcptr = *const mpfr_struct;
pub type mpfr_ptr = *mut mpfr_struct;

#[link(name = "mpfr")]
extern "C" {
	fn mpfr_init2(x: mpfr_ptr, prec: mpfr_prec_t);
	fn mpfr_clear(x: mpfr_ptr);
}

pub struct Mpfr {
    pub mpfr: mpfr_struct,
}

unsafe impl Send for Mpfr { }

impl Drop for Mpfr {
    fn drop(&mut self) { unsafe { mpfr_clear(&mut self.mpfr) } }
}

impl Mpfr {
    pub fn new(precision: c_int) -> Mpfr {
        unsafe {
            let mut mpfr = uninitialized();
            mpfr_init2(&mut mpfr, precision);
            Mpfr { mpfr: mpfr }
        }
    }
}
