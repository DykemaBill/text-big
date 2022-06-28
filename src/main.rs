use std::env; // Used for arguments

fn main() -> std::io::Result<()> {

    // Formatting (see https://docs.rs/embedded-text/0.4.0/embedded_text/style/index.html)
    // Text colors
    let text_black = "\x1b[30m";
    let text_red = "\x1b[31m";
    let text_green = "\x1b[32m";
    let text_yellow = "\x1b[33m";
    let text_blue = "\x1b[34m";
    let text_magenta = "\x1b[35m";
    let text_cyan = "\x1b[36m";
    let text_white = "\x1b[37m";
    let text_gray = "\x1b[90m";
    let text_salmon = "\x1b[91m";
    let text_fluorgreen = " \x1b[92m";
    let text_pastelyellow = "\x1b[93m";
    let text_babyblue = "\x1b[94m";
    let text_pink = "\x1b[95m";
    let text_cyan = "\x1b[96m";
    let text_white = "\x1b[97m";
    let text_colorclear = "\x1b[39m";
    // Background colors
    let text_backblack = "\x1b[40m";
    let text_backred = "\x1b[41m";
    let text_backgreen = "\x1b[42m";
    let text_backyellow = "\x1b[43m";
    let text_backblue = "\x1b[44m";
    let text_backmagenta = "\x1b[45m";
    let text_backcyan = "\x1b[46m";
    let text_backwhite = "\x1b[47m";
    let text_backgray = "\x1b[100m";
    let text_backsalmon = "\x1b[101m";
    let text_backfluorgreen = " \x1b[102m";
    let text_backpastelyellow = "\x1b[103m";
    let text_backbabyblue = "\x1b[104m";
    let text_backpink = "\x1b[105m";
    let text_backcyan = "\x1b[106m";
    let text_backwhite = "\x1b[107m";
    let text_backclear = "\x1b[49m";
    // Formatting
    let text_underline = "\x1b[4m";
    let text_underlineclear = "\x1b[24m";
    let text_strike = "\x1b[9m";
    let text_strickclear = "\x1b[29m";
    let text_allclear = "\x1b[0m";

    // Command line arguments
    let args: Vec<String> = env::args().collect();
    // Grab command run
    let command_run: String = args[0].parse::<String>().unwrap();
    // Check for the first passed argument
    let line_length = args[1].parse::<u16>().unwrap();
    // let mut line_length: u16 = 0;
    // if args[1].is_empty() {
    //     line_length = 0;
    // } else {
    //     // Grab first argument as Int 16
    //     line_length = args[1].parse::<u16>().unwrap();
    // }
    // Grab the rest of the arguments as String
    let text_input_all: Vec<String> = std::env::args().collect();
    let text_input: String = text_input_all[2..].join(" ");

    // Check arguments
    if (text_input.is_empty()) || (line_length < 1) { // TODO: NEED TO HANDLE NO LENGTH PASSED, THIS IS NOT WORKING
        println!("{}Syntax:{}", text_yellow, text_colorclear);
        println!("        {}{} {}{}[chars per line >1]{} {}{}['text to print']", text_gray, command_run, text_backgray, text_cyan, text_allclear, text_backgray, text_fluorgreen);
    } else {
        // Convert text to big text
        let text_big: Vec<String> = text_sm_bg(text_input, line_length);
        for text_line in &text_big {
            println!("{}", text_line);
        }
    }
    Ok(())

}

