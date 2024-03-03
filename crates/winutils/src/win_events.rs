use bevy::math::IRect;
use bevy::math::IVec2;
use crossbeam_channel::Receiver;
use crossbeam_channel::Sender;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;
use windows::core::PCWSTR;
use windows::Win32::Devices::HumanInterfaceDevice::HID_USAGE_GENERIC_KEYBOARD;
use windows::Win32::Devices::HumanInterfaceDevice::HID_USAGE_GENERIC_MOUSE;
use windows::Win32::Devices::HumanInterfaceDevice::HID_USAGE_PAGE_GENERIC;
use windows::Win32::Foundation::HWND;
use windows::Win32::Foundation::*;
use windows::Win32::System::Variant::VARIANT;
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
use windows::Win32::UI::Input::RIM_TYPEKEYBOARD;
use windows::Win32::UI::Input::RIM_TYPEMOUSE;
use windows::Win32::UI::WindowsAndMessaging::EVENT_OBJECT_CREATE;
use windows::Win32::UI::WindowsAndMessaging::EVENT_OBJECT_DESTROY;
use windows::Win32::UI::WindowsAndMessaging::EVENT_OBJECT_HIDE;
use windows::Win32::UI::WindowsAndMessaging::EVENT_OBJECT_LIVEREGIONCHANGED;
use windows::Win32::UI::WindowsAndMessaging::EVENT_OBJECT_STATECHANGE;
use windows::Win32::UI::WindowsAndMessaging::EVENT_OBJECT_VALUECHANGE;
use windows::Win32::UI::WindowsAndMessaging::*;

#[derive(Debug)]
pub enum ProcMessage {
    MouseMoved(IVec2),
    KeyDown(char),
    Event {
        event_name: String,
        name: Option<String>,
        role: Option<String>,
        state: Option<String>,
        bounds: Option<IRect>,
    },
}

static SENDERS: Lazy<Mutex<HashMap<isize, Sender<ProcMessage>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));
static HWND_TO_HOOK: Lazy<Mutex<HashMap<isize, isize>>> = Lazy::new(|| Mutex::new(HashMap::new()));
fn store_sender_for_identifier(
    identifier: isize,
    sender: Sender<ProcMessage>,
) -> Result<(), &'static str> {
    let mut senders = SENDERS.lock().map_err(|_| "Failed to lock SENDERS map")?;
    senders.insert(identifier, sender);
    Ok(())
}
fn get_sender_for_window(hwnd: HWND) -> Option<Sender<ProcMessage>> {
    let senders = SENDERS.lock().ok()?;
    senders.get(&hwnd.0).cloned()
}
fn get_sender_for_hook(hook: HWINEVENTHOOK) -> Option<Sender<ProcMessage>> {
    let senders = SENDERS.lock().ok()?;
    senders.get(&hook.0).cloned()
}
fn store_sender(
    hwnd: HWND,
    hook: HWINEVENTHOOK,
    sender: Sender<ProcMessage>,
) -> Result<(), &'static str> {
    store_sender_for_identifier(hwnd.0, sender.clone())?;
    store_sender_for_identifier(hook.0, sender)?;
    Ok(())
}
fn drop_senders_for_window(hwnd: HWND) -> Result<(), &'static str> {
    let mut senders = SENDERS.lock().map_err(|_| "Failed to lock SENDERS map")?;

    // Remove the sender for the window
    senders.remove(&hwnd.0);

    // Get the hook for the window
    let mut hook_map = HWND_TO_HOOK
        .lock()
        .map_err(|_| "Failed to lock HWND_TO_HOOK map")?;
    if let Some(hook) = hook_map.remove(&hwnd.0) {
        // Drop the sender for the hook
        senders.remove(&hook);
    }

    Ok(())
}

