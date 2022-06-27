fn main() -> std::io::Result<()> {
    println!("Please enter some text:");
    let mut text_input = String::new();
    std::io::stdin().read_line(&mut text_input)?;
    // Convert text to big text
    let mut text_big = String::new();
    text_big = text_sm_bg(text_input, 10);
    println!("Here is your text as big text:");
    println!("{}", text_big);
    Ok(())
}

fn text_sm_bg(text_to_convert: String, line_size: i128) -> String {
    let mut text_converted = String::new();
    let mut text_converted_line1 = String::new();
    let mut text_converted_line2 = String::new();
    let mut text_converted_line3 = String::new();
    let mut text_converted_line4 = String::new();
    let mut text_converted_line5 = String::new();

    // Loop through text letters here
    let mut char_counter = 1;
    let mut text_line: i128 = 1;
    let mut text_converted_return = "\n";
    for text_character in text_to_convert.chars() {
        if char_counter == line_size {
            text_converted_line1 = format!("{}{}", text_converted_line1, text_converted_return);
            text_converted_line2 = format!("{}{}", text_converted_line2, text_converted_return);
            text_converted_line3 = format!("{}{}", text_converted_line3, text_converted_return);
            text_converted_line4 = format!("{}{}", text_converted_line4, text_converted_return);
            text_converted_line5 = format!("{}{}", text_converted_line5, text_converted_return);
            char_counter = 1;
        } else {
            char_counter = char_counter + 1;
            match text_character {
                // Replace individual characters with a large version
                'a' | 'A' => {
                    text_converted_line1 = format!("{} {}", text_converted_line1, "    A    ");
                    text_converted_line2 = format!("{} {}", text_converted_line2, "   A A   ");
                    text_converted_line3 = format!("{} {}", text_converted_line3, "  A   A  ");
                    text_converted_line4 = format!("{} {}", text_converted_line4, " AAAAAAA ");
                    text_converted_line5 = format!("{} {}", text_converted_line5, "A       A");
                        }
                'b' | 'B' => {
                    text_converted_line1 = format!("{} {}", text_converted_line1, "BBBBBBBB ");
                    text_converted_line2 = format!("{} {}", text_converted_line2, "B       B");
                    text_converted_line3 = format!("{} {}", text_converted_line3, "BBBBBBBB ");
                    text_converted_line4 = format!("{} {}", text_converted_line4, "B       B");
                    text_converted_line5 = format!("{} {}", text_converted_line5, "BBBBBBBB ");
                        }
                'c' | 'C' => {
                    text_converted_line1 = format!("{} {}", text_converted_line1, " CCCCCCCC");
                    text_converted_line2 = format!("{} {}", text_converted_line2, "C        ");
                    text_converted_line3 = format!("{} {}", text_converted_line3, "C        ");
                    text_converted_line4 = format!("{} {}", text_converted_line4, "C        ");
                    text_converted_line5 = format!("{} {}", text_converted_line5, " CCCCCCCC");
                        }
                'd' | 'D' => {
                    text_converted_line1 = format!("{} {}", text_converted_line1, "DDDDDDDD ");
                    text_converted_line2 = format!("{} {}", text_converted_line2, "D       D");
                    text_converted_line3 = format!("{} {}", text_converted_line3, "D       D");
                    text_converted_line4 = format!("{} {}", text_converted_line4, "D       D");
                    text_converted_line5 = format!("{} {}", text_converted_line5, "DDDDDDDD ");
                        }
                'e' | 'E' => {
                    text_converted_line1 = format!("{} {}", text_converted_line1, "EEEEEEEEE");
                    text_converted_line2 = format!("{} {}", text_converted_line2, "E        ");
                    text_converted_line3 = format!("{} {}", text_converted_line3, "EEEEEEEE ");
                    text_converted_line4 = format!("{} {}", text_converted_line4, "E        ");
                    text_converted_line5 = format!("{} {}", text_converted_line5, "EEEEEEEEE");
                        }
                'f' | 'F' => {
                    text_converted_line1 = format!("{} {}", text_converted_line1, "FFFFFFFFF");
                    text_converted_line2 = format!("{} {}", text_converted_line2, "F        ");
                    text_converted_line3 = format!("{} {}", text_converted_line3, "FFFFFFFF ");
                    text_converted_line4 = format!("{} {}", text_converted_line4, "F        ");
                    text_converted_line5 = format!("{} {}", text_converted_line5, "F        ");
                        }
                'g' | 'G' => {
                    text_converted_line1 = format!("{} {}", text_converted_line1, " GGGGGGG ");
                    text_converted_line2 = format!("{} {}", text_converted_line2, "G        ");
                    text_converted_line3 = format!("{} {}", text_converted_line3, "G     GGG");
                    text_converted_line4 = format!("{} {}", text_converted_line4, "G       G");
                    text_converted_line5 = format!("{} {}", text_converted_line5, " GGGGGGG ");
                        }
                'h' | 'H' => {
                    text_converted_line1 = format!("{} {}", text_converted_line1, "H       H");
                    text_converted_line2 = format!("{} {}", text_converted_line2, "H       H");
                    text_converted_line3 = format!("{} {}", text_converted_line3, "HHHHHHHHH");
                    text_converted_line4 = format!("{} {}", text_converted_line4, "H       H");
                    text_converted_line5 = format!("{} {}", text_converted_line5, "H       H");
                        }
                'i' | 'I' => {
                    text_converted_line1 = format!("{} {}", text_converted_line1, "IIIIIIIII");
                    text_converted_line2 = format!("{} {}", text_converted_line2, "    I    ");
                    text_converted_line3 = format!("{} {}", text_converted_line3, "    I    ");
                    text_converted_line4 = format!("{} {}", text_converted_line4, "    I    ");
                    text_converted_line5 = format!("{} {}", text_converted_line5, "IIIIIIIII");
                        }
                'j' | 'J' => {
                    text_converted_line1 = format!("{} {}", text_converted_line1, "        J");
                    text_converted_line2 = format!("{} {}", text_converted_line2, "        J");
                    text_converted_line3 = format!("{} {}", text_converted_line3, "        J");
                    text_converted_line4 = format!("{} {}", text_converted_line4, " J      J");
                    text_converted_line5 = format!("{} {}", text_converted_line5, " JJJJJJJJ");
                        }
                'k' | 'K' => {
                    text_converted_line1 = format!("{} {}", text_converted_line1, "K       K");
                    text_converted_line2 = format!("{} {}", text_converted_line2, "K      K ");
                    text_converted_line3 = format!("{} {}", text_converted_line3, "KKKKKKK  ");
                    text_converted_line4 = format!("{} {}", text_converted_line4, "K      K ");
                    text_converted_line5 = format!("{} {}", text_converted_line5, "K       K");
                        }
                'l' | 'L' => {
                    text_converted_line1 = format!("{} {}", text_converted_line1, "L        ");
                    text_converted_line2 = format!("{} {}", text_converted_line2, "L        ");
                    text_converted_line3 = format!("{} {}", text_converted_line3, "L        ");
                    text_converted_line4 = format!("{} {}", text_converted_line4, "L        ");
                    text_converted_line5 = format!("{} {}", text_converted_line5, "LLLLLLLLL");
                        }
                'm' | 'M' => {
                    text_converted_line1 = format!("{} {}", text_converted_line1, "MMMM MMMM");
                    text_converted_line2 = format!("{} {}", text_converted_line2, "M   M   M");
                    text_converted_line3 = format!("{} {}", text_converted_line3, "M   M   M");
                    text_converted_line4 = format!("{} {}", text_converted_line4, "M   M   M");
                    text_converted_line5 = format!("{} {}", text_converted_line5, "M   M   M");
                        }
                'n' | 'N' => {
                    text_converted_line1 = format!("{} {}", text_converted_line1, "NNN     N");
                    text_converted_line2 = format!("{} {}", text_converted_line2, "N  N    N");
                    text_converted_line3 = format!("{} {}", text_converted_line3, "N   N   N");
                    text_converted_line4 = format!("{} {}", text_converted_line4, "N    N  N");
                    text_converted_line5 = format!("{} {}", text_converted_line5, "N     NNN");
                        }
                'o' | 'O' => {
                    text_converted_line1 = format!("{} {}", text_converted_line1, " OOOOOOO ");
                    text_converted_line2 = format!("{} {}", text_converted_line2, "O       O");
                    text_converted_line3 = format!("{} {}", text_converted_line3, "O       O");
                    text_converted_line4 = format!("{} {}", text_converted_line4, "O       O");
                    text_converted_line5 = format!("{} {}", text_converted_line5, " OOOOOOO ");
                        }
                'p' | 'P' => {
                    text_converted_line1 = format!("{} {}", text_converted_line1, "PPPPPPPP ");
                    text_converted_line2 = format!("{} {}", text_converted_line2, "P       P");
                    text_converted_line3 = format!("{} {}", text_converted_line3, "PPPPPPPP ");
                    text_converted_line4 = format!("{} {}", text_converted_line4, "P        ");
                    text_converted_line5 = format!("{} {}", text_converted_line5, "P        ");
                        }
                'q' | 'Q' => {
                    text_converted_line1 = format!("{} {}", text_converted_line1, " QQQQQQQ ");
                    text_converted_line2 = format!("{} {}", text_converted_line2, "Q       Q");
                    text_converted_line3 = format!("{} {}", text_converted_line3, "Q       Q");
                    text_converted_line4 = format!("{} {}", text_converted_line4, "Q    Q  Q");
                    text_converted_line5 = format!("{} {}", text_converted_line5, " QQQQQQQ ");
                        }
                'r' | 'R' => {
                    text_converted_line1 = format!("{} {}", text_converted_line1, "RRRRRRRR ");
                    text_converted_line2 = format!("{} {}", text_converted_line2, "R       R");
                    text_converted_line3 = format!("{} {}", text_converted_line3, "RRRRRRRR ");
                    text_converted_line4 = format!("{} {}", text_converted_line4, "R     R  ");
                    text_converted_line5 = format!("{} {}", text_converted_line5, "R      RR");
                        }
                's' | 'S' => {
                    text_converted_line1 = format!("{} {}", text_converted_line1, " SSSSSSS ");
                    text_converted_line2 = format!("{} {}", text_converted_line2, "S        ");
                    text_converted_line3 = format!("{} {}", text_converted_line3, " SSSSSSS ");
                    text_converted_line4 = format!("{} {}", text_converted_line4, "        S");
                    text_converted_line5 = format!("{} {}", text_converted_line5, " SSSSSSS ");
                        }
                't' | 'T' => {
                    text_converted_line1 = format!("{} {}", text_converted_line1, "TTTTTTTTT");
                    text_converted_line2 = format!("{} {}", text_converted_line2, "    T    ");
                    text_converted_line3 = format!("{} {}", text_converted_line3, "    T    ");
                    text_converted_line4 = format!("{} {}", text_converted_line4, "    T    ");
                    text_converted_line5 = format!("{} {}", text_converted_line5, "    T    ");
                        }
                'u' | 'U' => {
                    text_converted_line1 = format!("{} {}", text_converted_line1, "U       U");
                    text_converted_line2 = format!("{} {}", text_converted_line2, "U       U");
                    text_converted_line3 = format!("{} {}", text_converted_line3, "U       U");
                    text_converted_line4 = format!("{} {}", text_converted_line4, "U       U");
                    text_converted_line5 = format!("{} {}", text_converted_line5, " UUUUUUU ");
                        }
                'v' | 'V' => {
                    text_converted_line1 = format!("{} {}", text_converted_line1, "V       V");
                    text_converted_line2 = format!("{} {}", text_converted_line2, " V     V ");
                    text_converted_line3 = format!("{} {}", text_converted_line3, "  V   V  ");
                    text_converted_line4 = format!("{} {}", text_converted_line4, "   V V   ");
                    text_converted_line5 = format!("{} {}", text_converted_line5, "    V    ");
                        }
                'w' | 'W' => {
                    text_converted_line1 = format!("{} {}", text_converted_line1, "W   W   W");
                    text_converted_line2 = format!("{} {}", text_converted_line2, "W   W   W");
                    text_converted_line3 = format!("{} {}", text_converted_line3, " W W W W ");
                    text_converted_line4 = format!("{} {}", text_converted_line4, "  W   W  ");
                    text_converted_line5 = format!("{} {}", text_converted_line5, "  W   W  ");
                        }
                'x' | 'X' => {
                    text_converted_line1 = format!("{} {}", text_converted_line1, "X       X");
                    text_converted_line2 = format!("{} {}", text_converted_line2, "  X   X  ");
                    text_converted_line3 = format!("{} {}", text_converted_line3, "   XXX   ");
                    text_converted_line4 = format!("{} {}", text_converted_line4, "  X   X  ");
                    text_converted_line5 = format!("{} {}", text_converted_line5, "X       X");
                        }
                'y' | 'Y' => {
                    text_converted_line1 = format!("{} {}", text_converted_line1, "Y       Y");
                    text_converted_line2 = format!("{} {}", text_converted_line2, "  Y   Y  ");
                    text_converted_line3 = format!("{} {}", text_converted_line3, "   YYY   ");
                    text_converted_line4 = format!("{} {}", text_converted_line4, "    Y    ");
                    text_converted_line5 = format!("{} {}", text_converted_line5, "    Y    ");
                        }
                'z' | 'Z' => {
                    text_converted_line1 = format!("{} {}", text_converted_line1, "ZZZZZZZZZ");
                    text_converted_line2 = format!("{} {}", text_converted_line2, "      ZZ ");
                    text_converted_line3 = format!("{} {}", text_converted_line3, "    Z    ");
                    text_converted_line4 = format!("{} {}", text_converted_line4, " ZZ      ");
                    text_converted_line5 = format!("{} {}", text_converted_line5, "ZZZZZZZZZ");
                        }
                ' ' => { // Space
                    text_converted_line1 = format!("{} {}", text_converted_line1, "         ");
                    text_converted_line2 = format!("{} {}", text_converted_line2, "         ");
                    text_converted_line3 = format!("{} {}", text_converted_line3, "         ");
                    text_converted_line4 = format!("{} {}", text_converted_line4, "         ");
                    text_converted_line5 = format!("{} {}", text_converted_line5, "         ");
                        }
                '\n' => (), // Do nothing
                _=> {
                    text_converted_line1 = format!("{} {}", text_converted_line1, "    ??   ");
                    text_converted_line2 = format!("{} {}", text_converted_line2, "  ??  ?? ");
                    text_converted_line3 = format!("{} {}", text_converted_line3, "     ?   ");
                    text_converted_line4 = format!("{} {}", text_converted_line4, "         ");
                    text_converted_line5 = format!("{} {}", text_converted_line5, "    ??   ");
                        }
            };
        }
    }

    text_converted = format!("{}\n{}\n{}\n{}\n{}", text_converted_line1, text_converted_line2, text_converted_line3, text_converted_line4, text_converted_line5);

    return text_converted
}