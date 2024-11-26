#![no_std]

pub use static_trignometry_macros::{
    f32_sine_values, f64_sine_values, StaticTrigF32, StaticTrigF64,
};

use num_traits::{Float, FloatConst, NumCast, One, ToPrimitive};

pub trait StaticTrignometry<const N: usize> {
    type FloatType: Float + FloatConst;

    const QUARTER_SINE: [Self::FloatType; N];

    #[inline]
    fn sampled_sin(index: usize) -> Option<Self::FloatType> {
        if index < N {
            Some(Self::QUARTER_SINE[index])
        } else {
            None
        }
    }

    #[inline]
    fn sampled_sin_inclusive(index: usize) -> Option<Self::FloatType> {
        if index == N {
            Some(Self::FloatType::one())
        } else {
            Self::sampled_sin(index)
        }
    }

    fn sin(radians: Self::FloatType) -> Self::FloatType {
        let negative = Float::is_sign_negative(radians);
        let radians = Float::abs(radians) % (Self::FloatType::PI() + Self::FloatType::PI());
        let (negative, radians) = {
            if radians >= Self::FloatType::PI() {
                (!negative, radians - Self::FloatType::PI())
            } else if radians > Self::FloatType::FRAC_PI_2() {
                (negative, Self::FloatType::FRAC_PI_2() - radians)
            } else {
                (negative, radians)
            }
        };
        let index = (radians * (Self::FloatType::FRAC_2_PI() * NumCast::from(N).unwrap()))
            .round()
            .to_usize()
            .unwrap();
        let value = Self::sampled_sin_inclusive(index).unwrap();
        match negative {
            false => value,
            true => -value,
        }
    }

    #[inline]
    fn cos(radians: Self::FloatType) -> Self::FloatType {
        Self::sin(radians + Self::FloatType::FRAC_PI_2())
    }

    #[inline]
    fn tan(radians: Self::FloatType) -> Self::FloatType {
        Self::sin(radians) / Self::cos(radians)
    }
}

pub trait StaticTrigF32<const N: usize>: StaticTrignometry<N, FloatType = f32> {}
impl<T, const N: usize> StaticTrigF32<N> for T where T: StaticTrignometry<N, FloatType = f32> {}

pub trait StaticTrigF64<const N: usize>: StaticTrignometry<N, FloatType = f64> {}
impl<T, const N: usize> StaticTrigF64<N> for T where T: StaticTrignometry<N, FloatType = f64> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(StaticTrigF32)]
    #[trig(samples = 1024)]
    struct TrigF32Engine;

    #[derive(StaticTrigF64)]
    #[trig(samples = 1024)]
    struct TrigF64Engine;

    #[test]
    fn sin_f32_quadrants() {
        assert_eq!(TrigF32Engine::sin(f32::FRAC_PI_2()), 1.);
        assert_eq!(TrigF32Engine::sin(f32::PI()), 0.);
        assert_eq!(TrigF32Engine::sin(f32::PI() + f32::FRAC_PI_2()), -1.);
        assert_eq!(TrigF32Engine::sin(f32::PI() + f32::PI()), 0.);

        assert_eq!(TrigF32Engine::sin(-f32::FRAC_PI_2()), -1.);
        assert_eq!(TrigF32Engine::sin(-(f32::PI() + f32::FRAC_PI_2())), 1.);
    }

    #[test]
    fn sin_f64_quadrants() {
        assert_eq!(TrigF64Engine::sin(f64::FRAC_PI_2()), 1.);
        assert_eq!(TrigF64Engine::sin(f64::PI()), 0.);
        assert_eq!(TrigF64Engine::sin(f64::PI() + f64::FRAC_PI_2()), -1.);
        assert_eq!(TrigF64Engine::sin(f64::PI() + f64::PI()), 0.);

        assert_eq!(TrigF64Engine::sin(-f64::FRAC_PI_2()), -1.);
        assert_eq!(TrigF64Engine::sin(-(f64::PI() + f64::FRAC_PI_2())), 1.);
    }
}
