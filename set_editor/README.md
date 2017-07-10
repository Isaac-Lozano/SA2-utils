Set File Editor
===============

This program edits setfiles and can turn setfiles to and from human-readable json files.

You can run this program either from the commandline or as a GUI application.

GUI
---

To launch as a GUI application, simply execute the program normally (or with the commandline option -g).

Features:
* Object editing.
* Add and Remove objects.
* Object sorting by clicking on the column headers.
* Object searching via the Search By Column menu.
* Distance searching via the Search By Distance menu.
* Add and Remove objects.
* Object Name translation using a json file lookup table.
    - (The Level button on the bottom right selects which level object table to look at)

CLI
---

Actions:
* Decode setfiles to json.
    - `set_editor.exe -d [SETFILE] [JSON_OUTPUT]`
    - Optional single-line mode `-s`
* Encode json to setfile format.
    - `set_editor.exe -e [JSONFILE] [SETFILE_OUTPUT]`
* Start GUI mode.
    - `set_editor.exe -g`
* Help
    - `set_editor.exe -h`
