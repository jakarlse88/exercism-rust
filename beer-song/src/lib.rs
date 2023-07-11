pub fn verse( n : u32 ) -> String {
    if n == 0 as u32 {
        return "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string();
    }
    else if n == 1 as u32 {
        return "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".to_string();
    }
    else if n == 2 as u32 {
        return format!( "{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottle of beer on the wall.\n" , n , n , n - 1 );
    }
    else {
        return format!( "{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n" , n , n , n - 1 );
    }
}


pub fn sing( start : u32 , end : u32 ) -> String {
    let mut i      = start;
    let mut result : String = "".to_owned();

    while i >= end {
        result.push_str( &verse( i ));

        if i != end {
            result.push_str( "\n" );
        }

        if i == 0 {
            break;
        }

        i -= 1;
    }

    result
}
