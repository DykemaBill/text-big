fn main() -> std::io::Result<()> {
    println!("This program takes your input and prints it out in big text!");
    println!("Please enter some text:");
    let mut text_input = String::new();
    std::io::stdin().read_line(&mut text_input)?;
    // Convert text to big text
    let mut text_big = String::new();
    text_big = text_sm_bg(text_input);
    println!("Here is your text as big text:");
    println!("{}", text_big);
    Ok(())
}

fn text_sm_bg(text_to_convert: String) -> String {
    let mut text_converted = String::new();
    text_converted = format!("{} {}", "Just returning your text for now:", text_to_convert);

    // Loop through text letters here
    for text_character in text_to_convert.chars() {
        match text_character {
            // Just a test so far, next will need to figure out how to make this into something useful
            'a' => println!("AAA"),
            'b' => println!("BBB"),
            'c' => println!("CCC"),
            _=> print!("???"),
        };
    }

    return text_converted
}