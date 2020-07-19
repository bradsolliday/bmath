
use num_traits::PrimInt;
use num::rational::Ratio;
use std::fmt;

/// A sequence of integers [a0; a1, a2, a3, ...] where all but the first
/// element must be positive.
///
/// We evaluate a finite continued fraction as
///
/// [a0; a1, a2, ..., an] = a0 + 1 / (a1 + 1 / (a2 + 1 / (... + 1 / an ...) ) )
///
/// We evaluate an infinte continued fraction as
///
/// [a0; a1, a2, ...] = lim_{ n -> infinity } [a0; a1, a2, ..., an]
///
/// If representing a rational number, the sequence is finite and terminates
/// in a value greater than 1. If representing a irrational number, the
/// sequence is infinite. Every real number r is uniquely identified by 
/// a continued fraction 
#[derive(Debug, PartialEq)]
pub struct CFrac<I: PrimInt> {
    values: Vec<I>
}

// I should make it iterable. If for some reason I wanted to do that. Would be
// easy considering my implementation.
impl <I: num::Integer + PrimInt> CFrac<I> {

    /// Returns the continued fraction (CFrac) of the rational number
    /// p / q
    ///
    /// # Examples
    /// ```
    /// use bmath::CFrac;
    /// use num::rational::Ratio;
    ///
    /// let (p, q) = (-4, 5);
    /// 
    /// let cf = CFrac::<i64>::from_fraction(p, q);
    ///
    /// assert_eq!(Ratio::from(&cf), Ratio::new(p,q));
    ///
    /// let (p, q) = (47, 51);
    ///
    /// let cf = CFrac::<usize>::from_fraction(p, q);
    ///
    /// assert_eq!(Ratio::from(&cf), Ratio::new(p,q));
    /// ```
    pub fn from_fraction(mut p: I, mut q: I) -> CFrac<I> {
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

    /// Returns the continued fraction of the given ratio.
    ///
    /// # Examples
    ///
    /// ```
    /// use bmath::CFrac;
    /// use num::rational::Ratio;
    ///
    /// let [p, q]: [usize; 2] = [17, 18];
    ///
    /// let from_frac = CFrac::from_fraction(p, q);
    ///
    /// let from_ratio = CFrac::from_ratio(Ratio::new(p, q));
    ///
    /// assert_eq!(from_frac, from_ratio);
    /// ```
    pub fn from_ratio(ratio: Ratio<I>) -> CFrac<I> {
        CFrac::from_fraction(*ratio.numer(), *ratio.denom())
    }

    /// Returns a vector of Ratios representing the continued fraction's
    /// convergents.
    /// 
    /// The kth convergent of a continued fraction [a0; a1, a2, ..., an] is
    /// [a0; a1, a2, ..., ak].
    ///
    /// If cf = [a0; a1, a2, ..., an], then
    ///
    /// cf.convergents\[0\] = \[a0\],
    /// cf.convergents\[1\] = \[a0; a1\],
    /// cf.convergents\[4\] = \[a0; a1, a2, a3, a4\],
    /// and so on.
    ///
    /// # Examples
    /// 
    /// ```
    /// use bmath::CFrac;
    /// use num::rational::Ratio;
    ///
    /// // 317_811 is the 27th fibonacci number
    /// // 196_418 is the 26th fibonacci number
    /// // Their ratio is an approximation of the golden ratio
    /// let golden = CFrac::from_fraction(317811, 196418);
    ///
    /// // golden = [1; 1, ..., 1, 1, 2] 
    ///
    /// let fib_ratios = golden.convergents();
    ///
    /// // The fibonacci numbers go as:
    /// // 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...
    ///
    /// assert_eq!(fib_ratios[0], Ratio::new( 1,  1));
    /// assert_eq!(fib_ratios[1], Ratio::new( 2,  1));
    /// assert_eq!(fib_ratios[2], Ratio::new( 3,  2));
    /// assert_eq!(fib_ratios[3], Ratio::new( 5,  3));
    /// assert_eq!(fib_ratios[4], Ratio::new( 8,  5));
    /// assert_eq!(fib_ratios[5], Ratio::new(13,  8));
    /// assert_eq!(fib_ratios[6], Ratio::new(21, 13));
    /// assert_eq!(fib_ratios[7], Ratio::new(34, 21));
    /// assert_eq!(fib_ratios[8], Ratio::new(55, 34));
    /// assert_eq!(fib_ratios[9], Ratio::new(89, 55));
    /// ```
    ///
    /// To see how it is that the fibonacci sequence arose from the convergents
    /// of the ratio of two fibonacci numbers, see the source code.
    // The goal is to avoid repeating calculations as you would if you
    // we're to calcualte each convergence indepedently and naively.
    // The idea is as follows:
    //
    // The algebraic expression for [a0; a1, ..., ak, a{k+1}] is just the
    // algebraic expression for [a0; a1, ..., ak] with ak + 1/a{k+1}
    // substituted everywhere ak is found.
    //
    // Hence, we should seprate the numerator and denominator of
    // [a0; a1, ..., ak] into parts dependent on ak and parts not
    // dependent on ak.
    // If you try this by hand with a few examples, you see that the
    // convergent's numerator and denomator depend linearly on ak.
    // 
    // So let's see if we can prove this and define
    // Nk = N{k-1}*ak + N{k-2}
    // Dk = D{k-1}*ak + D{k-2}
    // with (N{-1}, N{-2}) = (1, 0),
    //      (D{-1}, D{-2}) = (0, 1).
    //
    // Note that with our particular choice of negatively indexed N and D
    // [a0] = a0 = (N{-1}*a0 + N{-2}*a{-2}) / (D{-1}*a0 + D{-2})
    //
    // Now, assume by induction that
    // [a0; a1, ..., ak] = Nk / Dk = (N{k-1}*ak + N{k-2}) / (D{k-1}*ak + D{k-2})
    //
    // then (substituting ak + 1/a{k+1} for ak) to get the next convergent:
    //
    // [a0; a1, ..., a{k+1}] = (N{k-1}*(ak + 1/a{k+1}) + N{k-2})
    //                       / (D{k-1}*(ak + 1/a{k+1}) + D{k-2})
    //
    //                       = ((N{k-1}*ak + N{k-2})*a{k+1} + N{k-1})
    //                       / ((D{k-1}*ak + D{k-2})*a{k+1} + D{k-1})
    //
    //                       = (Nk*a{k+1} + N{k-1}
    //                       / (Dk*a{k+1} + D{k-1}
    //
    //                       = N{k / Dk
    // 
    // By induction, [a0; a1, ..., ak] = Nk / Dk, and we can use the simple
    // formula defined above to calculate Nk and Dk.
    // 
    // NOTE: If ak = 1 for all k, then the expressions for Nk and Dk become
    // that of successive elements of the fibonacci sequence. Since both
    // sequences start with (in a manner) 0, 1, we see that the convergents
    // of a continued fraction of the form [1; 1, 1, 1, ...] are the ratios
    // of consecutive fibonacci numbers.
    pub fn convergents(&self) -> Vec<Ratio<I>> {
        let (mut n1, mut n0) = (I::one(),  I::zero());
        let (mut d1, mut d0) = (I::zero(), I::one());
        let mut convergent_vec: Vec<Ratio<I>> =
            Vec::<Ratio<I>>::with_capacity(self.values.len());
        let mut i = 0;
        while i + 1 < self.values.len() {
            // Loop is partially unrolled to avoid temporary variable for swap
            let a = self.values[i];
            n0 = n1*a + n0;
            d0 = d1*a + d0;
            convergent_vec.push(Ratio::new(n0, d0));
            i += 1;

            let a = self.values[i];
            n1 = n0*a + n1;
            d1 = d0*a + d1;
            convergent_vec.push(Ratio::new(n1, d1));
            i += 1;
        }
        if i < self.values.len() {
            let a = self.values[i];
            n0 = n1*a + n0;
            d0 = d1*a + d0;
            convergent_vec.push(Ratio::new(n0, d0));
        }
        convergent_vec
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
        write!(f, "[{}]", s)
    }
}
