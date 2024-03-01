#![allow(dead_code)]
use bevy::log::debug;
use bevy::log::error;
use bevy::log::info;
use bevy::math::IVec2;
use windows::Win32::Devices::HumanInterfaceDevice::HID_USAGE_GENERIC_MOUSE;
use windows::Win32::Devices::HumanInterfaceDevice::HID_USAGE_PAGE_GENERIC;
use windows::Win32::Foundation::*;
use windows::Win32::System::Ole::VarBstrFromDec;
use windows::Win32::System::Variant::VARIANT;
use windows::Win32::System::Variant::VT_BSTR;
use windows::Win32::System::Variant::VT_I4;
use windows::Win32::UI::Accessibility::*;
use windows::Win32::UI::Input::GetRawInputData;
use windows::Win32::UI::Input::RegisterRawInputDevices;
use windows::Win32::UI::Input::HRAWINPUT;
use windows::Win32::UI::Input::RAWINPUT;
use windows::Win32::UI::Input::RAWINPUTDEVICE;
use windows::Win32::UI::Input::RAWINPUTHEADER;
use windows::Win32::UI::Input::RIDEV_INPUTSINK;
use windows::Win32::UI::Input::RID_INPUT;
use windows::Win32::UI::Input::RIM_TYPEMOUSE;
use windows::Win32::UI::WindowsAndMessaging::EVENT_OBJECT_CREATE;
use windows::Win32::UI::WindowsAndMessaging::EVENT_OBJECT_DESTROY;
use windows::Win32::UI::WindowsAndMessaging::EVENT_OBJECT_HIDE;
use windows::Win32::UI::WindowsAndMessaging::EVENT_OBJECT_LIVEREGIONCHANGED;
use windows::Win32::UI::WindowsAndMessaging::EVENT_OBJECT_STATECHANGE;
use windows::Win32::UI::WindowsAndMessaging::EVENT_OBJECT_VALUECHANGE;
use windows::Win32::UI::WindowsAndMessaging::*;

#[derive(Debug)]
pub enum EventHookError {
    Unknown,
}
pub fn register_interest_in_all_events_with_os_with_default_callback(
) -> Result<isize, EventHookError> {
    unsafe {
        match SetWinEventHook(
            EVENT_MIN, // or specific event codes
            EVENT_MAX, // or specific event codes
            None,      // hmodWinEventProc
            Some(win_event_proc),
            0, // idProcess
            0, // idThread
            WINEVENT_OUTOFCONTEXT,
        ) {
            HWINEVENTHOOK(0) => Err(EventHookError::Unknown),
            HWINEVENTHOOK(x) => Ok(x),
        }
    }
}

pub fn register_interest_in_mouse_with_os(hwnd: isize) -> Result<(), windows::core::Error> {
    unsafe {
        let raw_input_device = RAWINPUTDEVICE {
            usUsagePage: HID_USAGE_PAGE_GENERIC,
            usUsage: HID_USAGE_GENERIC_MOUSE,
            dwFlags: RIDEV_INPUTSINK,
            hwndTarget: HWND(hwnd),
        };
        RegisterRawInputDevices(
            &mut [raw_input_device],
            std::mem::size_of::<RAWINPUTDEVICE>() as u32,
        )
    }
}

#[derive(Debug)]
pub enum MessageLoopError {
    WIN32,
}
#[derive(Debug)]
pub enum MessageLoopMessage {
    MouseMoved(IVec2),
    Unknown,
}
pub fn message_loop(
    hwnd: isize,
    receiver: fn(x: MessageLoopMessage),
) -> Result<(), MessageLoopError> {
    unsafe {
        let mut win_msg = MSG::default();
        debug!("Launching message loop");
        while GetMessageW(&mut win_msg, HWND(hwnd), 0, 0).as_bool() {
            debug!("GOT MESSAGE: {:?}", win_msg);
            TranslateMessage(&win_msg);
            DispatchMessageW(&win_msg);

            if win_msg.message == WM_INPUT {
                let mut input_data: RAWINPUT = std::mem::zeroed();
                let mut size = std::mem::size_of::<RAWINPUT>() as u32;
                let recv_size = GetRawInputData(
                    HRAWINPUT(win_msg.lParam.0),
                    RID_INPUT,
                    Some(&mut input_data as *mut _ as *mut _),
                    &mut size,
                    std::mem::size_of::<RAWINPUTHEADER>() as u32,
                );
                if recv_size as i32 != -1 {
                    if input_data.header.dwType == RIM_TYPEMOUSE.0 {
                        let mouse_data = input_data.data.mouse;
                        let x = mouse_data.lLastX;
                        let y = mouse_data.lLastY;
                        receiver(MessageLoopMessage::MouseMoved(IVec2::new(x, y)));
                    }
                }
            } else {
                debug!("Unknown message: {:?}", win_msg.message);
            }
        }

        // if let Some(error) = windows::core::Error::from_win32() {
        //     error!("An error occurred in the message loop: {:?}", error);
        //     return Err(());
        // }

        Ok(())
    }
}

