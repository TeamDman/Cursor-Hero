- restart program too

- show tools which have conflicting keys in red
- tool to focus the game window and maximize it on the monitors with dpad

- new tool tool
  - bind: type "plug"
  - bind: press tab
  - bind: rename symbol refactor hotkey (Shift+F6)
  - bind: voice2text
  - bind: enter
- new file tool
  - bind: left click (on folder -> new file)
  - bind: voice prompt with instructions on file name preference, include txt of tree output next to the cursor in the vscode file explorer
  - bind: hit enter


heuristics!

add a binding to the voice tool for hitting the enter button.Add a binding.for the Ctrl-S hotkey.add a hotkey to thefor inserting a new line.Add a camera tool to own the hotkey for fire.following the character.- tool selection wheel show on rstick click, left stick to pick item in the wheel - is the toolbelt a tool?
- teleport tool - rstick in maps left stick to full screen coords. Full left stick places cursor as far left, across all screens, as possible
- find way to overlay on top of other windows without drawing the screens
- text buffer - a change-listenable history-queryable place to store text
- d-pad keyboard navigation tool , arrow keys, wasd, hjkl
- clipboard tool - keybind to push text buffer to system clipboard, keybind to pull from system clipboard to text buffer, keybind to send ctrl+v input
- voice2text tool - hold to talk to transcribe your voice into the text buffer
- annotation tool - keybind to append to a file (timestamp, cursor pos, element under cursor persistent id, text buffer)
- cursor tool - mapping to shift to enable shift-clicking, mapping control to enable control clicking.Combinable with shift.
- summarize tool - summarize the clicked element. north/south: summary length adjust. east/west: level of detail adjust
- dump tool - poop emoji icon, dumps context of the current timestep to the text buffer. keybind to adjust detail
- zoom tool - right trigger to control zoom
- type tool - send keyboard inputs according to the text buffer
- d-pad quick toggle tool system
- timestamp marking tool - save to file with note
- describe hovered element tool
- llm inference for arbitrary string payloads
- rebind hotkey tool that analyzes project for keybinding defaults in code and edits the code in place to change the default
- todo tool that reads top 3 todo.md entries and top 3 git log into LLM to suggest the next thing to work on
- TTS (glados)
- more real time voice 2 text
- indicator out of bounds when OBS is not recording / status lights for OBS
- detect periods of waiting and replace progress spinners / cargo build logs / chatgpt still typing a response with a video from my watch later, my move in a chess game, a chess puzzle, an email -- predict the length of the time spinning to pick a thing to that will take that amount of time, 
- point at program and open the source code - mapping from program to disk location
- automate bevy cloning and dependency update to point at local clones of the repos

# use case

select multiple files from vscode, tool read the contents of those files and format in markdown code blocks including file path 

```main.rs
pub mod other;

fn main() {
    println!("hello world");
}
```

```other.rs
pub fn other() {
    println!("other");
}
```

to be combined in non-conflicting keys

- left click
- enter
- voice2text
- copy to clipboard
- append to selection
- remove from selection

# use case - slider hub

volume slider hub

Master slider
Audio mixer slider per app
App slider - game master volume slider, music/sfx slider; youtube video slider, mpv volume slider
control all from single location - can use browser extension to control youtube without bringing the video to the front

