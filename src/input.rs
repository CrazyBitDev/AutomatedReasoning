use std::{self, io::stdout, io::Write, time::Duration, io::Result};

use crossterm::{
    cursor::position,
    event::{poll, read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
};

pub fn input_formatted() -> Result<Vec<String>> {

    let characters: Vec<String> = vec!["←".to_string(), "<".to_string(), "-".to_string(), ">".to_string(), "→".to_string()];
    
    let mut input_string: Vec<String> = Vec::new();
    let mut last_len = 0;
    let mut cursor: isize = 0;
    let mut show_math_characters = true;

    loop {
        // Wait up to 1s for another event
        if poll(Duration::from_millis(1_000))? {
            // It's guaranteed that read() wont block if `poll` returns `Ok(true)`
            let event = read()?;

            /*if event.kind == KeyEvent::Release() {
                println!(event)
            }*/

            if let Event::Key(key_event) = event {
                if key_event.kind == KeyEventKind::Press {

                    let mut check_arrows = false;

                    if let KeyCode::Char(c) = key_event.code {
                        if let KeyModifiers::CONTROL = key_event.modifiers {
                            if key_event.code == KeyCode::Char('a') || key_event.code == KeyCode::Char('A') {
                                if show_math_characters {
                                    input_string.insert(cursor as usize, "∧".to_string());
                                } else {
                                    input_string.insert(cursor as usize, "+".to_string());
                                }
                                cursor += 1;
                            } else if key_event.code == KeyCode::Char('v') || key_event.code == KeyCode::Char('V') {
                                if show_math_characters {
                                    input_string.insert(cursor as usize, "∨".to_string());
                                } else {
                                    input_string.insert(cursor as usize, "*".to_string());
                                }
                                cursor += 1;
                            } else if key_event.code == KeyCode::Char('z') || key_event.code == KeyCode::Char('Z') {
                                show_math_characters = !show_math_characters;

                                let mut temp_cursor = 0;
                                loop {
                                    if input_string[temp_cursor] == "∧" && !show_math_characters {
                                        input_string[temp_cursor] = "+".to_string();
                                    } else if input_string[temp_cursor] == "∨" && !show_math_characters {
                                        input_string[temp_cursor] = "*".to_string();
                                    } else if input_string[temp_cursor] == "+" && show_math_characters {
                                        input_string[temp_cursor] = "∧".to_string();
                                    } else if input_string[temp_cursor] == "*" && show_math_characters {
                                        input_string[temp_cursor] = "∨".to_string();
                                    }
                                    
                                    temp_cursor += 1;
                                    if temp_cursor >= input_string.len() {
                                        break;
                                    }
                                }
                            }
                        } else if key_event.code == KeyCode::Char('+') {
                            if show_math_characters {
                                input_string.insert(cursor as usize, "∧".to_string());
                            } else {
                                input_string.insert(cursor as usize, "+".to_string());
                            }
                            cursor += 1;
                        } else if key_event.code == KeyCode::Char('*') {
                            if show_math_characters {
                                input_string.insert(cursor as usize, "∨".to_string());
                            } else {
                                input_string.insert(cursor as usize, "*".to_string());
                            }
                            cursor += 1;
                        } else {
                            if c == '<' || c == '>' || c == '-' {
                                check_arrows = true;   
                            }
                                
                            input_string.insert(cursor as usize, c.to_string());
                            cursor += 1;
                        }
                    } else if KeyCode::Backspace == key_event.code && cursor > 0{
                        check_arrows = true;
                        input_string.remove(cursor as usize - 1);
                        cursor -= 1;
                    } else if KeyCode::Delete == key_event.code && cursor < input_string.len() as isize {
                        check_arrows = true;
                        input_string.remove(cursor as usize);
                    } else if KeyCode::Enter == key_event.code {
                        return Ok(input_string);
                    } else if KeyCode::Left == key_event.code {
                        cursor -= 1;
                    } else if KeyCode::Right == key_event.code {
                        cursor += 1;
                        if cursor > input_string.len() as isize {
                            cursor = input_string.len() as isize;
                        }
                    } else if KeyCode::Esc == key_event.code {
                        break;
                    }

                    if cursor < 0 {
                        cursor = 0;
                    }
                    if check_arrows {

                        let mut temp_cursor = 0;

                        let mut left_arrow = -1;
                        let mut left_bracket = -1;
                        let mut minus = -1;
                        let mut right_bracket = -1;
                        let mut right_arrow = -1;

                        loop {
                            if temp_cursor < input_string.len() {
                                let mut found_this_char = false;

                                if input_string[temp_cursor] == "←" {
                                    if left_bracket >= 0 {
                                        left_bracket = -1;
                                    }
                                    left_arrow = temp_cursor as isize;
                                    found_this_char = true;
                                } else if input_string[temp_cursor] == "<" {
                                    left_bracket = temp_cursor as isize;
                                    found_this_char = true;
                                } else if input_string[temp_cursor] == "-" {
                                    minus = temp_cursor as isize;
                                    found_this_char = true;
                                } else if input_string[temp_cursor] == ">" {
                                    right_bracket = temp_cursor as isize;
                                    found_this_char = true;
                                } else if input_string[temp_cursor] == "→" {
                                    right_arrow = temp_cursor as isize;
                                    found_this_char = true;
                                }
                                if minus >= 0 && right_bracket >= 0 {
                                    let mut double = false;
                                    let mut min = minus;
                                    let mut max = right_bracket;
                                    if left_bracket >= 0 && left_bracket < minus {
                                        double = true;
                                        min = left_bracket;
                                    }

                                    if !double {
                                        max += 1;
                                    }

                                    //remove all characters from max to min
                                    for _ in min+1..max {
                                        input_string.remove((min+1) as usize);

                                        if cursor > min {
                                            cursor -= 1;
                                        }
                                    }

                                    if double {
                                        input_string[min as usize] = "←".to_string();
                                        min += 1;
                                    }
                                    input_string[(min) as usize] = "→".to_string();

                                    minus = -1;
                                    right_bracket = -1;
                                    left_bracket = -1;

                                    temp_cursor = min as usize;
                                } else if left_bracket >= 0 && right_arrow >= 0 && left_bracket < right_arrow{

                                    //remove all characters from max to min
                                    for _ in left_bracket+1..right_arrow {
                                        input_string.remove((left_bracket) as usize);
                                        if cursor > left_bracket {
                                            cursor -= 1;
                                        }
                                    }

                                    input_string[left_bracket as usize] = "←".to_string();
                                    cursor = left_bracket+1;
                                        
                                    minus = -1;
                                    right_bracket = -1;
                                    left_bracket = -1;

                                    temp_cursor = 0;
                                } else if (!found_this_char && characters.contains(&input_string[temp_cursor])) || temp_cursor == input_string.len() - 1 {

                                    //if left_arrow is not -1
                                    if left_arrow >= 0 && right_arrow < 0 {
                                        input_string.remove(left_arrow as usize);
                                        cursor -= 1;
                                        temp_cursor = left_arrow as usize;                                        
                                    }
                                    left_arrow = -1;
                                    left_bracket = -1;
                                    minus = -1;
                                    right_bracket = -1;
                                    right_arrow = -1;
                                }


                            } else {
                                break;
                            }
                            temp_cursor += 1;
                        }
                    }

                    print!("\r{}", input_string.join(""));
                    if last_len > input_string.len() {
                        print!("{}", " ".repeat(last_len - input_string.len()))
                    }
                    print!("\r{}", input_string[..cursor as usize].join(""));
                    last_len = input_string.len();
                    stdout().flush()?;
                }
            }
        }
    }
    return Ok(Vec::<String>::new());
}

pub fn bool_confirm(message: &str, default: bool) -> Result<bool> {
    let mut input_suggestion = "";
    if default {
        input_suggestion = "Y/n";
    } else {
        input_suggestion = "y/N";
    }
    print!("{} ({}) ", message, input_suggestion);
    stdout().flush()?;

    
    loop {
        // Wait up to 1s for another event
        if poll(Duration::from_millis(1_000))? {

            let event = read()?;

            if let Event::Key(key_event) = event {
                if key_event.kind == KeyEventKind::Press {
                    if let KeyCode::Char(c) = key_event.code {
                        if c == 'y' || c == 'Y' {
                            println!("{}", 'Y');
                            return Ok(true);
                        } else if c == 'n' || c == 'N' {
                            println!("{}", 'N');
                            return Ok(false);
                        }
                    } else if KeyCode::Enter == key_event.code {
                        if default {
                            println!("{}", 'Y');
                        } else {
                            println!("{}", 'N');
                        }
                        return Ok(default);
                    }
                }
            }
        }
    }
}