extern "system" fn win_event_proc(
    _h_win_event_hook: HWINEVENTHOOK,
    event: u32,
    hwnd: windows::Win32::Foundation::HWND,
    object_id: i32,
    child_id: i32,
    _id_event_thread: u32,
    _dwms_event_time: u32,
) {
    if event < 1000
        || event == EVENT_OBJECT_SHOW
        || event == EVENT_OBJECT_LOCATIONCHANGE
        || event == EVENT_OBJECT_NAMECHANGE
        || event == EVENT_OBJECT_REORDER
        || event == EVENT_OBJECT_VALUECHANGE
        || event == EVENT_OBJECT_CREATE
        || event == EVENT_OBJECT_DESTROY
        || event == EVENT_OBJECT_HIDE
        || event == EVENT_OBJECT_LIVEREGIONCHANGED
    {
        return;
    }
    // debug!(
    //     "Event: {:?} ({}), HWND: {:?}, idObject: {:?}, idChild: {:?}",
    //     event,
    //     event_to_name(event),
    //     hwnd,
    //     object_id,
    //     child_id
    // );
    if (event == EVENT_OBJECT_SELECTIONADD || event == EVENT_OBJECT_STATECHANGE)
        && object_id == OBJID_CLIENT.0
    {
        // Here you get the name and state of the element that triggered the event.
        // Implement the logic to retrieve the name and state using the AccessibleObjectFromEvent function.
        let mut acc_ptr: Option<IAccessible> = None;
        let mut elem = VARIANT::default();

        unsafe {
            let lookup = AccessibleObjectFromEvent(
                hwnd,
                object_id as u32,
                child_id as u32,
                &mut acc_ptr,
                &mut elem,
            );

            if lookup.is_ok() {
                let acc = acc_ptr.unwrap();
                let name_bstr = match acc.get_accName(elem.clone()) {
                    Ok(bstr) => bstr,
                    Err(e) => {
                        error!("Error getting name: {:?}", e);
                        return;
                    }
                };
                let role_var = match acc.get_accRole(elem.clone()) {
                    Ok(role) => role,
                    Err(e) => {
                        error!("Error getting role: {:?}", e);
                        return;
                    }
                };
                let state_var = match acc.get_accState(elem.clone()) {
                    Ok(state) => state,
                    Err(e) => {
                        error!("Error getting state: {:?}", e);
                        return;
                    }
                };

                let role = role_to_name(variant_to_int(&role_var).unwrap_or(-123) as u32);
                let state = state_to_string(variant_to_int(&state_var).unwrap_or(-123) as u32);

                let mut pxleft = 0;
                let mut pytop = 0;
                let mut pcxwidth = 0;
                let mut pcyheight = 0;
                if let Err(e) =
                    acc.accLocation(&mut pxleft, &mut pytop, &mut pcxwidth, &mut pcyheight, elem)
                {
                    error!("Error getting location: {:?}", e);
                    return;
                }
                let bounds = bevy::math::IRect::from_corners(
                    bevy::math::IVec2::new(pxleft, pytop),
                    bevy::math::IVec2::new(pxleft + pcxwidth, pytop + pcyheight),
                );
                info!(
                    "{} Name={:?} Role={:?} State={:?}, bounds={:?}",
                    event_to_name(event),
                    name_bstr.to_string(),
                    role,
                    state,
                    bounds,
                );
            }
        }
    }
}

