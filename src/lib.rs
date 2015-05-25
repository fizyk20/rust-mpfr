extern crate libc;

macro_rules! gen_overloads_inner {
    ($tr:ident, $meth:ident, $T:ident) => {
        impl<'a> $tr <&'a $T> for $T {
            type Output = $T;
            fn $meth(self, other: &'a $T) -> $T {
                (&self).$meth(other)
            }
        }
        impl<'a> $tr <$T> for &'a $T {
            type Output = $T;
            fn $meth(self, other: $T) -> $T {
                self.$meth(&other)
            }
        }
        impl $tr<$T> for $T {
            type Output = $T;
            fn $meth(self, other: $T) -> $T {
                (&self).$meth(&other)
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
        impl Neg for $T {
            type Output = $T;
            fn neg(self) -> $T {
                -(&self)
            }
        }
    }
}

pub mod mpfr;

#[cfg(test)]
mod test;