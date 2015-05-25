use libc::{c_int, c_void};

#[repr(C)]
pub struct mpfr_struct {
    _mpfr_prec: c_int,
    _mpfr_sign: c_int,
    _mpfr_exp: c_int,
    _mpfr_d: *mut c_void
}

pub type mpfr_srcptr = *const mpfr_struct;
pub type mpfr_ptr = *mut mpfr_struct;

#[link(name = "mpfr")]
extern "C" {
	
}