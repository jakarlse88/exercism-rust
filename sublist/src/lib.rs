#[derive( Debug , PartialEq , Eq )]
pub enum Comparison {
    Equal
  , Sublist
  , Superlist
  , Unequal
    }


enum LengthComparison {
    Equal   = 0
  , ALonger = -1
  , BLonger = 1
}


fn compare_length<T>( arr_a: &[ T ] , arr_b : &[ T ] ) -> LengthComparison {
    let diff = arr_a.len() as i32 - arr_b.len() as i32;

    return 
        if diff == 0 {
            LengthComparison::Equal
        }
        else if diff > 0 {
            LengthComparison::ALonger
        }
        else {
            LengthComparison::BLonger
        }
}


fn compare<T : Eq>( arr_a: &[ T ] , arr_b : &[ T ] ) -> bool {
    let mut i : usize = 0;
    let mut k : usize = arr_a.len();

    loop {
        if k > arr_b.len() {
            break;
        }

        let _temp = &arr_b[ i .. k ];

        if arr_a == &arr_b[ i .. k ] {
            return true
        }
        
        i += 1;
        k += 1;
    }

    false
}


fn compare_equal<T : Eq>( arr_a: &[ T ] , arr_b : &[ T ] ) -> Comparison {
    return 
        if arr_a == arr_b {
            Comparison::Equal
        }
        else {
            Comparison::Unequal
        }
}

pub fn sublist<T : PartialEq + Eq>( arr_a: &[ T ] , arr_b : &[ T ] ) -> Comparison {
    if arr_a == arr_b {
        return Comparison::Equal
    }

    if arr_a.is_empty() {
        return Comparison::Sublist
    }

    if arr_b.is_empty() {
        return Comparison::Superlist
    }

    return match compare_length( arr_a , arr_b ) {
        LengthComparison::Equal   => compare_equal( arr_a , arr_b )
      , LengthComparison::ALonger => if compare( arr_b , arr_a ) { Comparison::Superlist } else { Comparison::Unequal }
      , LengthComparison::BLonger => if compare( arr_a , arr_b ) { Comparison::Sublist }   else { Comparison::Unequal }
    }
}