fn text_sm_bg(text_to_convert: String, line_size: u16) -> Vec<String> {

    // Strings for assembling big text
    let mut text_converted: Vec<String> = vec![];
    let mut text_converted_line1 = String::new();
    let mut text_converted_line2 = String::new();
    let mut text_converted_line3 = String::new();
    let mut text_converted_line4 = String::new();
    let mut text_converted_line5 = String::new();

    // Spacing
    let text_bound: String = "".to_string();
    let text_space: String = " ".to_string();

    // Loop through text letters here
    let mut char_counter: u16 = 1;
    let mut position_counter: usize = 1;
    let text_length = text_to_convert.chars().count();
    for text_character in text_to_convert.chars() {
        // Add a big character
        match text_character {
            // Replace individual characters with a large version
            'a' | 'A' => {
                text_converted_line1.push_str(&format!("{}    A    {}{}", text_bound, text_bound, text_space));
                text_converted_line2.push_str(&format!("{}   A A   {}{}", text_bound, text_bound, text_space));
                text_converted_line3.push_str(&format!("{}  A   A  {}{}", text_bound, text_bound, text_space));
                text_converted_line4.push_str(&format!("{} AAAAAAA {}{}", text_bound, text_bound, text_space));
                text_converted_line5.push_str(&format!("{}A       A{}{}", text_bound, text_bound, text_space));
                    }
            'b' | 'B' => {
                text_converted_line1.push_str(&format!("{}BBBBBBBB {}{}", text_bound, text_bound, text_space));
                text_converted_line2.push_str(&format!("{}B       B{}{}", text_bound, text_bound, text_space));
                text_converted_line3.push_str(&format!("{}BBBBBBBB {}{}", text_bound, text_bound, text_space));
                text_converted_line4.push_str(&format!("{}B       B{}{}", text_bound, text_bound, text_space));
                text_converted_line5.push_str(&format!("{}BBBBBBBB {}{}", text_bound, text_bound, text_space));
                    }
            'c' | 'C' => {
                text_converted_line1.push_str(&format!("{} CCCCCCCC{}{}", text_bound, text_bound, text_space));
                text_converted_line2.push_str(&format!("{}C        {}{}", text_bound, text_bound, text_space));
                text_converted_line3.push_str(&format!("{}C        {}{}", text_bound, text_bound, text_space));
                text_converted_line4.push_str(&format!("{}C        {}{}", text_bound, text_bound, text_space));
                text_converted_line5.push_str(&format!("{} CCCCCCCC{}{}", text_bound, text_bound, text_space));
                    }
            'd' | 'D' => {
                text_converted_line1.push_str(&format!("{}DDDDDDDD {}{}", text_bound, text_bound, text_space));
                text_converted_line2.push_str(&format!("{}D       D{}{}", text_bound, text_bound, text_space));
                text_converted_line3.push_str(&format!("{}D       D{}{}", text_bound, text_bound, text_space));
                text_converted_line4.push_str(&format!("{}D       D{}{}", text_bound, text_bound, text_space));
                text_converted_line5.push_str(&format!("{}DDDDDDDD {}{}", text_bound, text_bound, text_space));
                    }
            'e' | 'E' => {
                text_converted_line1.push_str(&format!("{}EEEEEEEEE{}{}", text_bound, text_bound, text_space));
                text_converted_line2.push_str(&format!("{}E        {}{}", text_bound, text_bound, text_space));
                text_converted_line3.push_str(&format!("{}EEEEEEEE {}{}", text_bound, text_bound, text_space));
                text_converted_line4.push_str(&format!("{}E        {}{}", text_bound, text_bound, text_space));
                text_converted_line5.push_str(&format!("{}EEEEEEEEE{}{}", text_bound, text_bound, text_space));
                    }
            'f' | 'F' => {
                text_converted_line1.push_str(&format!("{}FFFFFFFFF{}{}", text_bound, text_bound, text_space));
                text_converted_line2.push_str(&format!("{}F        {}{}", text_bound, text_bound, text_space));
                text_converted_line3.push_str(&format!("{}FFFFFFFF {}{}", text_bound, text_bound, text_space));
                text_converted_line4.push_str(&format!("{}F        {}{}", text_bound, text_bound, text_space));
                text_converted_line5.push_str(&format!("{}F        {}{}", text_bound, text_bound, text_space));
                    }
            'g' | 'G' => {
                text_converted_line1.push_str(&format!("{} GGGGGGG {}{}", text_bound, text_bound, text_space));
                text_converted_line2.push_str(&format!("{}G        {}{}", text_bound, text_bound, text_space));
                text_converted_line3.push_str(&format!("{}G     GGG{}{}", text_bound, text_bound, text_space));
                text_converted_line4.push_str(&format!("{}G       G{}{}", text_bound, text_bound, text_space));
                text_converted_line5.push_str(&format!("{} GGGGGGG {}{}", text_bound, text_bound, text_space));
                    }
            'h' | 'H' => {
                text_converted_line1.push_str(&format!("{}H       H{}{}", text_bound, text_bound, text_space));
                text_converted_line2.push_str(&format!("{}H       H{}{}", text_bound, text_bound, text_space));
                text_converted_line3.push_str(&format!("{}HHHHHHHHH{}{}", text_bound, text_bound, text_space));
                text_converted_line4.push_str(&format!("{}H       H{}{}", text_bound, text_bound, text_space));
                text_converted_line5.push_str(&format!("{}H       H{}{}", text_bound, text_bound, text_space));
                    }
            'i' | 'I' => {
                text_converted_line1.push_str(&format!("{}IIIIIIIII{}{}", text_bound, text_bound, text_space));
                text_converted_line2.push_str(&format!("{}    I    {}{}", text_bound, text_bound, text_space));
                text_converted_line3.push_str(&format!("{}    I    {}{}", text_bound, text_bound, text_space));
                text_converted_line4.push_str(&format!("{}    I    {}{}", text_bound, text_bound, text_space));
                text_converted_line5.push_str(&format!("{}IIIIIIIII{}{}", text_bound, text_bound, text_space));
                    }
            'j' | 'J' => {
                text_converted_line1.push_str(&format!("{}        J{}{}", text_bound, text_bound, text_space));
                text_converted_line2.push_str(&format!("{}        J{}{}", text_bound, text_bound, text_space));
                text_converted_line3.push_str(&format!("{}        J{}{}", text_bound, text_bound, text_space));
                text_converted_line4.push_str(&format!("{} J      J{}{}", text_bound, text_bound, text_space));
                text_converted_line5.push_str(&format!("{} JJJJJJJJ{}{}", text_bound, text_bound, text_space));
                    }
            'k' | 'K' => {
                text_converted_line1.push_str(&format!("{}K       K{}{}", text_bound, text_bound, text_space));
                text_converted_line2.push_str(&format!("{}K      K {}{}", text_bound, text_bound, text_space));
                text_converted_line3.push_str(&format!("{}KKKKKKK  {}{}", text_bound, text_bound, text_space));
                text_converted_line4.push_str(&format!("{}K      K {}{}", text_bound, text_bound, text_space));
                text_converted_line5.push_str(&format!("{}K       K{}{}", text_bound, text_bound, text_space));
                    }
            'l' | 'L' => {
                text_converted_line1.push_str(&format!("{}L        {}{}", text_bound, text_bound, text_space));
                text_converted_line2.push_str(&format!("{}L        {}{}", text_bound, text_bound, text_space));
                text_converted_line3.push_str(&format!("{}L        {}{}", text_bound, text_bound, text_space));
                text_converted_line4.push_str(&format!("{}L        {}{}", text_bound, text_bound, text_space));
                text_converted_line5.push_str(&format!("{}LLLLLLLLL{}{}", text_bound, text_bound, text_space));
                    }
            'm' | 'M' => {
                text_converted_line1.push_str(&format!("{}MMMM MMMM{}{}", text_bound, text_bound, text_space));
                text_converted_line2.push_str(&format!("{}M   M   M{}{}", text_bound, text_bound, text_space));
                text_converted_line3.push_str(&format!("{}M   M   M{}{}", text_bound, text_bound, text_space));
                text_converted_line4.push_str(&format!("{}M   M   M{}{}", text_bound, text_bound, text_space));
                text_converted_line5.push_str(&format!("{}M   M   M{}{}", text_bound, text_bound, text_space));
                    }
            'n' | 'N' => {
                text_converted_line1.push_str(&format!("{}NNN     N{}{}", text_bound, text_bound, text_space));
                text_converted_line2.push_str(&format!("{}N  N    N{}{}", text_bound, text_bound, text_space));
                text_converted_line3.push_str(&format!("{}N   N   N{}{}", text_bound, text_bound, text_space));
                text_converted_line4.push_str(&format!("{}N    N  N{}{}", text_bound, text_bound, text_space));
                text_converted_line5.push_str(&format!("{}N     NNN{}{}", text_bound, text_bound, text_space));
                    }
            'o' | 'O' => {
                text_converted_line1.push_str(&format!("{} OOOOOOO {}{}", text_bound, text_bound, text_space));
                text_converted_line2.push_str(&format!("{}O       O{}{}", text_bound, text_bound, text_space));
                text_converted_line3.push_str(&format!("{}O       O{}{}", text_bound, text_bound, text_space));
                text_converted_line4.push_str(&format!("{}O       O{}{}", text_bound, text_bound, text_space));
                text_converted_line5.push_str(&format!("{} OOOOOOO {}{}", text_bound, text_bound, text_space));
                    }
            'p' | 'P' => {
                text_converted_line1.push_str(&format!("{}PPPPPPPP {}{}", text_bound, text_bound, text_space));
                text_converted_line2.push_str(&format!("{}P       P{}{}", text_bound, text_bound, text_space));
                text_converted_line3.push_str(&format!("{}PPPPPPPP {}{}", text_bound, text_bound, text_space));
                text_converted_line4.push_str(&format!("{}P        {}{}", text_bound, text_bound, text_space));
                text_converted_line5.push_str(&format!("{}P        {}{}", text_bound, text_bound, text_space));
                    }
            'q' | 'Q' => {
                text_converted_line1.push_str(&format!("{} QQQQQQQ {}{}", text_bound, text_bound, text_space));
                text_converted_line2.push_str(&format!("{}Q       Q{}{}", text_bound, text_bound, text_space));
                text_converted_line3.push_str(&format!("{}Q       Q{}{}", text_bound, text_bound, text_space));
                text_converted_line4.push_str(&format!("{}Q    Q  Q{}{}", text_bound, text_bound, text_space));
                text_converted_line5.push_str(&format!("{} QQQQQQQ {}{}", text_bound, text_bound, text_space));
                    }
            'r' | 'R' => {
                text_converted_line1.push_str(&format!("{}RRRRRRRR {}{}", text_bound, text_bound, text_space));
                text_converted_line2.push_str(&format!("{}R       R{}{}", text_bound, text_bound, text_space));
                text_converted_line3.push_str(&format!("{}RRRRRRRR {}{}", text_bound, text_bound, text_space));
                text_converted_line4.push_str(&format!("{}R     R  {}{}", text_bound, text_bound, text_space));
                text_converted_line5.push_str(&format!("{}R      RR{}{}", text_bound, text_bound, text_space));
                    }
            's' | 'S' => {
                text_converted_line1.push_str(&format!("{} SSSSSSS {}{}", text_bound, text_bound, text_space));
                text_converted_line2.push_str(&format!("{}S        {}{}", text_bound, text_bound, text_space));
                text_converted_line3.push_str(&format!("{} SSSSSSS {}{}", text_bound, text_bound, text_space));
                text_converted_line4.push_str(&format!("{}        S{}{}", text_bound, text_bound, text_space));
                text_converted_line5.push_str(&format!("{} SSSSSSS {}{}", text_bound, text_bound, text_space));
                    }
            't' | 'T' => {
                text_converted_line1.push_str(&format!("{}TTTTTTTTT{}{}", text_bound, text_bound, text_space));
                text_converted_line2.push_str(&format!("{}    T    {}{}", text_bound, text_bound, text_space));
                text_converted_line3.push_str(&format!("{}    T    {}{}", text_bound, text_bound, text_space));
                text_converted_line4.push_str(&format!("{}    T    {}{}", text_bound, text_bound, text_space));
                text_converted_line5.push_str(&format!("{}    T    {}{}", text_bound, text_bound, text_space));
                    }
            'u' | 'U' => {
                text_converted_line1.push_str(&format!("{}U       U{}{}", text_bound, text_bound, text_space));
                text_converted_line2.push_str(&format!("{}U       U{}{}", text_bound, text_bound, text_space));
                text_converted_line3.push_str(&format!("{}U       U{}{}", text_bound, text_bound, text_space));
                text_converted_line4.push_str(&format!("{}U       U{}{}", text_bound, text_bound, text_space));
                text_converted_line5.push_str(&format!("{} UUUUUUU {}{}", text_bound, text_bound, text_space));
                    }
            'v' | 'V' => {
                text_converted_line1.push_str(&format!("{}V       V{}{}", text_bound, text_bound, text_space));
                text_converted_line2.push_str(&format!("{} V     V {}{}", text_bound, text_bound, text_space));
                text_converted_line3.push_str(&format!("{}  V   V  {}{}", text_bound, text_bound, text_space));
                text_converted_line4.push_str(&format!("{}   V V   {}{}", text_bound, text_bound, text_space));
                text_converted_line5.push_str(&format!("{}    V    {}{}", text_bound, text_bound, text_space));
                    }
            'w' | 'W' => {
                text_converted_line1.push_str(&format!("{}W   W   W{}{}", text_bound, text_bound, text_space));
                text_converted_line2.push_str(&format!("{}W   W   W{}{}", text_bound, text_bound, text_space));
                text_converted_line3.push_str(&format!("{} W W W W {}{}", text_bound, text_bound, text_space));
                text_converted_line4.push_str(&format!("{}  W   W  {}{}", text_bound, text_bound, text_space));
                text_converted_line5.push_str(&format!("{}  W   W  {}{}", text_bound, text_bound, text_space));
                    }
            'x' | 'X' => {
                text_converted_line1.push_str(&format!("{}X       X{}{}", text_bound, text_bound, text_space));
                text_converted_line2.push_str(&format!("{}  X   X  {}{}", text_bound, text_bound, text_space));
                text_converted_line3.push_str(&format!("{}   XXX   {}{}", text_bound, text_bound, text_space));
                text_converted_line4.push_str(&format!("{}  X   X  {}{}", text_bound, text_bound, text_space));
                text_converted_line5.push_str(&format!("{}X       X{}{}", text_bound, text_bound, text_space));
                    }
            'y' | 'Y' => {
                text_converted_line1.push_str(&format!("{}Y       Y{}{}", text_bound, text_bound, text_space));
                text_converted_line2.push_str(&format!("{}  Y   Y  {}{}", text_bound, text_bound, text_space));
                text_converted_line3.push_str(&format!("{}   YYY   {}{}", text_bound, text_bound, text_space));
                text_converted_line4.push_str(&format!("{}    Y    {}{}", text_bound, text_bound, text_space));
                text_converted_line5.push_str(&format!("{}    Y    {}{}", text_bound, text_bound, text_space));
                    }
            'z' | 'Z' => {
                text_converted_line1.push_str(&format!("{}ZZZZZZZZZ{}{}", text_bound, text_bound, text_space));
                text_converted_line2.push_str(&format!("{}      ZZ {}{}", text_bound, text_bound, text_space));
                text_converted_line3.push_str(&format!("{}    Z    {}{}", text_bound, text_bound, text_space));
                text_converted_line4.push_str(&format!("{} ZZ      {}{}", text_bound, text_bound, text_space));
                text_converted_line5.push_str(&format!("{}ZZZZZZZZZ{}{}", text_bound, text_bound, text_space));
                    }
            ' ' => { // Space
                text_converted_line1.push_str(&format!("{}         {}{}", text_bound, text_bound, text_space));
                text_converted_line2.push_str(&format!("{}         {}{}", text_bound, text_bound, text_space));
                text_converted_line3.push_str(&format!("{}         {}{}", text_bound, text_bound, text_space));
                text_converted_line4.push_str(&format!("{}         {}{}", text_bound, text_bound, text_space));
                text_converted_line5.push_str(&format!("{}         {}{}", text_bound, text_bound, text_space));
                    }
            '\n' => (), // Do nothing
            _=> {
                text_converted_line1.push_str(&format!("{}    ??   {}{}", text_bound, text_bound, text_space));
                text_converted_line2.push_str(&format!("{}  ??  ?? {}{}", text_bound, text_bound, text_space));
                text_converted_line3.push_str(&format!("{}     ?   {}{}", text_bound, text_bound, text_space));
                text_converted_line4.push_str(&format!("{}         {}{}", text_bound, text_bound, text_space));
                text_converted_line5.push_str(&format!("{}    ??   {}{}", text_bound, text_bound, text_space));
                    }
        };
        // Once the line length has been hit, assembled that line and add it to the Vector
        if (char_counter == line_size) || (position_counter == text_length ) {
            char_counter = 0;
            text_converted.push(format!("{}\n{}\n{}\n{}\n{}", text_converted_line1, text_converted_line2, text_converted_line3, text_converted_line4, text_converted_line5));
            text_converted_line1.clear();
            text_converted_line2.clear();
            text_converted_line3.clear();
            text_converted_line4.clear();
            text_converted_line5.clear();
        }
        char_counter = char_counter + 1;
        position_counter = position_counter + 1;
    }

    return text_converted
}