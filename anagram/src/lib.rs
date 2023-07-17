use std::collections::HashSet;
use std::collections::hash_map::DefaultHasher;
use std::hash::{ Hash , Hasher };


fn calculate_hash<T : Hash>( t : &T ) -> u64 {
    let mut hasher = DefaultHasher::new();

    t.hash( &mut hasher );

    hasher.finish()
}


fn normalise( input_str : &String ) -> String {
    let mut char_vec = input_str
                        .chars()
                        .collect::<Vec<char>>();

    char_vec.sort_unstable();

    char_vec
        .iter()
        .collect::<String>()
}


fn normalise_and_calculate_hash( input_str : &String ) -> u64 {
    calculate_hash( &normalise( input_str ))
}


pub fn anagrams_for<'a>( word: &'a str , possible_anagrams : &[ &'a str ]) -> HashSet<&'a str> {
    let mut match_set : HashSet<&'a str> = HashSet::new();

    if possible_anagrams.len() < 1 {
        return match_set
    }

    let word = &word.to_lowercase();

    for candidate in possible_anagrams {
        let cand_lower = &candidate.to_lowercase();

        if cand_lower.eq( word ) {
            continue;
        }
        
        let hashed_word      = &normalise_and_calculate_hash( word );
        let hashed_candidate = &normalise_and_calculate_hash( cand_lower ); 

        if hashed_word == hashed_candidate {
            let _ = match_set.insert( &candidate );
        }
    }

    match_set
}
