use std::collections::HashSet;


fn normalise( input_str : &String ) -> String {
    let mut char_vec = input_str
                        .chars()
                        .collect::<Vec<char>>();

    char_vec.sort_unstable();

    char_vec
        .iter()
        .collect::<String>()
}


fn is_not_duplicate( word : &String , candidate : &String ) -> bool {
    word != candidate
}


fn is_match( word : &String , candidate : &String ) -> bool {
    &normalise( word ) == &normalise( candidate )
}


pub fn anagrams_for<'a>( word: &str , possible_anagrams : &[ &'a str ]) -> HashSet<&'a str> {
    let mut match_set : HashSet<&'a str> = HashSet::new();

    if possible_anagrams.len() < 1 {
        return match_set
    }

    let word = &word.to_lowercase();

    for candidate in possible_anagrams {
        if candidate.len() != word.len() {
            continue;
        }

        let cand_lower = &candidate.to_lowercase();

        if is_not_duplicate( word , cand_lower ) && is_match( word , cand_lower ) {
            let _ = match_set.insert( &candidate );
        }
    }

    match_set
}
