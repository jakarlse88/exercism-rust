// Seconds per earth year
const EARTH_SEC_YEAR : f64 = 31557600.0;


//
//      TRAITS
//

trait OrbitalPeriod {
    const ORBITAL_PERIOD : f64;
}


pub trait Planet {
    fn years_during( d : &Duration ) -> f64;
}


//
//      STRUCTS
//

#[derive( Debug )]
pub struct Duration {
    seconds : f64
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;


//
//      IMPLEMENTATIONS
//

impl From<u64> for Duration {
    fn from( s : u64 ) -> Self {
        Duration {
            seconds : s as f64
        }
    }
}


macro_rules! impl_planet {
    ( $structName : ident ) => {
        impl Planet for $structName {
            fn years_during( d : &Duration ) -> f64 {
                ( d.seconds / Self::ORBITAL_PERIOD ) / EARTH_SEC_YEAR
            }
        }        
    };
}


impl OrbitalPeriod for Mercury {
    const ORBITAL_PERIOD : f64 = 0.2408467;
}


impl OrbitalPeriod for Venus {
    const ORBITAL_PERIOD : f64 = 0.61519726;
}


impl OrbitalPeriod for Earth {
    const ORBITAL_PERIOD : f64 = 1.0;
}


impl OrbitalPeriod for Mars {
    const ORBITAL_PERIOD : f64 = 1.8808158;
}


impl OrbitalPeriod for Jupiter {
    const ORBITAL_PERIOD : f64 = 11.862615;
}


impl OrbitalPeriod for Saturn {
    const ORBITAL_PERIOD : f64 = 29.447498;
}


impl OrbitalPeriod for Uranus {
    const ORBITAL_PERIOD : f64 = 84.016846;
}


impl OrbitalPeriod for Neptune {
    const ORBITAL_PERIOD : f64 = 164.79132;
}


impl_planet!( Mercury );
impl_planet!( Venus );
impl_planet!( Earth );
impl_planet!( Mars );
impl_planet!( Jupiter );
impl_planet!( Saturn );
impl_planet!( Uranus );
impl_planet!( Neptune );