fn decimal_to_string(decimal: DECIMAL) -> Result<String, windows::core::Error> {
    unsafe { VarBstrFromDec(&decimal, 0, 0).map(|bstr| bstr.to_string()) }
}

fn variant_to_int(variant: &VARIANT) -> Result<i32, windows::core::Error> {
    let var_type = unsafe { variant.Anonymous.Anonymous.vt };

    match var_type {
        VT_I4 => {
            // Extract 32-bit integer
            Ok(unsafe { variant.Anonymous.Anonymous.Anonymous.lVal })
        }
        _ => Err(windows::core::Error::new(
            windows::Win32::Foundation::E_FAIL,
            "Unsupported VARIANT type".into(),
        )),
    }
}

fn variant_to_string(variant: &VARIANT) -> Result<String, windows::core::Error> {
    let var_type = unsafe { variant.Anonymous.Anonymous.vt };

    match var_type {
        VT_BSTR => {
            // Extract BSTR and convert to String
            let bstr = unsafe { &variant.Anonymous.Anonymous.Anonymous.bstrVal };
            Ok(bstr.to_string())
        }
        VT_I4 => {
            // Extract 32-bit integer and convert to String
            let int_val = unsafe { variant.Anonymous.Anonymous.Anonymous.lVal };
            Ok(int_val.to_string())
        }
        // Add more cases as needed for other VARTYPEs you expect to handle
        _ => Err(windows::core::Error::new(
            windows::Win32::Foundation::E_FAIL,
            "Unsupported VARIANT type".into(),
        )),
    }
}

fn state_to_string(state: u32) -> String {
    let mut states = Vec::new();
    if state & STATE_SYSTEM_ALERT_HIGH != 0 {
        states.push("STATE_SYSTEM_ALERT_HIGH")
    }
    if state & STATE_SYSTEM_ALERT_LOW != 0 {
        states.push("STATE_SYSTEM_ALERT_LOW")
    }
    if state & STATE_SYSTEM_ALERT_MEDIUM != 0 {
        states.push("STATE_SYSTEM_ALERT_MEDIUM")
    }
    if state & STATE_SYSTEM_ANIMATED != 0 {
        states.push("STATE_SYSTEM_ANIMATED")
    }
    if state & STATE_SYSTEM_BUSY != 0 {
        states.push("STATE_SYSTEM_BUSY")
    }
    if state & STATE_SYSTEM_CHECKED != 0 {
        states.push("STATE_SYSTEM_CHECKED")
    }
    if state & STATE_SYSTEM_COLLAPSED != 0 {
        states.push("STATE_SYSTEM_COLLAPSED")
    }
    if state & STATE_SYSTEM_DEFAULT != 0 {
        states.push("STATE_SYSTEM_DEFAULT")
    }
    if state & STATE_SYSTEM_EXPANDED != 0 {
        states.push("STATE_SYSTEM_EXPANDED")
    }
    if state & STATE_SYSTEM_EXTSELECTABLE != 0 {
        states.push("STATE_SYSTEM_EXTSELECTABLE")
    }
    if state & STATE_SYSTEM_FLOATING != 0 {
        states.push("STATE_SYSTEM_FLOATING")
    }
    if state & STATE_SYSTEM_FOCUSED != 0 {
        states.push("STATE_SYSTEM_FOCUSED")
    }
    if state & STATE_SYSTEM_HOTTRACKED != 0 {
        states.push("STATE_SYSTEM_HOTTRACKED")
    }
    if state & STATE_SYSTEM_INDETERMINATE != 0 {
        states.push("STATE_SYSTEM_INDETERMINATE")
    }
    if state & STATE_SYSTEM_LINKED != 0 {
        states.push("STATE_SYSTEM_LINKED")
    }
    if state & STATE_SYSTEM_MARQUEED != 0 {
        states.push("STATE_SYSTEM_MARQUEED")
    }
    if state & STATE_SYSTEM_MIXED != 0 {
        states.push("STATE_SYSTEM_MIXED")
    }
    if state & STATE_SYSTEM_MOVEABLE != 0 {
        states.push("STATE_SYSTEM_MOVEABLE")
    }
    if state & STATE_SYSTEM_MULTISELECTABLE != 0 {
        states.push("STATE_SYSTEM_MULTISELECTABLE")
    }
    if state & STATE_SYSTEM_PROTECTED != 0 {
        states.push("STATE_SYSTEM_PROTECTED")
    }
    if state & STATE_SYSTEM_READONLY != 0 {
        states.push("STATE_SYSTEM_READONLY")
    }
    if state & STATE_SYSTEM_SELECTABLE != 0 {
        states.push("STATE_SYSTEM_SELECTABLE")
    }
    if state & STATE_SYSTEM_SELECTED != 0 {
        states.push("STATE_SYSTEM_SELECTED")
    }
    if state & STATE_SYSTEM_SELFVOICING != 0 {
        states.push("STATE_SYSTEM_SELFVOICING")
    }
    if state & STATE_SYSTEM_SIZEABLE != 0 {
        states.push("STATE_SYSTEM_SIZEABLE")
    }
    if state & STATE_SYSTEM_TRAVERSED != 0 {
        states.push("STATE_SYSTEM_TRAVERSED")
    }
    if state & STATE_SYSTEM_VALID != 0 {
        states.push("STATE_SYSTEM_VALID")
    }
    states.join(",")
}

