use libc::{c_char, c_int, c_ulong, c_long, c_float, c_double, c_void};
use gmp::mpz::mpz_srcptr;
use gmp::mpq::mpq_srcptr;
use gmp::mpf::mpf_srcptr;
use std::mem::uninitialized;
use std::cmp::{Eq, PartialEq, Ord, PartialOrd, Ordering};

type mpfr_prec_t = c_int;
type mpfr_sign_t = c_int;
type mpfr_exp_t = c_int;

#[repr(C)]
pub enum mpfr_rnd_t {
	MPFR_RNDN=0,  /* round to nearest, with ties to even */
	MPFR_RNDZ,    /* round toward zero */
	MPFR_RNDU,    /* round toward +Inf */
	MPFR_RNDD,    /* round toward -Inf */
	MPFR_RNDA,    /* round away from zero */
	MPFR_RNDF,    /* faithful rounding (not implemented yet) */
	MPFR_RNDNA=-1 /* round to nearest, with ties away from zero (mpfr_rouund) */
}

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
	// Initialization
	fn mpfr_init2(x: mpfr_ptr, prec: mpfr_prec_t);
	fn mpfr_clear(x: mpfr_ptr);
	fn mpfr_set_default_prec(prec: mpfr_prec_t);
	fn mpfr_get_default_prec() -> mpfr_prec_t;
	fn mpfr_set_prec(x: mpfr_ptr, prec: mpfr_prec_t);
	fn mpfr_get_prec(x: mpfr_srcptr) -> mpfr_prec_t;
	
	// Assignment
	fn mpfr_set(rop: mpfr_ptr, op: mpfr_ptr, rnd: mpfr_rnd_t) -> c_int;
	fn mpfr_set_ui(rop: mpfr_ptr, op: c_ulong, rnd: mpfr_rnd_t) -> c_int;
	fn mpfr_set_si(rop: mpfr_ptr, op: c_long, rnd: mpfr_rnd_t) -> c_int;
	fn mpfr_set_flt(rop: mpfr_ptr, op: c_float, rnd: mpfr_rnd_t) -> c_int;
	fn mpfr_set_d(rop: mpfr_ptr, op: c_double, rnd: mpfr_rnd_t) -> c_int;
	fn mpfr_set_z(rop: mpfr_ptr, op: mpz_srcptr, rnd: mpfr_rnd_t) -> c_int;
	fn mpfr_set_q(rop: mpfr_ptr, op: mpq_srcptr, rnd: mpfr_rnd_t) -> c_int;
	fn mpfr_set_f(rop: mpfr_ptr, op: mpf_srcptr, rnd: mpfr_rnd_t) -> c_int;
	fn mpfr_set_ui_2exp(rop: mpfr_ptr, op: c_ulong, e: mpfr_exp_t, rnd: mpfr_rnd_t) -> c_int;
	fn mpfr_set_si_2exp(rop: mpfr_ptr, op: c_long, e: mpfr_exp_t, rnd: mpfr_rnd_t) -> c_int;
	fn mpfr_set_z_2exp(rop: mpfr_ptr, op: mpz_srcptr, e: mpfr_exp_t, rnd: mpfr_rnd_t) -> c_int;
	fn mpfr_set_str(rop: mpfr_ptr, s: *const c_char, base: c_int, rnd: mpfr_rnd_t) -> c_int;
	fn mpfr_strtofr(rop: mpfr_ptr, nptr: *const c_char, endptr: *mut c_void, base: c_int, rnd: mpfr_rnd_t) -> c_int;
	fn mpfr_set_nan(x: mpfr_ptr);
	fn mpfr_set_inf(x: mpfr_ptr, sign: c_int);
	fn mpfr_set_zero(x: mpfr_ptr, sign: c_int);
	fn mpfr_swap(x: mpfr_ptr, y: mpfr_ptr);
	
	// Initialization and assignment
	fn mpfr_init_set(rop: mpfr_ptr, op: mpfr_srcptr, rnd: mpfr_rnd_t) -> c_int;
	
	// Comparison
	fn mpfr_cmp(op1: mpfr_srcptr, op2: mpfr_srcptr) -> c_int;
}

pub struct Mpfr {
    pub mpfr: mpfr_struct,
}

unsafe impl Send for Mpfr { }

impl Drop for Mpfr {
    fn drop(&mut self) { unsafe { mpfr_clear(&mut self.mpfr) } }
}

impl Clone for Mpfr {
    fn clone(&self) -> Mpfr {
        unsafe {
            let mut mpfr = uninitialized();
            mpfr_init_set(&mut mpfr, &self.mpfr, mpfr_rnd_t::MPFR_RNDN);
            Mpfr { mpfr: mpfr }
        }
    }
}

impl Mpfr {
    pub fn new(precision: c_int) -> Mpfr {
        unsafe {
            let mut mpfr = uninitialized();
            mpfr_init2(&mut mpfr, precision);
            Mpfr { mpfr: mpfr }
        }
    }
    
    pub fn get_default_prec() -> i64 {
    	unsafe {
    		mpfr_get_default_prec() as i64
    	}
    }
    
    pub fn set_default_prec(precision: i64) {
    	unsafe {
    		mpfr_set_default_prec(precision as c_int);
    	}
    }
    
    pub fn get_prec(&self) -> i64 {
    	unsafe {
    		mpfr_get_prec(&self.mpfr) as i64
    	}
    }
    
    pub fn set_prec(&mut self, precision: i64) {
    	unsafe {
    		mpfr_set_prec(&mut self.mpfr, precision as c_int);
    	}
    }
}

impl Eq for Mpfr { }
impl PartialEq for Mpfr {
	fn eq(&self, other: &Mpfr) -> bool {
		unsafe {
			mpfr_cmp(&self.mpfr, &other.mpfr) == 0
		}
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
