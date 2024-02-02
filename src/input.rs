use std::{self, io::{stdout, Write}, time::Duration};

use crossterm::event::{poll, read, Event, KeyCode, KeyEventKind};

use terminal_menu::{menu, run, mut_menu, TerminalMenuItem};

use crate::consts::editor_types::EditorTypes;

/// Reads a string from the user
/// 
/// # Arguments
/// 
/// * `message` - The message to display
/// 
/// # Returns
/// 
/// * `Result<String, std::io::Error>` - The result of the operation
/// 
pub fn input(message: &str) -> Result<String, std::io::Error> {
    print!("\n\n{}", message);
    stdout().flush()?;
    let mut input_string = String::new();
    std::io::stdin().read_line(&mut input_string)?;
    return Ok(input_string.trim().to_string());
}

/// Prompts the user to select an option from a menu
/// 
/// # Arguments
/// 
/// * `labels` - The labels of the menu
/// * `choices` - The choices of the menu
/// 
/// # Returns
/// 
/// * `Result<String, ()>` - The result of the operation
/// 
pub fn choice_menu(labels: Vec<&str>, choices: Vec<&str>) -> Result<String, ()> {
    
    let mut menu_vec: Vec<TerminalMenuItem> = vec![];

    for label in labels {
        menu_vec.push(terminal_menu::label(label));
    }
    menu_vec.push(terminal_menu::label(""));
    for choice in choices {
        menu_vec.push(terminal_menu::button(choice));
    }

    let menu = menu(menu_vec);
    run(&menu);

    let menu_result = mut_menu(&menu);

    if menu_result.canceled() {
        return Err(())
    }

    return Ok(menu_result.selected_item_name().to_string());
}

/// Prompts the user to select an option from a menu
/// The user can select multiple options
/// 
/// # Arguments
/// 
/// * `labels` - The labels of the menu
/// * `choices` - The choices of the menu. The first element of the tuple is the label, the second is the index of the selected item
/// 
/// # Returns
/// 
/// * `Result<Vec<String>, ()>` - The result of the operation
/// 
pub fn editor_menu<'a>(labels: Vec<&'a str>, choices: Vec<(&'a str, EditorTypes)>) -> Result<Vec<(&'a str, usize)>, ()> {
    
    let mut menu_vec: Vec<TerminalMenuItem> = vec![];

    for label in labels {
        menu_vec.push(terminal_menu::label(label));
    }
    menu_vec.push(terminal_menu::label(""));

    let mut choices_labels: Vec<&str> = vec![];

    for choice in choices {
        choices_labels.push(choice.0);
        match choice.1 {
            EditorTypes::Bool(is_true) => {
                menu_vec.push(
                    terminal_menu::list(choice.0, vec!["No", "Yes"]).set_selected_item(
                        match is_true {
                            true => 1,
                            false => 0,
                        }
                    )
                );
            },
            EditorTypes::StringArray(string_array, selected_idx) => {
                menu_vec.push(
                    terminal_menu::list(choice.0, string_array).set_selected_item(selected_idx)
                );
            },
        }
    }
    menu_vec.push(terminal_menu::button("Save"));

    let menu = menu(menu_vec);
    run(&menu);

    let menu_result = mut_menu(&menu);

    if menu_result.canceled() {
        return Err(())
    }

    let mut results: Vec<(&str, usize)> = vec![];
    for choice in choices_labels {
        results.push((choice, menu_result.selection_value_index(choice)));
    }

    Ok(results)
}

/// Prompts the user to confirm an action
/// 
/// # Arguments
/// 
/// * `message` - The message to display
/// * `default` - The default value
/// 
/// # Returns
/// 
/// * `Result<bool, std::io::Error>` - The result of the operation
/// 
pub fn bool_confirm(message: &str, default: bool) -> Result<bool, std::io::Error> {
    let input_suggestion ;
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
    //return E(())
}

/// Prompts the user to press ENTER to continue
/// 
/// # Arguments
/// 
/// * `message` - The message to display
/// 
pub fn pause(message: Option<&str>) {
    let mut wait_message = "Press ENTER to continue...";
    if let Some(message) = message {
        wait_message = message;
    }
    
    println!("{}", wait_message);

    let mut buffer = String::new();

    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");

    println!("\n");
}
