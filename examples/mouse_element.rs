use windows::core::Result;
use windows::Win32::Foundation::POINT;
use windows::Win32::System::Com::CoCreateInstance;
use windows::Win32::System::Com::CoInitializeEx;
use windows::Win32::System::Com::CoUninitialize;
use windows::Win32::System::Com::CLSCTX_ALL;
use windows::Win32::System::Com::COINIT_APARTMENTTHREADED;
use windows::Win32::UI::Accessibility::CUIAutomation;
use windows::Win32::UI::Accessibility::IUIAutomation;
use windows::Win32::UI::Accessibility::IUIAutomationElement;
use windows::Win32::UI::WindowsAndMessaging::GetCursorPos;

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