pub fn role_to_name(role: u32) -> &'static str {
    match role {
        ROLE_SYSTEM_ALERT => "ROLE_SYSTEM_ALERT",
        ROLE_SYSTEM_ANIMATION => "ROLE_SYSTEM_ANIMATION",
        ROLE_SYSTEM_APPLICATION => "ROLE_SYSTEM_APPLICATION",
        ROLE_SYSTEM_BORDER => "ROLE_SYSTEM_BORDER",
        ROLE_SYSTEM_BUTTONDROPDOWN => "ROLE_SYSTEM_BUTTONDROPDOWN",
        ROLE_SYSTEM_BUTTONDROPDOWNGRID => "ROLE_SYSTEM_BUTTONDROPDOWNGRID",
        ROLE_SYSTEM_BUTTONMENU => "ROLE_SYSTEM_BUTTONMENU",
        ROLE_SYSTEM_CARET => "ROLE_SYSTEM_CARET",
        ROLE_SYSTEM_CELL => "ROLE_SYSTEM_CELL",
        ROLE_SYSTEM_CHARACTER => "ROLE_SYSTEM_CHARACTER",
        ROLE_SYSTEM_CHART => "ROLE_SYSTEM_CHART",
        ROLE_SYSTEM_CHECKBUTTON => "ROLE_SYSTEM_CHECKBUTTON",
        ROLE_SYSTEM_CLIENT => "ROLE_SYSTEM_CLIENT",
        ROLE_SYSTEM_CLOCK => "ROLE_SYSTEM_CLOCK",
        ROLE_SYSTEM_COLUMN => "ROLE_SYSTEM_COLUMN",
        ROLE_SYSTEM_COLUMNHEADER => "ROLE_SYSTEM_COLUMNHEADER",
        ROLE_SYSTEM_COMBOBOX => "ROLE_SYSTEM_COMBOBOX",
        ROLE_SYSTEM_CURSOR => "ROLE_SYSTEM_CURSOR",
        ROLE_SYSTEM_DIAGRAM => "ROLE_SYSTEM_DIAGRAM",
        ROLE_SYSTEM_DIAL => "ROLE_SYSTEM_DIAL",
        ROLE_SYSTEM_DIALOG => "ROLE_SYSTEM_DIALOG",
        ROLE_SYSTEM_DOCUMENT => "ROLE_SYSTEM_DOCUMENT",
        ROLE_SYSTEM_DROPLIST => "ROLE_SYSTEM_DROPLIST",
        ROLE_SYSTEM_EQUATION => "ROLE_SYSTEM_EQUATION",
        ROLE_SYSTEM_GRAPHIC => "ROLE_SYSTEM_GRAPHIC",
        ROLE_SYSTEM_GRIP => "ROLE_SYSTEM_GRIP",
        ROLE_SYSTEM_GROUPING => "ROLE_SYSTEM_GROUPING",
        ROLE_SYSTEM_HELPBALLOON => "ROLE_SYSTEM_HELPBALLOON",
        ROLE_SYSTEM_HOTKEYFIELD => "ROLE_SYSTEM_HOTKEYFIELD",
        ROLE_SYSTEM_INDICATOR => "ROLE_SYSTEM_INDICATOR",
        ROLE_SYSTEM_IPADDRESS => "ROLE_SYSTEM_IPADDRESS",
        ROLE_SYSTEM_LINK => "ROLE_SYSTEM_LINK",
        ROLE_SYSTEM_LIST => "ROLE_SYSTEM_LIST",
        ROLE_SYSTEM_LISTITEM => "ROLE_SYSTEM_LISTITEM",
        ROLE_SYSTEM_MENUBAR => "ROLE_SYSTEM_MENUBAR",
        ROLE_SYSTEM_MENUITEM => "ROLE_SYSTEM_MENUITEM",
        ROLE_SYSTEM_MENUPOPUP => "ROLE_SYSTEM_MENUPOPUP",
        ROLE_SYSTEM_OUTLINE => "ROLE_SYSTEM_OUTLINE",
        ROLE_SYSTEM_OUTLINEBUTTON => "ROLE_SYSTEM_OUTLINEBUTTON",
        ROLE_SYSTEM_OUTLINEITEM => "ROLE_SYSTEM_OUTLINEITEM",
        ROLE_SYSTEM_PAGETAB => "ROLE_SYSTEM_PAGETAB",
        ROLE_SYSTEM_PAGETABLIST => "ROLE_SYSTEM_PAGETABLIST",
        ROLE_SYSTEM_PANE => "ROLE_SYSTEM_PANE",
        ROLE_SYSTEM_PROGRESSBAR => "ROLE_SYSTEM_PROGRESSBAR",
        ROLE_SYSTEM_PROPERTYPAGE => "ROLE_SYSTEM_PROPERTYPAGE",
        ROLE_SYSTEM_PUSHBUTTON => "ROLE_SYSTEM_PUSHBUTTON",
        ROLE_SYSTEM_RADIOBUTTON => "ROLE_SYSTEM_RADIOBUTTON",
        ROLE_SYSTEM_ROW => "ROLE_SYSTEM_ROW",
        ROLE_SYSTEM_ROWHEADER => "ROLE_SYSTEM_ROWHEADER",
        ROLE_SYSTEM_SCROLLBAR => "ROLE_SYSTEM_SCROLLBAR",
        ROLE_SYSTEM_SEPARATOR => "ROLE_SYSTEM_SEPARATOR",
        ROLE_SYSTEM_SLIDER => "ROLE_SYSTEM_SLIDER",
        ROLE_SYSTEM_SOUND => "ROLE_SYSTEM_SOUND",
        ROLE_SYSTEM_SPINBUTTON => "ROLE_SYSTEM_SPINBUTTON",
        ROLE_SYSTEM_SPLITBUTTON => "ROLE_SYSTEM_SPLITBUTTON",
        ROLE_SYSTEM_STATICTEXT => "ROLE_SYSTEM_STATICTEXT",
        ROLE_SYSTEM_STATUSBAR => "ROLE_SYSTEM_STATUSBAR",
        ROLE_SYSTEM_TABLE => "ROLE_SYSTEM_TABLE",
        ROLE_SYSTEM_TEXT => "ROLE_SYSTEM_TEXT",
        ROLE_SYSTEM_TITLEBAR => "ROLE_SYSTEM_TITLEBAR",
        ROLE_SYSTEM_TOOLBAR => "ROLE_SYSTEM_TOOLBAR",
        ROLE_SYSTEM_TOOLTIP => "ROLE_SYSTEM_TOOLTIP",
        ROLE_SYSTEM_WHITESPACE => "ROLE_SYSTEM_WHITESPACE",
        ROLE_SYSTEM_WINDOW => "ROLE_SYSTEM_WINDOW",
        _ => "<Unknown>",
    }
}

