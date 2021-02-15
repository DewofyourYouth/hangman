use std::fs::File;
use std::io::{BufRead, BufReader};

use ansi_term::{Colour, Style};
use rand::Rng;

fn main() {
    let phrases_src = "src/phrases.txt";
    let phrases_file = File::open(phrases_src).unwrap();
    let reader = BufReader::new(phrases_file);
    let mut phrases: Vec<String> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        phrases.push(line);
    }
    let num = rand::thread_rng().gen_range(0..phrases.len() - 1);
    // TODO abstract this to function / module - more elegant solution?
    let hang_pics = [
        "  +---+
  |   |
      |
      |
      |
      |
=========",
        "  +---+
  |   |
  O   |
      |
      |
      |
=========",
        "  +---+
  |   |
  O   |
  |   |
      |
      |
=========",
        "  +---+
  |   |
  O   |
 /|   |
      |
      |
=========",
        "  +---+
  |   |
  O   |
 /|\\  |
      |
      |
=========",
        "  +---+
  |   |
  O   |
 /|\\  |
 /    |
      |
=========",
        "  +---+
  |   |
  O   |   YOU
 /|\\  |   LOSE!!!!
 / \\  |   >:-P
      |
=========",
    ];

    print_ascii_title();
    let phrase = &phrases[num];
    let mut miss_count = 0;
    println!(
        "
===================================================================================================
    Secret Phrase:

    "
    );
    for c in phrase.as_bytes().iter() {
        match *c as char {
            'A'..='Z' | 'a'..='z' => print!("_"),
            _ => print!("{}", *c as char),
        };
    }
    println!("

===================================================================================================");
    print_ascii_prompt();
    let mut guess_vector = Vec::new();
    loop {
        let mut guess = String::new();
        std::io::stdin()
            .read_line(&mut guess)
            .ok()
            .expect("Failed to read line");
        let guess = guess.to_lowercase();
        let guess = guess.as_bytes();
        guess_vector.push(guess[0]);
        let g = guess[0];
        if !phrase.contains(g as char) {
            miss_count += 1;
        }
        println!("{}", Colour::Purple.paint(hang_pics[miss_count]));
        let mut ans_vec: Vec<i32> = Vec::new();
        // let mut solved = false;
        for c in phrase.as_bytes().iter() {
            match c {
                _ if guess_vector.contains(&c.to_ascii_lowercase()) => {
                    print!("{}", Style::new().bold().paint((*c as char).to_string()))
                }
                _ if *c < 65 || *c > 122 => print!("{}", *c as char),
                _ => {
                    ans_vec.push(1);
                    print!("_")
                }
            }
        }
        print!("\n");
        if ans_vec.len() == 0 {
            print_ascii_win();
            break;
        }

        if miss_count == hang_pics.len() - 1 {
            print_ascii_lose();
            break;
        } else {
            print_ascii_prompt();
        }
    }
}

fn print_ascii_title() {
    let title = "

                                                                                    .         .
8 8888        8          .8.          b.             8      ,o888888o.             ,8.       ,8.                   .8.          b.             8
8 8888        8         .888.         888o.          8     8888     `88.          ,888.     ,888.                 .888.         888o.          8
8 8888        8        :88888.        Y88888o.       8  ,8 8888       `8.        .`8888.   .`8888.               :88888.        Y88888o.       8
8 8888        8       . `88888.       .`Y888888o.    8  88 8888                 ,8.`8888. ,8.`8888.             . `88888.       .`Y888888o.    8
8 8888        8      .8. `88888.      8o. `Y888888o. 8  88 8888                ,8'8.`8888,8^8.`8888.           .8. `88888.      8o. `Y888888o. 8
8 8888        8     .8`8. `88888.     8`Y8o. `Y88888o8  88 8888               ,8' `8.`8888' `8.`8888.         .8`8. `88888.     8`Y8o. `Y88888o8
8 8888888888888    .8' `8. `88888.    8   `Y8o. `Y8888  88 8888   8888888    ,8'   `8.`88'   `8.`8888.       .8' `8. `88888.    8   `Y8o. `Y8888
8 8888        8   .8'   `8. `88888.   8      `Y8o. `Y8  `8 8888       .8'   ,8'     `8.`'     `8.`8888.     .8'   `8. `88888.   8      `Y8o. `Y8
8 8888        8  .888888888. `88888.  8         `Y8o.`     8888     ,88'   ,8'       `8        `8.`8888.   .888888888. `88888.  8         `Y8o.`
8 8888        8 .8'       `8. `88888. 8            `Yo      `8888888P'    ,8'         `         `8.`8888. .8'       `8. `88888. 8            `Yo


'||' '|'  ||       '||       '||   ||         '||          .|'''.|                    ||
  || |   ...     .. ||     .. ||  ...   ....   || ..       ||..  '   ....   .... ... ...  .. ...     ... .  ....
   ||     ||   .'  '||   .'  '||   ||  ||. '   ||' ||       ''|||.  '' .||   '|.  |   ||   ||  ||   || ||  ||. '
   ||     ||   |.   ||   |.   ||   ||  . '|..  ||  ||     .     '|| .|' ||    '|.|    ||   ||  ||    |''   . '|..
  .||.   .||.  '|..'||.  '|..'||. .||. |'..|' .||. ||.    |'....|'  '|..'|'    '|    .||. .||. ||.  '||||. |'..|'
                                                                            .. |                   .|....'
                                                                             ''

";
    println!("{}", Colour::Cyan.paint(title));
}

