Data:

- UI Automation tree
    - DrillId gives bounding box of apps and sometimes inner elements
- Some other windows API for getting window size and location and decoration thicknesses
- Statistical methods for specifying element locations

Current state of affairs:

- Egui working to have tree of elements with properties panel
- half baked automation using a scratch pad to get element drill ids and bounding boxes

Strengths:

- Obvious view into the tree representation of the UI elements
- Code generation helps accelerate an otherwise manual process

Weaknesses:

- egui is out of band with the rest of the project being in Bevyland
- Scratch pad automation still requires intervention step to integrate

Opportunities:

- Element studio to build app structures
  - Consider: core essence of app; ignore extraneous features
  - example: explorer.exe
    - overall size of the window
    - address bar bounds and content
    - for each item:
      - name
      - kind (file, folder)
      - icon
  - for each element:
    - action point
    - drill id - optional

- mouse and keyboard
  - hover over element to use UIAutomation focus to get bounds and drill id
  - scroll wheel to move focus up and down the tree
    - always go max depth
    - max depth config: scroll back decreases
    - reset max depth on hover changed
    - don't zoom camera when zooming hierarchy
- keyboard
  - ???
- gamepad
  - dpad up/down for scroll?

Threats:

- abandoning egui success for a bespoke bevy solution 
  - integrate better with egui
- egui being out of band means desync between agent and human experience

---

currently: focus tool switches through three focus modes

CameraFollowsCharacter,
CameraWanders,
CharacterWanders,

however, this is an abstraction of some possible states

there is the entity the movement tool moves in response to user inputs
there is the entity the camera is following

Both the camera and movement targets are optional.
The system can be in a state where the player moves something without moving the camera.
If the user is to manipulate egui, the user is moving the pointer in screen space instead of thinking about moving it in game space.

In addition to the camera and the character, the movement tool will have the cursor as a target.

There needs to be a binding for attaching and detaching the camera.
There needs to be a binding for switching the movement target.
Detaching the camera should switch the movement target to the camera.

Q: What is the point of the character?
A: The character is a social representation of the user in the game world.