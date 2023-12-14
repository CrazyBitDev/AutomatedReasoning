use std::{self, io::stdout, io::Write, time::Duration, io::Result};

use crossterm::{
    cursor::position,
    event::{poll, read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
};

const HELP: &str = r#"Blocking poll() & non-blocking read()
 - Keyboard, mouse and terminal resize events enabled
 - Prints "." every second if there's no event
 - Hit "c" to print current cursor position
 - Use Esc to quit
"#;

fn print_events() -> Result<()> {
    loop {
        // Wait up to 1s for another event
        if poll(Duration::from_millis(1_000))? {
            // It's guaranteed that read() wont block if `poll` returns `Ok(true)`
            let event = read()?;

            println!("Event::{:?}\r", event);

            if event == Event::Key(KeyCode::Char('c').into()) {
                println!("Cursor position: {:?}\r", position());
            }

            if event == Event::Key(KeyCode::Esc.into()) {
                break;
            }
        } else {
            // Timeout expired, no event for 1s
            println!(".\r");
        }
    }

    Ok(())
}

pub fn input_formatted() -> Result<()> {
    
    let mut input_string: Vec<String> = Vec::new();
    let mut last_len = 0;
    let mut cursor: isize = 0;

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
                            if key_event.code == KeyCode::Char('a') {
                                input_string.insert(cursor as usize, "∧".to_string());
                                cursor += 1;
                            } else if key_event.code == KeyCode::Char('v') {
                                input_string.insert(cursor as usize, '∨'.to_string());
                                cursor += 1;
                            }
                        } else {
                            if c == '<' || c == '>' || c == '-' {
                                check_arrows = true;   
                            }
                                
                            input_string.insert(cursor as usize, c.to_string());
                            cursor += 1;
                        }
                    } else if KeyCode::Backspace == key_event.code && cursor > 0{
                        input_string.remove(cursor as usize - 1);
                        cursor -= 1;
                    } else if KeyCode::Delete == key_event.code && cursor < input_string.len() as isize {
                        input_string.remove(cursor as usize);
                    } else if KeyCode::Enter == key_event.code {
                        //input_string
                    } else if KeyCode::Left == key_event.code {
                        cursor -= 1;
                    } else if KeyCode::Right == key_event.code {
                        cursor += 1;
                        if cursor > input_string.len() as isize {
                            cursor = input_string.len() as isize;
                        }
                    }

                    if cursor < 0 {
                        cursor = 0;
                    }
                    if check_arrows {

                        let mut temp_cursor = 0;
                        let mut left_bracket = -1;
                        let mut minus = -1;
                        let mut right_bracket = -1;
                        let mut right_arrow = -1;

                        loop {
                            if temp_cursor < input_string.len() {
                                if input_string[temp_cursor] == "<" {
                                    left_bracket = temp_cursor as isize;
                                } else if input_string[temp_cursor] == "-" {
                                    minus = temp_cursor as isize;
                                } else if input_string[temp_cursor] == ">" {
                                    right_bracket = temp_cursor as isize;
                                } else if input_string[temp_cursor] == "→" {
                                    right_arrow = temp_cursor as isize;
                                } else if input_string[temp_cursor] != " " {
                                    left_bracket = -1;
                                    minus = -1;
                                    right_arrow = -1;
                                }

                                if minus >= 0 && right_bracket >= 0 {
                                    let mut double = false;
                                    let mut min = minus;
                                    let max = right_bracket;
                                    if left_bracket >= 0 {
                                        double = true;
                                        min = left_bracket;
                                    }

                                    //remove all characters from max to min
                                    for _ in min+1..max+1 {
                                        input_string.remove((min+1) as usize);

                                        if cursor > min {
                                            cursor -= 1;
                                        }
                                    }

                                    if double {
                                        input_string[min as usize] = "⟷".to_string();
                                    } else {
                                        input_string[min as usize] = "→".to_string();
                                    }

                                    minus = -1;
                                    right_bracket = -1;
                                    left_bracket = -1;

                                    temp_cursor = 0;
                                } else if left_bracket >= 0 && right_arrow >= 0 {

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

            //println!("Event::{:?}\r", event);

            /*if event == Event::Key(KeyCode::Char('c').into()) {
                println!("Cursor position: {:?}\r", position());
            }*/

            if event == Event::Key(KeyCode::Esc.into()) {
                break;
            }
        }
    }
    Ok(())
}

/*fn main() -> Result<()> {
    println!("{}", HELP);

    enable_raw_mode()?;

    let mut stdout = stdout();
    execute!(stdout, EnableMouseCapture)?;

    if let Err(e) = print_events() {
        println!("Error: {:?}\r", e);
    }

    execute!(stdout, DisableMouseCapture)?;

    disable_raw_mode()
}*/