// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration{
    in_earth_years: f64
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        const ONE_EARTH_YRS_SEC: u64 = 31_557_600;
        Self { in_earth_years: s as f64 / ONE_EARTH_YRS_SEC as f64 }
    }
}

pub trait Planet {
    const FACTOR: f64;
    fn years_during(d: &Duration) -> f64{
        d.in_earth_years / Self::FACTOR
    }
}

pub mod orbital_factors{
    pub const MERCURY: f64 = 0.2408467;
    pub const VENUS: f64 = 0.61519726;
    pub const EARTH: f64 = 1.0;
    pub const MARS: f64 = 1.8808158;
    pub const JUPITER: f64 = 11.862615;
    pub const SATURN: f64 = 29.447498;
    pub const URANUS: f64 = 84.016846;
    pub const NEPTUNE: f64 = 164.79132;
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury{
    const FACTOR: f64 = orbital_factors::MERCURY;
}

impl Planet for Venus {
    const FACTOR: f64 = orbital_factors::VENUS;
}

impl Planet for Earth {
    const FACTOR: f64 = orbital_factors::EARTH;
}

impl Planet for Mars {
    const FACTOR: f64 = orbital_factors::MARS;
}

impl Planet for Jupiter {
    const FACTOR: f64 = orbital_factors::JUPITER;
}

impl Planet for Saturn {
    const FACTOR: f64 = orbital_factors::SATURN;
}

impl Planet for Uranus {
    const FACTOR: f64 = orbital_factors::URANUS;
}

impl Planet for Neptune {
    const FACTOR: f64 = orbital_factors::NEPTUNE;
}
