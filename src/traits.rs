use std::ops::*;

/// Definition of zero trait.
pub trait Zero { fn zero() -> Self; }

/// Implementation of zero trait for primitive types.
impl Zero for i8 { fn zero() -> Self { 0 } }
impl Zero for i16 { fn zero() -> Self { 0 } }
impl Zero for i32 { fn zero() -> Self { 0 } }
impl Zero for i64 { fn zero() -> Self { 0 } }
impl Zero for u8 { fn zero() -> Self { 0 } }
impl Zero for u16 { fn zero() -> Self { 0 } }
impl Zero for u32 { fn zero() -> Self { 0 } }
impl Zero for u64 { fn zero() -> Self { 0 } }
impl Zero for isize { fn zero() -> Self { 0 } }
impl Zero for usize { fn zero() -> Self { 0 } }
impl Zero for f32 { fn zero() -> Self { 0. } }
impl Zero for f64 { fn zero() -> Self { 0. } }

/// Definition of traits for vector components.
pub trait IsNumerical<T> : Zero + Add<Output = T> + AddAssign + Copy + Mul<Output = T> {}

/// Implementation of trait for vector components.
impl<T> IsNumerical<T> for T
    where T: Zero + Add<Output = T> + AddAssign + Copy + Mul<Output = T> {}
