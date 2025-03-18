/* specifications Zalgo program\
*The program must convert a user typed text to zalgo. It must also let the user choose level of complexity of the zalgo
* 
*There are three type of diacritics(character modifiers used in lanauges), upper, middle and lower ones, each of them change the
* upper, middle and lower parts of each character. The three of them must be used to make it more crazy
*
* The diacritics exist in unicode, Try to make a vector that contains ALL of the diacritics. One vector for each type of diacritics
*(upper,middle and lower)
*
* Thats all, and its gonna be a lot of work 乂 __ 乂
*
*
*/

use rand::{random, Rng}; // You'll need the rand crate to randomly apply the diacritics
use std::{fs::read, io};


use colored::*;


fn main() {
    
   // print!("{}",format!("A{}{}{}{}",ZALGO_UP[22],ZALGO_UP[20],ZALGO_UP[19],ZALGO_UP[0]));
    let complexity: u32 = read_complexity();
    println!("{}",zalgo_transform(complexity, read_string())); 
    
    //test lines
  // let mut test   = String::new();
    //test = String::from("ola tudo bemmm");
   // println!("{}",zalgo_transform(complexity, test)); 
   /// test lines end
}
///* 


fn read_string() -> String{
    let mut input = String::new();
    println!("Write down the words you want to convert to zalgo:");
   //io::stdout().flush().unwrap(); // 
    io::stdin().read_line(&mut input).expect("Failed to read input");
   // input.trim().to_string(); 
    return input;
}


fn zalgo_transform(complexity: u32, inputstr: String) -> String{
    let mut zalgo_string = String::new();
    let mut rng = rand::thread_rng();
    
    for cha in inputstr.chars(){
       // println!("Transformin char:{}", cha);
        zalgo_string.push(cha);// ad new character
        
        for i in 0..complexity {// add top diacritics
            let random_int1 = rng.gen_range(0..ZALGO_UP.len());// later on should make a different gen to each type of mod
          //  println!("top Iteration {}, generated the number {}, which is the diacritic {} ", i,random_int1, ZALGO_UP[random_int1] );
           // println!("number generated: {}", random_int1);
            zalgo_string.push(ZALGO_UP[random_int1]);
        }
        /*for i in 0..complexity {// add middle diacritics
            let random_int2 = rng.gen_range(0..ZALGO_MIDDLE.len());// later on should make a different gen to each type of mod
            println!("mid Iteration {}, generated the number {}, which is the diacritic {} ", i,random_int2, ZALGO_MIDDLE[random_int2] );
           // println!("number generated: {}", random_int2);
            zalgo_string.push(ZALGO_MIDDLE[random_int2]);
        }*//*
            for i in 0..2 {// add middle diacritics
            let random_int2 = rng.gen_range(0..ZALGO_MIDDLE.len());// later on should make a different gen to each type of mod
            println!("mid Iteration {}, generated the number {}, which is the diacritic {} ", i,random_int2, ZALGO_MIDDLE[random_int2] );
           // println!("number generated: {}", random_int2);
            zalgo_string.push(ZALGO_MIDDLE[random_int2]);
        }*/
        for i in 0..complexity {// add bottom(down) diacritics
            let random_int3 = rng.gen_range(0..ZALGO_DOWN.len());// later on should make a different gen to each type of mod
         //   println!("Down Iteration {}, generated the number {}, which is the diacritic {} ", i,random_int3, ZALGO_DOWN[random_int3] );
           // println!("mid Iteration {}", i);
          //  println!("number generated: {}", random_int3);
            zalgo_string.push(ZALGO_DOWN[random_int3]);
        }



    }
    return zalgo_string;
}
/* 
fn read_types_of_distortion(){
    print!




}*/

fn read_complexity() -> u32{
    let mut int_complexity_choice: u32;
    loop {
        let mut complexity_choice = String::new();
        println!("Type the ammount of complexity in the zalgo generation:"); // Assuming this function prints out the complexity options
        
        io::stdin()
            .read_line(&mut complexity_choice)
            .expect("Failed to read line");

        int_complexity_choice = match complexity_choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "\n\nInput a positive number! :C ".bright_red());
                continue;
            },
        };

        if int_complexity_choice <= 0 {
            println!("{}", "\n\nPrint a valid number!! One bigger than 0".bright_red());
            continue;
        } else { 
            
            return int_complexity_choice
        }
    }
   
   
   


}

// ALL DIACRITICS THAT EXIST IN UNICODE ACCORDING TO CHAT GPT

