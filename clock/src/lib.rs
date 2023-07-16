use std::fmt;


fn hour_offset_min( m : i32 ) -> ( i32 , i32 ) {
    let mut mm = m;
    let mut h  = 0;

    if m < 0 {
        while mm < 0 {
             h -= 1;
            mm += 60;
        }
    }
    else {
        while mm - 60 >= 0 {
             h += 1;
            mm -= 60;
        }
    }

    ( h , mm )
}


fn calc_hour( h : i32 ) -> i32 {
    let hh = h % 24;

    if   h < 0 { ( 24 + hh ) % 24 }
    else       { hh }
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
            hours   : calc_hour( hours + h )
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
