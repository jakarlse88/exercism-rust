use std::fmt;


fn hour_offset_min( m : i32 ) -> ( i32 , i32 ) {
    let mut m = m;
    let mut h = 0;

    if m < 0 {
        while m < 0 {
            h -= 1;
            m += 60;
        }
    }
    else {
        while m - 60 >= 0 {
            h += 1;
            m -= 60;
        }
    }

    ( h , m )
}


#[derive( PartialEq , Debug )]
pub struct Clock {
    hours   : i32
  , minutes : i32 
    }


impl Clock {
    pub fn new( hours : i32 , minutes : i32 ) -> Self {
        let ( h , m ) = hour_offset_min( minutes );

        Clock {
            hours   : ( hours + h ).rem_euclid( 24 )
          , minutes : m 
            }
    }

    pub fn add_minutes( &self , minutes: i32 ) -> Self {
       Clock::new( self.hours , self.minutes + minutes ) 
    }
}


impl fmt::Display for Clock {
    fn fmt( &self , f : &mut fmt::Formatter<'_> ) -> fmt::Result {
        write!( f , "{:>02}:{:>02}" , self.hours , self.minutes )
    }
}