fn print_ascii_lose() {
    let lose = "


`8.`8888.      ,8'     ,o888888o.     8 8888      88           8 8888             ,o888888o.        d888888o.   8888888 8888888888
 `8.`8888.    ,8'   . 8888     `88.   8 8888      88           8 8888          . 8888     `88.    .`8888:' `88.       8 8888
  `8.`8888.  ,8'   ,8 8888       `8b  8 8888      88           8 8888         ,8 8888       `8b   8.`8888.   Y8       8 8888
   `8.`8888.,8'    88 8888        `8b 8 8888      88           8 8888         88 8888        `8b  `8.`8888.           8 8888
    `8.`88888'     88 8888         88 8 8888      88           8 8888         88 8888         88   `8.`8888.          8 8888
     `8. 8888      88 8888         88 8 8888      88           8 8888         88 8888         88    `8.`8888.         8 8888
      `8 8888      88 8888        ,8P 8 8888      88           8 8888         88 8888        ,8P     `8.`8888.        8 8888
       8 8888      `8 8888       ,8P  ` 8888     ,8P           8 8888         `8 8888       ,8P  8b   `8.`8888.       8 8888
       8 8888       ` 8888     ,88'     8888   ,d8P            8 8888          ` 8888     ,88'   `8b.  ;8.`8888       8 8888
       8 8888          `8888888P'        `Y88888P'             8 888888888888     `8888888P'      `Y8888P ,88P'       8 8888

";
    println!("{}", Colour::Red.paint(lose));
}

fn print_ascii_win() {
    let win = "


`8.`8888.      ,8'     ,o888888o.     8 8888      88           `8.`888b                 ,8'     ,o888888o.     b.             8
 `8.`8888.    ,8'   . 8888     `88.   8 8888      88            `8.`888b               ,8'   . 8888     `88.   888o.          8
  `8.`8888.  ,8'   ,8 8888       `8b  8 8888      88             `8.`888b             ,8'   ,8 8888       `8b  Y88888o.       8
   `8.`8888.,8'    88 8888        `8b 8 8888      88              `8.`888b     .b    ,8'    88 8888        `8b .`Y888888o.    8
    `8.`88888'     88 8888         88 8 8888      88               `8.`888b    88b  ,8'     88 8888         88 8o. `Y888888o. 8
     `8. 8888      88 8888         88 8 8888      88                `8.`888b .`888b,8'      88 8888         88 8`Y8o. `Y88888o8
      `8 8888      88 8888        ,8P 8 8888      88                 `8.`888b8.`8888'       88 8888        ,8P 8   `Y8o. `Y8888
       8 8888      `8 8888       ,8P  ` 8888     ,8P                  `8.`888`8.`88'        `8 8888       ,8P  8      `Y8o. `Y8
       8 8888       ` 8888     ,88'     8888   ,d8P                    `8.`8' `8,`'          ` 8888     ,88'   8         `Y8o.`
       8 8888          `8888888P'        `Y88888P'                      `8.`   `8'              `8888888P'     8            `Yo

";
    println!("{}", Colour::Green.paint(win));
}

fn print_ascii_prompt() {
    let prompt = "

 +-+ +-+ +-+ +-+ +-+ +-+ +-+ +-+ +-+ +-+ +-+ +-+
                Guess a letter
 +-+ +-+ +-+ +-+ +-+ +-+ +-+ +-+ +-+ +-+ +-+ +-+

";
    println!("{}", Colour::Yellow.paint(prompt));
}
