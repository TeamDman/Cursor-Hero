use cursor_hero_winutils::win_events::create_os_event_listener;
use cursor_hero_winutils::win_events::ProcMessage;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: <command> [all|events|mouse|keyboard]");
        std::process::exit(1);
    }

    let result = match args[1].as_str() {
        "all" => listen_all(),
        "events" => listen_events(),
        "mouse" => listen_mouse(),
        "keyboard" => listen_keyboard(),
        _ => {
            eprintln!("Invalid argument: choose from [all|events|mouse|keyboard]");
            std::process::exit(1);
        }
    };

    if let Err(e) = result {
        eprintln!("Error occurred: {:?}", e);
        std::process::exit(1);
    }
}

pub fn listen_all() -> Result<(), windows::core::Error> {
    let rx = create_os_event_listener()?;
    while let Ok(msg) = rx.recv() {
        println!("Received message: {:?}", msg);
    }
    Ok(())
}

pub fn listen_events() -> Result<(), windows::core::Error> {
    let rx = create_os_event_listener()?;
    while let Ok(msg) = rx.recv() {
        if !matches!(msg, ProcMessage::Event { .. }) {
            continue;
        }
        println!("Received message: {:?}", msg);
    }
    Ok(())
}

pub fn listen_mouse() -> Result<(), windows::core::Error> {
    let rx = create_os_event_listener()?;
    while let Ok(msg) = rx.recv() {
        if !matches!(msg, ProcMessage::MouseMoved { .. }) {
            continue;
        }
        println!("Received message: {:?}", msg);
    }
    Ok(())
}

pub fn listen_keyboard() -> Result<(), windows::core::Error> {
    let rx = create_os_event_listener()?;
    while let Ok(msg) = rx.recv() {
        if !matches!(msg, ProcMessage::KeyDown { .. }) {
            continue;
        }
        println!("Received message: {:?}", msg);
    }
    Ok(())
}
