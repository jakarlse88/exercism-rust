use unicode_segmentation::UnicodeSegmentation;


pub fn reverse( input: &str ) -> String {
    let mut output = String::new();

    if input.is_empty() {
        return output
    }

    let mut g : Vec<&str> = UnicodeSegmentation::graphemes( input, true ).collect();

    loop {
        match g.pop() {
            Some( c ) => { output += c }
          , None      => { break;      }
            }
    }

    output
}
