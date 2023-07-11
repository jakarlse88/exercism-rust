use std::fmt;


fn fmt_temporal( i : i32 ) -> String {
    let mut result = String::new();

    if i < 10 {
        result += "0";
    }

    result += &i.to_string();

    result
}


fn min_to_hr_min( m : i32 ) -> ( i32 , i32 ) {
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

    if   h < 0 { 24 + hh }
    else       { hh }
}


#[derive( Eq , PartialEq , Debug )]
pub struct Clock {
    pub hour   : i32
  , pub minute : i32 
    }


impl Clock {
    pub fn new( hours : i32 , minutes : i32 ) -> Self {
        let ( h , m ) = min_to_hr_min( minutes );

        Clock {
            hour   : calc_hour( hours + h )
          , minute : m 
            }
    }

    pub fn add_minutes( &self , minutes: i32 ) -> Self {
        unimplemented!( "Add {minutes} minutes to existing Clock time" );
    }
}


impl fmt::Display for Clock {
    fn fmt( &self , f : &mut fmt::Formatter<'_> ) -> fmt::Result {
        write!( f , "{}:{}" , fmt_temporal( self.hour ) , fmt_temporal( self.minute ))
    }
}
