use uiautomation::controls::ControlType;
use uiautomation::types::UIProperty;
use uiautomation::variants::Variant;
use uiautomation::UIAutomation;
use uiautomation::UIElement;
use uiautomation::UITreeWalker;
use windows::{Win32::Foundation::POINT, Win32::UI::WindowsAndMessaging::GetCursorPos};

fn main() {
    let automation = UIAutomation::new().unwrap();
    let filter = automation
        .create_property_condition(
            UIProperty::ControlType,
            Variant::from(ControlType::Pane as i32),
            None,
        )
        .unwrap();
    let walker = automation.filter_tree_walker(filter).unwrap(); //automation.get_control_view_walker().unwrap();

    loop {
        let cursor_pos = get_cursor_position().expect("Failed to get cursor position");
        // println!("Cursor position: {:?}", cursor_pos);
        if let Ok(root) = automation
            .element_from_point(uiautomation::types::Point::new(cursor_pos.x, cursor_pos.y))
        {
            print_element(&walker, &root, 0).unwrap();
        }
        // sleep for 1 second
        // std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

fn print_element(
    walker: &UITreeWalker,
    element: &UIElement,
    level: usize,
) -> uiautomation::Result<()> {
    for _ in 0..level {
        print!(" ")
    }
    println!("{} - {}", element.get_classname()?, element.get_name()?);

    if let Ok(child) = walker.get_first_child(&element) {
        print_element(walker, &child, level + 1)?;

        let mut next = child;
        while let Ok(sibling) = walker.get_next_sibling(&next) {
            print_element(walker, &sibling, level + 1)?;

            next = sibling;
        }
    }

    Ok(())
}

fn get_cursor_position() -> Result<POINT, windows::core::Error> {
    unsafe {
        let mut point = POINT::default();
        GetCursorPos(&mut point)?;
        Ok(point)
    }
}
