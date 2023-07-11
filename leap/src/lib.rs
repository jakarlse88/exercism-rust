
pub fn is_leap_year( year: u64 ) -> bool {
    let div4   = &year % 4   == 0;
    let div100 = &year % 100 == 0;
    let div400 = &year % 400 == 0;

    match div4 && !div100
       || div4 &&  div100 && div400 {
       true  => true
     , false => false
    } 
}
