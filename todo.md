- sfx when service comes online
- restart memory for voice2text api key


- hot mic support for back and forth talking while coding
  - python server needs api with key set on launch
    - key not present -> "who are you, how did you get into my house?"
- agent restart memory
- better prompt history
- vscode buffer observations
- environment restart observations 

- Onboarder startup button
- voice2text startup button
- Screen pause toggle button
- add "thinking" bubble above agent head when request in flight

- customizable launch params for the wt invocation to launch ollama and stuff in case people have different WSL distros than ubuntu
  - restart memory, edit in game
- ollama server documentation button to open the github
- character selection room, displaying all the variants of the character using a closet that you scroll through the hangers that describe the situation in which each sprite is used, exploded view to show all variants / the sprite sheet
- run animation

- performance profiling (so I can close the browser tab I have open for the rust metrics lib)
- reduce background CPU usage from 22% 😱 https://learn.microsoft.com/en-us/windows-hardware/test/wpt/introduction-to-wpr

- switcher wheel to collect all switcher tools for predictable navigation
  - gamepad rumble on hover
  - sound effect on hover tool
- fullscreen tool binding f11 to fullscreen toggle
- set clippy rule to flag `unwrap` and `expect` usage
- post to openai discord use-cases channel https://discord.com/channels/974519864045756446/1155775326253756456
  - tag @abdubs (Alex)
- window switcher tool; window wheel -> focus, teleport
- Github releases in world
- File browser in world
- Voice input tool
  - Push to talk
  - Push to toggle talk
  - Insert period
  - Insert space
  - Hit enter

- interpreter agent that acts as "middleware" that will process user speech transcription to clean it up typos and other errors that can decrease model performance before it gets included in main agent observations
- track recently played sounds and empower the user to adjust the volume of individual sounds and future sounds of the same class
- sprint tool scales scroll speed

- couple click tool with window/screen mouse position tool
- make overlay logic stop when inspection tool disabled

- add version number to binary as part of build

- show tools which have conflicting keys in red
- tool to focus the game window and maximize it on the monitors with dpad

- jump tool - aimlock with right stick to jump to targets at varying distances

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