const ZALGO_UP: &[char] = &[
    '\u{0300}', // Grave accent
    '\u{0301}', // Acute accent
    '\u{0302}', // Circumflex
    '\u{0303}', // Tilde
    '\u{0304}', // Macron
    '\u{0305}', // Overline
    '\u{0306}', // Breve
    '\u{0307}', // Dot above
    '\u{0308}', // Diaeresis
    '\u{0309}', // Hook above
    '\u{030A}', // Ring above
    '\u{030B}', // Double acute accent
    '\u{030C}', // Caron
    '\u{030F}', // Double grave accent
    '\u{0310}', // Candrabindu
    '\u{0311}', // Inverted breve
    '\u{0313}', // Greek koronis
    '\u{0314}', // Greek psili
    '\u{031B}', // Horn
    '\u{033D}', // X above
    '\u{0340}', // Grave tone mark
    '\u{0341}', // Acute tone mark
    '\u{0342}', // Greek perispomeni
    '\u{0343}', // Greek koronides
    '\u{0344}', // Dialytika tonos (Greek)
    '\u{0346}', // Bridge above
    '\u{034F}', // Combining grapheme joiner
    '\u{0350}', // Right arrowhead above
    '\u{0351}', // Left half ring above
    '\u{0352}', // Fermata
    '\u{0357}', // Right half ring above
    '\u{0358}', // Dot above right
    '\u{035B}', // Reversed comma above
    '\u{0363}', // Greek letters in superscript (some accents on vowels)
    '\u{036F}', // Nonspacing (various small marks)
];

//const ZALGO_UP: &[char] = &[
const ZALGO_MIDDLE: &[char] = &[
        '\u{0334}', // Combining tilde overlay
        '\u{0335}', // Combining short stroke overlay
        '\u{0336}', // Combining long stroke overlay (strikethrough)
        '\u{0337}', // Combining short solidus overlay (short slash)
        '\u{0338}', // Combining long solidus overlay (long slash)
        '\u{20E5}', // Combining reverse slash
        '\u{0346}', // Combining bridge above (used in scripts like Cyrillic)
        '\u{034A}', // Combining left right arrowhead below (double arrow overlay)
        '\u{034B}', // Combining right arrowhead below
        '\u{034C}', // Combining right arrowhead above
        '\u{20D2}', // Combining enclosing square
        '\u{20D3}', // Combining enclosing diamond
        '\u{20D4}', // Combining enclosing circle
        '\u{20D5}', // Combining enclosing circle backslash
        '\u{20D6}', // Combining left arrow above
        '\u{20D7}', // Combining right arrow above
        '\u{20E6}', // Combining double vertical stroke overlay
        '\u{20E7}', // Combining ankh above
        '\u{20E8}', // Combining long vertical line overlay
        '\u{20E9}', // Combining left arrow below
 ];
 
 const ZALGO_DOWN: &[char] = &[
        '\u{0316}', // Subscript grave
        '\u{0317}', // Subscript acute
        '\u{0318}', // Subscript circumflex
        '\u{0319}', // Subscript inverted breve
        '\u{031C}', // Left half ring below
        '\u{031D}', // Right half ring below
        '\u{031E}', // Left tack below
        '\u{031F}', // Right tack below
        '\u{0320}', // Long stroke below
        '\u{0323}', // Dot below
        '\u{0324}', // Diaeresis below
        '\u{0325}', // Ring below
        '\u{0326}', // Comma below
        '\u{0327}', // Cedilla
        '\u{0328}', // Ogonek
        '\u{0329}', // Vertical line below
        '\u{032A}', // Bridge below
        '\u{032B}', // Inverted double arch below
        '\u{032C}', // Caron below
        '\u{032D}', // Circumflex below
        '\u{032E}', // Breve below
        '\u{032F}', // Inverted breve below
        '\u{0330}', // Tilde below
        '\u{0331}', // Macron below
        '\u{0332}', // Low line (underscore)
        '\u{0333}', // Double low line (double underscore)
        '\u{0339}', // Left half ring below
        '\u{033A}', // Right half ring below
        '\u{033B}', // Reversed comma below
        '\u{033C}', // Reversed double arch below
        '\u{0345}', // Iota subscript (Greek)
        '\u{0347}', // Equals sign below
        '\u{0348}', // Double vertical line below
        '\u{0349}', // Left arrowhead below
        '\u{034A}', // Right arrowhead below
        '\u{034B}', // Left right arrowhead below
        '\u{034C}', // Right arrowhead below
        '\u{0353}', // X below
        '\u{0359}', // Vertical tilde below
        '\u{035A}', // Double breve below
    ];



