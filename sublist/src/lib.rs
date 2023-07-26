#[derive( Debug , PartialEq , Eq )]
pub enum Comparison {
    Equal
  , Sublist
  , Superlist
  , Unequal
    }


fn should_eval_sublist<T>( arr_a: &[ T ] , arr_b : &[ T ] ) -> bool {
    ( arr_a.len() as i8 - arr_b.len() as i8 ) < 0
}


fn occurs_in<T : Eq>( arr_a: &[ T ] , arr_b : &[ T ] ) -> bool {
    let mut i : usize = 0;
    let mut k : usize = arr_a.len();

    loop {
        if k > arr_b.len() {
            break;
        }

        if arr_a == &arr_b[ i .. k ] {
            return true
        }
        
        i += 1;
        k += 1;
    }

    false
}


fn eval_sublist<T : Eq>( arr_a: &[ T ] , arr_b : &[ T ] ) -> Comparison {
    let is_sublist = arr_a.is_empty() 
                  || occurs_in( arr_a , arr_b );

    if   is_sublist { Comparison::Sublist } 
    else            { Comparison::Unequal }
}


fn eval_superlist<T : Eq>( arr_a: &[ T ] , arr_b : &[ T ] ) -> Comparison {
    let is_superlist = arr_b.is_empty() 
                    || occurs_in( arr_b , arr_a );

    if   is_superlist { Comparison::Superlist }   
    else              { Comparison::Unequal   }
}


pub fn sublist<T : PartialEq + Eq>( arr_a: &[ T ] , arr_b : &[ T ] ) -> Comparison {
    if      arr_a == arr_b                       { Comparison::Equal }
    else if should_eval_sublist( arr_a , arr_b ) { eval_sublist(   arr_a,  arr_b ) }
    else                                         { eval_superlist( arr_a , arr_b ) }
}