pub fn create_os_event_listener() -> Result<Receiver<ProcMessage>, windows::core::Error> {
    let (tx, rx) = crossbeam_channel::unbounded();
    std::thread::spawn(move || match create_window_and_do_message_loop(tx) {
        Ok(()) => {
            unreachable!("create_window_and_do_message_loop should never return Ok");
        }
        Err(e) => {
            eprintln!("Error in os_event_listener_thread: {:?}", e);
        }
    });
    Ok(rx)
}

fn create_window_and_do_message_loop(tx: Sender<ProcMessage>) -> Result<(), windows::core::Error> {
    let hwnd = init_window()?;
    // attach_tx_pointer(hwnd, tx.clone());
    let hook = register_os_event_listener()?;
    store_sender(hwnd, hook, tx).map_err(|e| windows::core::Error::new(E_FAIL, e.into()))?;

    register_interest_in_mouse_with_os(hwnd.0)?;
    register_interest_in_keyboard_with_os(hwnd.0)?;
    unsafe {
        let mut message = MSG::default();
        println!("Starting message loop");
        while GetMessageA(&mut message, hwnd, 0, 0).as_bool() {
            TranslateMessage(&message);
            DispatchMessageA(&message);
            // println!("ballin, got {:?}", message);
        }
        DestroyWindow(hwnd)?;
    }
    Ok(())
}

fn register_os_event_listener() -> Result<HWINEVENTHOOK, windows::core::Error> {
    unsafe {
        match SetWinEventHook(
            EVENT_MIN, // or specific event codes
            EVENT_MAX, // or specific event codes
            None,      // hmodWinEventProc
            Some(os_event_procedure),
            0, // idProcess
            0, // idThread
            WINEVENT_OUTOFCONTEXT,
        ) {
            HWINEVENTHOOK(0) => Err(windows::core::Error::new(
                E_FAIL,
                "Failed to register interest in all events".into(),
            )),
            x => Ok(x),
        }
    }
}

fn register_interest_in_mouse_with_os(hwnd: isize) -> Result<(), windows::core::Error> {
    unsafe {
        let device = RAWINPUTDEVICE {
            usUsagePage: HID_USAGE_PAGE_GENERIC,
            usUsage: HID_USAGE_GENERIC_MOUSE,
            dwFlags: RIDEV_INPUTSINK,
            hwndTarget: HWND(hwnd),
        };
        RegisterRawInputDevices(&mut [device], std::mem::size_of::<RAWINPUTDEVICE>() as u32)
    }
}

fn register_interest_in_keyboard_with_os(hwnd: isize) -> Result<(), windows::core::Error> {
    unsafe {
        let device = RAWINPUTDEVICE {
            usUsagePage: HID_USAGE_PAGE_GENERIC,
            usUsage: HID_USAGE_GENERIC_KEYBOARD,
            dwFlags: RIDEV_INPUTSINK,
            hwndTarget: HWND(hwnd),
        };
        RegisterRawInputDevices(&[device], std::mem::size_of::<RAWINPUTDEVICE>() as u32)
    }
}

