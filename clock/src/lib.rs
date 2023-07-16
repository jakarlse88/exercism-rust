use std::fmt;


fn total_hours( hours : i32 , minutes : i32 ) -> i32 {
    let min_offset = ( minutes - minutes.rem_euclid( 60 )) / 60;
    let hours      = ( hours + min_offset ) % 24;

    if   hours >= 0 { hours }
    else            { 24 + hours }
}


fn total_minutes( minutes : i32 ) -> i32 {
    minutes.rem_euclid( 60 )
}


#[derive( PartialEq , Debug )]
pub struct Clock {
    hours   : i32
  , minutes : i32 
    }


impl Clock {
    pub fn new( hours : i32 , minutes : i32 ) -> Self {
        Clock {
            hours   : total_hours( hours, minutes )
          , minutes : total_minutes( minutes )
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
