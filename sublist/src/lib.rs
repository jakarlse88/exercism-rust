use std::{collections::HashSet, hash::Hash};


#[derive( Debug , PartialEq , Eq )]
pub enum Comparison {
    Equal
  , Sublist
  , Superlist
  , Unequal
    }


fn index_of<T : PartialEq>( arr : &[ T ] , e : &Option<&T> ) -> Option<usize> {
    e.and_then( | x | arr.iter().position( | y | y == x ))
}


fn last_index_of<T : PartialEq>( arr : &[ T ] , e : &Option<&T> ) -> Option<usize> {
    e.and_then( | x | arr.iter().rposition( | y | y == x ) )
}


fn get_indices<T : PartialEq>( arr_b : &[ T ] , arr_a : &[ T ] ) -> ( Option<usize> , Option<usize> ) {
    let first  =      index_of( &arr_b , &arr_a.first() );
    let second = last_index_of( &arr_b , &arr_a.last()  );

    ( first , second )
} 


fn first_list_greater<T>( arr_a: &[ T ] , arr_b : &[ T ] ) -> bool {
    let len_a = arr_a.len();
    let len_b = arr_b.len();

    len_a > len_b
}


pub fn sublist<T : PartialEq + Eq + Hash>( arr_a: &[ T ] , arr_b : &[ T ] ) -> Comparison {
    if arr_a == arr_b {
        return Comparison::Equal
    }

    if arr_a.is_empty() {
        return Comparison::Sublist
    }

    if arr_b.is_empty() {
        return Comparison::Superlist
    }

    let ( idx_first , idx_second ) = 
        if   first_list_greater( &arr_b , &arr_a ) { get_indices( &arr_b , &arr_a ) }
        else                                       { get_indices( &arr_a , &arr_b ) };

    if idx_first.is_none() || idx_second.is_none() {
        return Comparison::Unequal
    }
    
    let first = idx_first.unwrap(); 
    let scnd  = idx_second.unwrap();

    let tmp = &arr_b[ first .. scnd + 1 ];

    if arr_a == tmp {
        return Comparison::Sublist
    }

    Comparison::Unequal
}
