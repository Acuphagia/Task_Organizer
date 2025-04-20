fn main()
{
    println!("Terminal, online.");

    let mut tasks: Vec<String> = Vec::new();

    loop
    {
        clear_screen();
        println!("Menu Options:");
        println!("1: Add Tasks");
        println!("2: View Tasks");
        println!("3: Remove Tasks");
        println!("4: Exit");
        println!("5: Clear Screen");
        println!("6: Mark As Complete");
        println!("7: Save & Load");
        println!("8: Help");
        println!("9: Return to Main Menu");
        let mut response: String = String::new();
        std::io::stdin()
            .read_line(&mut response)
            .expect("Failed to read line");
        let response: i8 = match response.trim().parse()
        {
            Ok(num) => num,
            Err(_) =>
            {
                println!("Please type a valid number!");
                pause();
                continue;
            }
        };

        match response
        {
            1 =>
            {
                add_task(&mut tasks);
            }
            2 =>
            {
                view_tasks(&tasks);
            }
            3 =>
            {
                remove_task(&mut tasks);
            }
            4 =>
            {
                exit();
            }
            5 =>
            {
                clear_screen();
            }
            6 =>
            {
                mark_as_complete();
            }
            7 =>
            {
                save_load();
            }
            8 =>
            {
                help();
            }
            9 =>
            {
                continue; // Return to main menu
            }
            _ =>
            {
                println!("Invalid option, please try again.");
            }
        }
    }
}

fn add_task(tasks: &mut Vec<String>)
{
    clear_screen();
    println!("Enter the task to add:");
    let mut action = String::new();
    std::io::stdin()
        .read_line(&mut action)
        .expect("Failed to read line");
    tasks.push(action.trim().to_string());
    println!("Task added successfully.");
    pause();
}

fn view_tasks(tasks: &Vec<String>)
{
    clear_screen();
    if tasks.is_empty()
    {
        println!("No tasks available.");
    }
    else
    {
        for (i, task) in tasks.iter().enumerate()
        {
            println!("{}. {}", i + 1, task);
        }
    }
    pause();
}

fn remove_task(tasks: &mut Vec<String>)
{
    clear_screen();
    println!("Enter the task number to remove:");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    if let Ok(index) = input.trim().parse::<usize>()
    {
        if index > 0 && index <= tasks.len()
        {
            tasks.remove(index - 1);
            println!("Task removed successfully.");
        }
        else
        {
            println!("Invalid task number.");
        }
    }
    else
    {
        println!("Please enter a valid number.");
    }
    pause();
}

fn exit()
{
    clear_screen();
    println!("Exiting the program.");
    std::process::exit(0);
}

fn clear_screen()
{
    print!("\x1B[2J\x1B[1;1H");
}

fn mark_as_complete()
{
    clear_screen();
    println!("Mark As Complete function called.");
    pause();
}

fn save_load()
{
    clear_screen();
    println!("Save & Load function called.");
    pause();
}

fn help()
{
    clear_screen();
    println!("If the buttons ain't working for you, then please ensure that you are typing in the options as numbers.");
    pause();
}

fn pause()
{
    println!("Press Enter to return to the main menu...");
    let mut _pause = String::new();
    std::io::stdin()
        .read_line(&mut _pause)
        .expect("Failed to read line");
}
