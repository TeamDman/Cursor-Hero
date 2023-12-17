use windows::Win32::System::Com::{CoCreateInstance, CLSCTX_ALL};
use windows::{
    core::Result,
    Win32::{
        Foundation::POINT,
        System::Com::{CoInitializeEx, CoUninitialize, COINIT_APARTMENTTHREADED},
        UI::Accessibility::{CUIAutomation, IUIAutomation, IUIAutomationElement},
        UI::WindowsAndMessaging::GetCursorPos,
    },
};

fn main() -> Result<()> {
    unsafe {
        CoInitializeEx(Some(std::ptr::null_mut()), COINIT_APARTMENTTHREADED)?;

        // Create an instance of IUIAutomation
        // You might need to find the right function or method to do this.
        let automation: IUIAutomation = CoCreateInstance(
            &CUIAutomation, // This would be the CLSID of the CUIAutomation class
            None,
            CLSCTX_ALL,
        )?;
        let mut point = POINT::default();
        GetCursorPos(&mut point).unwrap();
        let element: IUIAutomationElement = automation.ElementFromPoint(point)?;
        
        let element_name = element.CurrentName()?.to_string();

        println!("Element Name: {}", element_name);

        CoUninitialize();
    }

    Ok(())
}
