
use num_traits::PrimInt;
use num::rational::Ratio;
use std::fmt;
use std::ops::Mul;

#[derive(Debug)]
pub struct CFrac<I: PrimInt> {
    values: Vec<I>
}

impl <I: PrimInt> CFrac<I> {

    pub fn new(mut p: I, mut q: I) -> CFrac<I> {
        let zero = I::zero();
        assert!(q > zero, "Denominator in CFrac must be positive");
        let mut values = Vec::new();
        let mut a;
        loop {
            a = p / q;
            p = p % q;
            values.push(a);
            if p == zero { break; }
            a = q / p;
            q = q % p;
            values.push(a);
            if q == zero { break; }
        }
        CFrac { values }
    }

}

impl <I: num::Integer + PrimInt> From<&CFrac<I>> for Ratio<I> {
    fn from(cf: &CFrac<I>) -> Self {
        let mut vals = cf.values.iter().rev();
        let mut num = *vals.next().unwrap(); // Cannot fail
        let mut den = I::one();
        for a in vals {
            let temp = num;
            num = *a * num + den;
            den = temp;
        }
        Ratio::new(num, den)
    }
}
            


impl <I: PrimInt + fmt::Display> fmt::Display for CFrac<I> {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let vals: Vec<String> =
            self.values
            .iter()
            .map(|i| i.to_string())
            .collect();
        let s = [vals[0..2].join("; "), vals[2..].join(", ")].join(", "); 
        write!(f, "[ {}]", s)
    }
}

