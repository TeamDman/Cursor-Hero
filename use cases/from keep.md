Suggest rename
- get llm to suggest multiple new names for a code symbol
- give context to llm: recent voice and rename activity
- display suggestions as radial menu
- picking a suggestion runs the rename hotkey, types it, hits enter



Make a music mix by picking tracks out of hour long mixes downloaded from YouTube. Use chapter splits and timestamp notices from notes to identify song boundaries


View two peoples monitor sets on the same world. Click and drag a file between computers - doorway maybe to make it clear when the transfer will start;passing between the boundary

Permission zones, read (screen, files, nothing), write (send files to downloads or other configured locations, add torrents to their client and only see the torrents you added) 
Permission of read/write for media player playback of a file to perform Syncplay duties

Monitor set parent to be used as offset for positioning cursor; let them be repositioned as a group and individually



Wrapped - create a carousel of the year, a screenshot/file from each day



When the computer is locked - map the controller to 0-9
Dpad 1,2,3,4
Buttins 5,6,7,8
L1 9
R1 0
Unlock using the pin pad


Privacy guards - disable microphone when inputting a password as it can be used to identify keyboard inputs - I wonder how distinct the controller button press sounds are? Todo: train a model to predict controller inputs from their sound
Should be easy to get a ground truth dataset

Minecraft gear change sound when switching tool(belt)s


Toolbelts
- numeric input - 0-9 buttons
- intellisense - ctrl, space, tab, arrow keys
- movement - sticks for character and pointer, l3 sprint
- clicking - l1=m2 r1=m1
- voice - r3=voice to text
- pause - unbind all, chord l3+r3+start to equip switcher toolbelt again
- switcher - start and select show different controller previews with keys shown

Switcher
Hold select to show the wheel
Hold start to show another wheel
Each wheel entry is a box with the icons for the tools it contains
Switching to a Toolbelt will unequip Toolbelt that have conflicting bindings


Holding start shows a diagram of the controller. Will need to build a representation of the controller in the game. The Dpad, the sticks, and the buttons can all be used as 8-directional picker wheels. L1, R1 can be used as additional shift keys to open more wheels. 


Rebind tool - show 6 controller previews (select, select+L1, select+r2, start, start+R1, start+r2) with the picker wheels. Let user click and drag toolbelts between picker wheel slots. 

Chord rebinding - record a key sequence to identify the source, record a second key sequence as the destination, perform move. 

Rebinding is done by changing the runtime input maps in addition to editing the source code defaults so that the changes persist between runs. If the file is open with modifications in vscode, save the file before editing on disk. 


Window management tool
L3 - swap - switch the position of the cursor hero window with the window under the pointer. If cursor hero is on left monitor and vscode on main, easily swap the two sizes and positions.
Dpad - arrow keys
West - windows button
South - alt
L1 - tab
R1 - mouse1
North - maximize
East - minimize
Lets the user do win+tab or alt+tab chords.
L2 - open task bar wheel. Take the task bar and stretch it to fit a circle. Map the pointer position from the wheel to the actual task bar position so when the user clicks with R1 it clicks the task bar. Make the icon at the edge of the circle and have the label on the outside (if at the left of the circle, the icon and label will be reversed from the usual taskbar layout where the icon is to the left of the window title) 
If there are many windows open, the circle will wrap around. Each icon has a larger radius than the previous one, such that it is on a new orbit by the time it wraps around to the start. Only show the window titles for the orbit radius closest to the pointer. 


Right monitor is easy input switch. Left 2 monitors for work (mouse n keyboard), right monitor to control personal system with the controller. 


Media controls
- pause play
- volume
- skip, previous
- show currently playing



Boundary/zone tool
Draw new boundaries
Delete boundaries
Lock boundaries
Unlock boundaries

