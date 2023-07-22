fn to_u32_vec( code : &str ) -> Option<Vec<u32>> {
    code
       .chars()
       .map( | c | c.to_digit( 10 ))
       .into_iter()
       .collect::<Option<Vec<u32>>>()
}


fn double_luhn( i : &u32 ) -> u32 {
    let j = i * 2;

    if   j > 9 { j - 9 }
    else       { j     }
}


fn sum_luhn( vec : &Vec<u32> ) -> u32 {
    let mut sum : u32 = 0;

    for ( i , val ) in vec.iter().rev().enumerate() {
        sum =
            if   !( i % 2 == 0 ) { sum + double_luhn( val ) }
            else                 { sum + val };
    }
    
    sum
}


fn validate_checksum( checksum : &u32 ) -> bool {
    checksum % 10 == 0
}


pub fn is_valid( code: &str ) -> bool {
    let sanitised = &code.replace( ' ' , "" );

    if sanitised.len() < 2 {
        return false
    }

    match &to_u32_vec( &sanitised ) {
        Some( vec ) => validate_checksum( &sum_luhn( &vec ) )
      , None        => false  
    }
}
