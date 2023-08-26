const  BOMB_CH : u8 = b'*';
const SPACE_CH : u8 = b' ';


pub fn annotate( minefield : &[ &str ] ) -> Vec<String> {
    let mut result : Vec<String> = Vec::new();

    for ( row_idx , &row ) in minefield.iter().enumerate() {
        let mut row_result = String::new();

        for ( ch_idx , ch ) in row.as_bytes().iter().enumerate() {
            if *ch != BOMB_CH {
                let mut counter = 0;

                for x in row_idx.saturating_sub( 1 )..usize::min( row_idx + 2 , minefield.len() ) {
                    for y in ch_idx.saturating_sub( 1 )..usize::min( ch_idx + 2 , row.len() ) {
                        if minefield[ x ].as_bytes()[ y ] == BOMB_CH {
                            counter += 1;
                        }
                    }
                }

                row_result.push( 
                    if   counter == 0 { SPACE_CH } 
                    else              { counter as u8 + b'0' } as char 
                 );
            }    
            else {
                row_result.push( *ch as char ); 
            }
        }

        result.push( row_result );
    }
    
    result
}
