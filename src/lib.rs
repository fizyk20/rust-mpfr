#![crate_name = "rust_mpfr"]
#![allow(non_camel_case_types)]

extern crate libc;
extern crate gmp;
extern crate rustc_serialize;

macro_rules! gen_overloads_inner {
    ($tr:ident, $meth:ident, $T:ident) => {
        impl<'a> $tr <$T> for &'a $T {
            type Output = $T;
            #[inline]
            fn $meth(self, other: $T) -> $T {
                self.$meth(&other)
            }
        }
        impl $tr<$T> for $T {
            type Output = $T;
            #[inline]
            fn $meth(self, other: $T) -> $T {
                self.$meth(&other)
            }
        }
    }
}

macro_rules! gen_overloads {
    ($T:ident) => {
        gen_overloads_inner!(Add, add, $T);
        gen_overloads_inner!(Sub, sub, $T);
        gen_overloads_inner!(Mul, mul, $T);
        gen_overloads_inner!(Div, div, $T);
    }
}

#[macro_export]
macro_rules! mpfr {
    ($t:tt) => {
        $crate::mpfr::Mpfr::new_from_str(stringify!($t), 10).expect("Invalid floating point literal")
    }
}

pub mod mpfr;

#[cfg(test)]
mod test;