fn init_window() -> Result<HWND, windows::core::Error> {
    let class_name =
        widestring::U16CString::from_str("bruh").map_err(|_| windows::core::Error::OK)?;
    let class_name_ptr = class_name.as_ptr();
    let class_name_pcwstr = PCWSTR(class_name_ptr);

    let hinstance = unsafe { windows::Win32::System::LibraryLoader::GetModuleHandleW(None)? };

    let mut wnd = WNDCLASSEXW::default();
    wnd.cbSize = std::mem::size_of::<WNDCLASSEXW>() as u32;
    wnd.lpfnWndProc = Some(window_message_procedure);
    wnd.hInstance = hinstance.into();
    wnd.lpszClassName = class_name_pcwstr;

    let _reg = unsafe { RegisterClassExW(&wnd) };

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

unsafe extern "system" fn window_message_procedure(
    hwnd: HWND,
    msg: u32,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    let next = || DefWindowProcW(hwnd, msg, w_param, l_param);

    // let tx = match get_tx_pointer(hwnd) {
    //     Some(tx) => tx,
    //     None => return next(),
    // };

    let tx = match get_sender_for_window(hwnd) {
        Some(tx) => tx,
        None => return next(),
    };

    match msg {
        WM_INPUT => {
            let mut size = 0;
            let result = GetRawInputData(
                HRAWINPUT(l_param.0),
                RID_INPUT,
                None, // Pointer to data is null, requesting size only
                &mut size,
                std::mem::size_of::<RAWINPUTHEADER>() as u32,
            );
            assert_eq!(result as i32, 0);

            let mut data = vec![0u8; size as usize];
            let recv_size = GetRawInputData(
                HRAWINPUT(l_param.0),
                RID_INPUT,
                Some(data.as_mut_ptr() as *mut std::ffi::c_void),
                &mut size,
                std::mem::size_of::<RAWINPUTHEADER>() as u32,
            );
            assert_eq!(recv_size as i32, size as i32);
            let input = &*(data.as_ptr() as *const RAWINPUT);

            if (*input).header.dwType == RIM_TYPEKEYBOARD.0
                && (*input).data.keyboard.Message == WM_KEYDOWN as u32
            {
                let key = (*input).data.keyboard.VKey as u8 as char;
                if let Err(e) = tx.send(ProcMessage::KeyDown(key)) {
                    eprintln!("Error sending keyboard message: {:?}", e);
                    return LRESULT(0);
                }
            }

            if (*input).header.dwType == RIM_TYPEMOUSE.0 {
                let mouse_data = (*input).data.mouse;
                let x = mouse_data.lLastX;
                let y = mouse_data.lLastY;
                if let Err(e) = tx.send(ProcMessage::MouseMoved(IVec2::new(x, y))) {
                    eprintln!("Error sending mouse message: {:?}", e);
                    return LRESULT(0);
                }
            }

            LRESULT(0)
        }
        WM_DESTROY => {
            // detach_tx_pointer_and_drop(hwnd);
            drop_senders_for_window(hwnd).unwrap();
            PostQuitMessage(0);
            LRESULT(0)
        }
        _ => next(),
    }
}

unsafe extern "system" fn os_event_procedure(
    hook: HWINEVENTHOOK,
    event: u32,
    hwnd: HWND,
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
    // if event < 1000 {
    //     return;
    // }

    let event_name = event_to_name(event).to_string();
    // println!(
    //     "Hook: {:?}, Event: {:?} ({}), HWND: {:?}, idObject: {:?}, idChild: {:?}",
    //     hook,
    //     event,
    //     event_name,
    //     hwnd,
    //     object_id,
    //     child_id
    // );

    // println!("Attempting getting tx from window hwnd: {:?}", hwnd);
    let tx = match get_sender_for_hook(hook) {
        Some(tx) => {
            // println!("Got tx from hook {:?}", hook);
            tx
        },
        None => {
            eprintln!("No tx found for hook {:?}", hook);
            return;
        }
    };

    if object_id != OBJID_CLIENT.0 {
        return;
    }
    // println!("happy path");
    // if (event == EVENT_OBJECT_SELECTIONADD || event == EVENT_OBJECT_STATECHANGE)
    //     && object_id == OBJID_CLIENT.0 {}
    // Here you get the name and state of the element that triggered the event.
    // Implement the logic to retrieve the name and state using the AccessibleObjectFromEvent function.
    let mut acc_ptr: Option<IAccessible> = None;
    let mut elem = VARIANT::default();

    // println!("Getting accessible object");
    let lookup = AccessibleObjectFromEvent(
        hwnd,
        object_id as u32,
        child_id as u32,
        &mut acc_ptr,
        &mut elem,
    );
    if lookup.is_err() {
        // eprintln!("Error getting accessible object: {:?}", lookup);
        return;
    }
    let acc = match acc_ptr {
        Some(acc) => acc,
        None => {
            // eprintln!("Error getting accessible object");
            return;
        }
    };

    // println!("Getting name");
    let name = acc.get_accName(elem.clone()).map(|x| x.to_string()).ok();

    // println!("Getting role");
    let role = acc
        .get_accRole(elem.clone())
        .and_then(|variant| variant_to_int(&variant))
        .map(|i| role_to_name(i as u32).to_string())
        .ok();

    // println!("Getting state");
    let state = acc
        .get_accState(elem.clone())
        .and_then(|variant| variant_to_int(&variant))
        .map(|i| state_to_string(i as u32))
        .ok();

    let mut pxleft = 0;
    let mut pytop = 0;
    let mut pcxwidth = 0;
    let mut pcyheight = 0;

    // println!("Getting location");
    let bounds = match acc.accLocation(&mut pxleft, &mut pytop, &mut pcxwidth, &mut pcyheight, elem)
    {
        Ok(()) => Some(IRect::from_corners(
            IVec2::new(pxleft, pytop),
            IVec2::new(pxleft + pcxwidth, pytop + pcyheight),
        )),
        Err(_) => None,
    };

    // println!("Building msg");
    let msg = ProcMessage::Event {
        event_name,
        name,
        role,
        state,
        bounds,
    };

    // println!("Sending event message {:?}", msg);
    if let Err(e) = tx.send(msg) {
        eprintln!("Error sending event message: {:?}", e);
    } else {
        // println!("Sent event message :D");
    }
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

// fn decimal_to_string(decimal: DECIMAL) -> Result<String, windows::core::Error> {
//     unsafe { VarBstrFromDec(&decimal, 0, 0).map(|bstr| bstr.to_string()) }
// }

// fn variant_to_string(variant: &VARIANT) -> Result<String, windows::core::Error> {
//     let var_type = unsafe { variant.Anonymous.Anonymous.vt };

//     match var_type {
//         VT_BSTR => {
//             // Extract BSTR and convert to String
//             let bstr = unsafe { &variant.Anonymous.Anonymous.Anonymous.bstrVal };
//             Ok(bstr.to_string())
//         }
//         VT_I4 => {
//             // Extract 32-bit integer and convert to String
//             let int_val = unsafe { variant.Anonymous.Anonymous.Anonymous.lVal };
//             Ok(int_val.to_string())
//         }
//         // Add more cases as needed for other VARTYPEs you expect to handle
//         _ => Err(windows::core::Error::new(
//             windows::Win32::Foundation::E_FAIL,
//             "Unsupported VARIANT type".into(),
//         )),
//     }
// }

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
    use super::*;

    #[test]
    fn listen_all() -> Result<(), windows::core::Error> {
        let rx = create_os_event_listener()?;
        while let Ok(msg) = rx.recv() {
            println!("Received message: {:?}", msg);
        }
        Ok(())
    }

    #[test]
    fn listen_events() -> Result<(), windows::core::Error> {
        let rx = create_os_event_listener()?;
        while let Ok(msg) = rx.recv() {
            if !matches!(msg, ProcMessage::Event { .. }) {
                continue;
            }
            println!("Received message: {:?}", msg);
        }
        Ok(())
    }

    #[test]
    fn listen_mouse() -> Result<(), windows::core::Error> {
        let rx = create_os_event_listener()?;
        while let Ok(msg) = rx.recv() {
            if !matches!(msg, ProcMessage::MouseMoved { .. }) {
                continue;
            }
            println!("Received message: {:?}", msg);
        }
        Ok(())
    }

    #[test]
    fn listen_keyboard() -> Result<(), windows::core::Error> {
        let rx = create_os_event_listener()?;
        while let Ok(msg) = rx.recv() {
            if !matches!(msg, ProcMessage::KeyDown { .. }) {
                continue;
            }
            println!("Received message: {:?}", msg);
        }
        Ok(())
    }
}
