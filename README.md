# Cursor Hero

[![Visitors](https://api.visitorbadge.io/api/visitors?path=github.com%2FTeamDman%2FCursor-Hero&countColor=%23263759)](https://visitorbadge.io/status?path=github.com%2FTeamDman%2FCursor-Hero)

## Video demo

[![yt thumbnail](https://img.youtube.com/vi/t1PYks0UTL8/0.jpg)](https://youtu.be/t1PYks0UTL8)

## Regarding DualShock controllers

Try [DS4Windows](https://ds4windows.dev/).

## Integrations

- [TeamDman/voice2text: Local speech recognition](https://github.com/teamdman/voice2text)
- [ollama/ollama: Local LLM inference](https://github.com/ollama/ollama)
- [R2D2FISH/glados-tts: GLaDOS speech synthesis](https://github.com/TeamDman/glados-tts.git)

## Diagnosing problems

```pwsh
$env:RUST_BACKTRACE="1"
```

## Project description

Cursor Hero is a thing built using Rust and the Bevy game engine.

Cursor Hero, when launched, presents you with a character that has a pointer. You can move the character and pointer independently using the mouse and keyboard, or using both sticks on a gamepad.

In the game world, you can see a scale representation of your monitors. I have three monitors, so when I launch the game I see the left, center, and right monitor, usually containing VSCode, Discord, and the Cursor Hero window itself.

Cursor Hero uses the `windows` crate to integrate tightly with the Windows operating system. It reads and writes the host cursor position, with integrations with the game to do things like position the host mouse "cursor" over the in-game "pointer", or updating the cursor so that it matches the game world position of the pointer, letting the player interact with the OS using the game representation of the screens.

This includes integrating with Windows UI automation to grab the locations of UI elements, copying the texture of the screen onto bricks in the game world that are physics objects.

UI information is attached to the bricks, using `bevy-egui` to render UI elements with screen coordinates updated to correspond with world-coordinates of the bricks.

Using `egui`, the tree hierarchy of the UI is shown for the bricks created by clicking on UI elements in the screen texture. The tree could potentially grow to something like inspect.exe, where the properties of the UI element can be explored. Currently, the tree folding and selectable labels is working, but the properties panel on the right is not yet implemented.

In addition to showing the host environment, a game environment is also constructed. The game environment contains a virtual agent and a mimicry of the host OS. The mimicry extends to include a copy of the desktop background, a taskbar that pulls the colour from the OS preferences, and a start menu button that toggles visibility of an empty panel.

The virtual agent walks in circles. It has an observation buffer which other systems can publish events for something happening, and the buffer will conditionally append such events which are used in the construction of prompts for a local LLM ran using `ollama`.

In response to player chat messages, or in response to a period of inactivity, the agent will prompt the LLM in a chat format. With the response, it publishes its own chat message, and it invokes the `glados-tts` to speak the response to the player.

This agent behaviour, combined with integration to `voice2text`, allows the human to converse to the agent entirely locally using `whisper-x` to speak with the addition of a push-to-talk button and a toggle-active-listening button.

Given that the host cursor is being updated by the game pointer when in gamepad input mode, the game takes care to listen for raw mouse input events to determine when the player is trying to use mouse and keyboard input mode, avoiding holding the pointer captive since traditional input detection is not suitable. After all, if the pointer is programmatically controlled by the game, "mouse moved events" become less reliable when aiming for physical movement detection.

Again using the `windows` crate, the game is able to identify running processes and programmatically detect the exe path and extract the icons of the process from it.
Opportunity exists to detect dynamic icons from process windows as well.

The game has a radial menu for enabling/disabling "tools", some of which start disabled and upon enabling will clear and repopulate the tools in the toolbelt with a different loadout.

The tool system has been used to show a radial menu of programs in the taskbar, using UI automation to grab the texture at the rect of each item to be displayed radially.

There is a tool loadout that lets you snap the game window to any corner or fullscreen of any monitors detected.

The default tool loadout starts you in click mode, letting you move your character and its pointer, with the ability to send click events to the OS, and to send click events to game objects using the ECS fundamentals. 

The game has its own model system for the UI hierarchy, with support for detecting the details of running VSCode windows, including tabs on the left, open tab, contents of the explorer tab if open, editor groups open and involved tabs and file contents, current cursor line and position displayed in the bottom corner.

## Where do we go from here

I am continuously exploring many different trajectories for this project.

See [`./todo.md`](./todo.md) for brainstorming on potential features to add.

See [`./research_notes.md`](./research_notes.md) for links to internet resources that could be helpful.