pub fn event_to_name(event: u32) -> &'static str {
    match event {
        EVENT_AIA_END => "EVENT_AIA_END",
        EVENT_AIA_START => "EVENT_AIA_START",
        EVENT_CONSOLE_CARET => "EVENT_CONSOLE_CARET",
        EVENT_CONSOLE_END => "EVENT_CONSOLE_END",
        EVENT_CONSOLE_END_APPLICATION => "EVENT_CONSOLE_END_APPLICATION",
        EVENT_CONSOLE_LAYOUT => "EVENT_CONSOLE_LAYOUT",
        EVENT_CONSOLE_START_APPLICATION => "EVENT_CONSOLE_START_APPLICATION",
        EVENT_CONSOLE_UPDATE_REGION => "EVENT_CONSOLE_UPDATE_REGION",
        EVENT_CONSOLE_UPDATE_SCROLL => "EVENT_CONSOLE_UPDATE_SCROLL",
        EVENT_CONSOLE_UPDATE_SIMPLE => "EVENT_CONSOLE_UPDATE_SIMPLE",
        EVENT_OBJECT_ACCELERATORCHANGE => "EVENT_OBJECT_ACCELERATORCHANGE",
        EVENT_OBJECT_CLOAKED => "EVENT_OBJECT_CLOAKED",
        EVENT_OBJECT_CONTENTSCROLLED => "EVENT_OBJECT_CONTENTSCROLLED",
        EVENT_OBJECT_CREATE => "EVENT_OBJECT_CREATE",
        EVENT_OBJECT_DEFACTIONCHANGE => "EVENT_OBJECT_DEFACTIONCHANGE",
        EVENT_OBJECT_DESCRIPTIONCHANGE => "EVENT_OBJECT_DESCRIPTIONCHANGE",
        EVENT_OBJECT_DESTROY => "EVENT_OBJECT_DESTROY",
        EVENT_OBJECT_DRAGCANCEL => "EVENT_OBJECT_DRAGCANCEL",
        EVENT_OBJECT_DRAGCOMPLETE => "EVENT_OBJECT_DRAGCOMPLETE",
        EVENT_OBJECT_DRAGDROPPED => "EVENT_OBJECT_DRAGDROPPED",
        EVENT_OBJECT_DRAGENTER => "EVENT_OBJECT_DRAGENTER",
        EVENT_OBJECT_DRAGLEAVE => "EVENT_OBJECT_DRAGLEAVE",
        EVENT_OBJECT_DRAGSTART => "EVENT_OBJECT_DRAGSTART",
        EVENT_OBJECT_END => "EVENT_OBJECT_END",
        EVENT_OBJECT_FOCUS => "EVENT_OBJECT_FOCUS",
        EVENT_OBJECT_HELPCHANGE => "EVENT_OBJECT_HELPCHANGE",
        EVENT_OBJECT_HIDE => "EVENT_OBJECT_HIDE",
        EVENT_OBJECT_HOSTEDOBJECTSINVALIDATED => "EVENT_OBJECT_HOSTEDOBJECTSINVALIDATED",
        EVENT_OBJECT_IME_CHANGE => "EVENT_OBJECT_IME_CHANGE",
        EVENT_OBJECT_IME_HIDE => "EVENT_OBJECT_IME_HIDE",
        EVENT_OBJECT_IME_SHOW => "EVENT_OBJECT_IME_SHOW",
        EVENT_OBJECT_INVOKED => "EVENT_OBJECT_INVOKED",
        EVENT_OBJECT_LIVEREGIONCHANGED => "EVENT_OBJECT_LIVEREGIONCHANGED",
        EVENT_OBJECT_LOCATIONCHANGE => "EVENT_OBJECT_LOCATIONCHANGE",
        EVENT_OBJECT_NAMECHANGE => "EVENT_OBJECT_NAMECHANGE",
        EVENT_OBJECT_PARENTCHANGE => "EVENT_OBJECT_PARENTCHANGE",
        EVENT_OBJECT_REORDER => "EVENT_OBJECT_REORDER",
        EVENT_OBJECT_SELECTION => "EVENT_OBJECT_SELECTION",
        EVENT_OBJECT_SELECTIONADD => "EVENT_OBJECT_SELECTIONADD",
        EVENT_OBJECT_SELECTIONREMOVE => "EVENT_OBJECT_SELECTIONREMOVE",
        EVENT_OBJECT_SELECTIONWITHIN => "EVENT_OBJECT_SELECTIONWITHIN",
        EVENT_OBJECT_SHOW => "EVENT_OBJECT_SHOW",
        EVENT_OBJECT_STATECHANGE => "EVENT_OBJECT_STATECHANGE",
        EVENT_OBJECT_TEXTEDIT_CONVERSIONTARGETCHANGED => {
            "EVENT_OBJECT_TEXTEDIT_CONVERSIONTARGETCHANGED"
        }
        EVENT_OBJECT_TEXTSELECTIONCHANGED => "EVENT_OBJECT_TEXTSELECTIONCHANGED",
        EVENT_OBJECT_UNCLOAKED => "EVENT_OBJECT_UNCLOAKED",
        EVENT_OBJECT_VALUECHANGE => "EVENT_OBJECT_VALUECHANGE",
        EVENT_OEM_DEFINED_END => "EVENT_OEM_DEFINED_END",
        EVENT_OEM_DEFINED_START => "EVENT_OEM_DEFINED_START",
        EVENT_SYSTEM_ALERT => "EVENT_SYSTEM_ALERT",
        EVENT_SYSTEM_ARRANGMENTPREVIEW => "EVENT_SYSTEM_ARRANGMENTPREVIEW",
        EVENT_SYSTEM_CAPTUREEND => "EVENT_SYSTEM_CAPTUREEND",
        EVENT_SYSTEM_CAPTURESTART => "EVENT_SYSTEM_CAPTURESTART",
        EVENT_SYSTEM_CONTEXTHELPEND => "EVENT_SYSTEM_CONTEXTHELPEND",
        EVENT_SYSTEM_CONTEXTHELPSTART => "EVENT_SYSTEM_CONTEXTHELPSTART",
        EVENT_SYSTEM_DESKTOPSWITCH => "EVENT_SYSTEM_DESKTOPSWITCH",
        EVENT_SYSTEM_DIALOGEND => "EVENT_SYSTEM_DIALOGEND",
        EVENT_SYSTEM_DIALOGSTART => "EVENT_SYSTEM_DIALOGSTART",
        EVENT_SYSTEM_DRAGDROPEND => "EVENT_SYSTEM_DRAGDROPEND",
        EVENT_SYSTEM_DRAGDROPSTART => "EVENT_SYSTEM_DRAGDROPSTART",
        EVENT_SYSTEM_END => "EVENT_SYSTEM_END",
        EVENT_SYSTEM_FOREGROUND => "EVENT_SYSTEM_FOREGROUND",
        EVENT_SYSTEM_IME_KEY_NOTIFICATION => "EVENT_SYSTEM_IME_KEY_NOTIFICATION",
        EVENT_SYSTEM_MENUEND => "EVENT_SYSTEM_MENUEND",
        EVENT_SYSTEM_MENUPOPUPEND => "EVENT_SYSTEM_MENUPOPUPEND",
        EVENT_SYSTEM_MENUPOPUPSTART => "EVENT_SYSTEM_MENUPOPUPSTART",
        EVENT_SYSTEM_MENUSTART => "EVENT_SYSTEM_MENUSTART",
        EVENT_SYSTEM_MINIMIZEEND => "EVENT_SYSTEM_MINIMIZEEND",
        EVENT_SYSTEM_MINIMIZESTART => "EVENT_SYSTEM_MINIMIZESTART",
        EVENT_SYSTEM_MOVESIZEEND => "EVENT_SYSTEM_MOVESIZEEND",
        EVENT_SYSTEM_MOVESIZESTART => "EVENT_SYSTEM_MOVESIZESTART",
        EVENT_SYSTEM_SCROLLINGEND => "EVENT_SYSTEM_SCROLLINGEND",
        EVENT_SYSTEM_SCROLLINGSTART => "EVENT_SYSTEM_SCROLLINGSTART",
        EVENT_SYSTEM_SOUND => "EVENT_SYSTEM_SOUND",
        EVENT_SYSTEM_SWITCHEND => "EVENT_SYSTEM_SWITCHEND",
        EVENT_SYSTEM_SWITCHER_APPDROPPED => "EVENT_SYSTEM_SWITCHER_APPDROPPED",
        EVENT_SYSTEM_SWITCHER_APPGRABBED => "EVENT_SYSTEM_SWITCHER_APPGRABBED",
        EVENT_SYSTEM_SWITCHER_APPOVERTARGET => "EVENT_SYSTEM_SWITCHER_APPOVERTARGET",
        EVENT_SYSTEM_SWITCHER_CANCELLED => "EVENT_SYSTEM_SWITCHER_CANCELLED",
        EVENT_SYSTEM_SWITCHSTART => "EVENT_SYSTEM_SWITCHSTART",
        EVENT_UIA_EVENTID_END => "EVENT_UIA_EVENTID_END",
        EVENT_UIA_EVENTID_START => "EVENT_UIA_EVENTID_START",
        EVENT_UIA_PROPID_END => "EVENT_UIA_PROPID_END",
        EVENT_UIA_PROPID_START => "EVENT_UIA_PROPID_START",
        _ => "<Unknown>",
    }
}

#[cfg(test)]
mod tests {
    use windows::core::PCWSTR;

    use super::*;

    #[test]
    fn listen_keyboard_events() {
        let hwnd = init_window().unwrap();
        let dev = RAWINPUTDEVICE {
            usUsagePage: 0x01,
            usUsage: 0x06,
            dwFlags: RIDEV_INPUTSINK,
            hwndTarget: hwnd,
        };

        unsafe {
            RegisterRawInputDevices(&[dev], std::mem::size_of::<RAWINPUTDEVICE>() as u32).unwrap();

            let mut message = MSG::default();
            println!("Starting message loop");
            while GetMessageA(&mut message, hwnd, 0, 0).as_bool() {
                DispatchMessageA(&message);
            }

            DestroyWindow(hwnd).unwrap();
        }
    }

    fn init_window() -> Result<HWND, windows::core::Error> {
        let class_name = widestring::U16CString::from_str("bruh")
            .map_err(|_| windows::core::Error::OK)?;
        let class_name_ptr = class_name.as_ptr();
        let class_name_pcwstr = PCWSTR(class_name_ptr);

        let hinstance = unsafe { windows::Win32::System::LibraryLoader::GetModuleHandleW(None)? };

        let mut wnd = WNDCLASSEXW::default();
        wnd.cbSize = std::mem::size_of::<WNDCLASSEXW>() as u32;
        wnd.lpfnWndProc = Some(window_proc);
        wnd.hInstance = hinstance.into();
        wnd.lpszClassName = class_name_pcwstr;

        let reg = unsafe { RegisterClassExW(&wnd) };

        let window = unsafe {
            CreateWindowExW(
                WINDOW_EX_STYLE::default(),
                PCWSTR::from(class_name_pcwstr),
                None,
                WINDOW_STYLE(0),
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                None,
                None,
                hinstance,
                None,
            )
        };

        unsafe { GetLastError()? };
        if window.0 == 0 {
            return Err(windows::core::Error::from_win32());
        }

        Ok(window)
    }

    unsafe extern "system" fn window_proc(
        hwnd: HWND,
        msg: u32,
        w_param: WPARAM,
        l_param: LPARAM,
    ) -> LRESULT {
        match msg {
            WM_INPUT => {
                let mut size = 0;
                let result = GetRawInputData(
                    HRAWINPUT(l_param.0),
                    RID_INPUT,
                    None,
                    &mut size,
                    std::mem::size_of::<RAWINPUTHEADER>() as u32,
                );
                assert_eq!(result as i32, 0);

                let mut data = vec![0u8; size as usize];
                let result = GetRawInputData(
                    HRAWINPUT(l_param.0),
                    RID_INPUT,
                    Some(data.as_mut_ptr() as *mut std::ffi::c_void),
                    &mut size,
                    std::mem::size_of::<RAWINPUTHEADER>() as u32,
                );
                assert_eq!(result as i32, size as i32);

                let input = &*(data.as_ptr() as *const RAWINPUT);
                if (*input).header.dwType == windows::Win32::UI::Input::RIM_TYPEKEYBOARD.0
                    && (*input).data.keyboard.Message == WM_KEYDOWN as u32
                {
                    let device = (*input).header.hDevice;
                    let key = (*input).data.keyboard.VKey as u8 as char;
                    println!("[{:?}]: {}", device, key);
                }

                LRESULT(0)
            }
            _ => DefWindowProcW(hwnd, msg, w_param, l_param),
        }
    }
}
