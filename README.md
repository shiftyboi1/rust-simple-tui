# INTUITIVE
Intuitive is a lightweight tool designed for making quick and simple terminal UIs in Rust. It is to be used for projects like installers, configuration tools and other simple applications where a full-screen TUI would be overkill. It is designed to be as simple as possible, while still being customisable.

To use it, add the following to your Cargo.toml:
```toml
[dependencies]
intuitive = { git = "https://github.com/shiftyboi1/Intuitive" }
```

___
## Features
- **Simplicity** - Instead of making a full-screen TUI requiring management, Intuitive renders a simple UI within a new view in the user's terminal. This makes the UI adaptable to the user's style. The UI is shown using a single command, returning an ID of the selected value, which makes it's implementation trivial.
- **Labels and Actions** - A label allows the addition of a non-selectable item into the UI, which can be used as a description, information or decoration.
- **Header** - An optional string to be rendered at the start of each item in the menu. This will be rendered before the selection cursor.
- **Footer** - An optional string to be rendered at the end of each item. The library gets the length of the longest line in the menu, and aligns the footers to all start after it, handling alignment automatically.
- **Auto-wrap** - The library automatically handles text wrap, wrapping only the content of each line. The header and footer can optionally repeat on each line if an item needs to be wrapped, allowing for more customisable decoration.

## Making a TUI
### 1. Instantiate the Menu object:
```let mut demo_menu: Menu = Menu::default();```
### 2. Add your items

The ```.bordered_action(/*...*/)``` function takes the label (displayed text), the id (value returned when selected), the header, the footer and whether to repeat the header and footer on wrapped lines. The ```.bordered_label(/*...*/)``` function takes the same parameters, but without the id since labels are not selectable.

The ```.action(/*...*/)``` and ```.label(/*...*/)``` functions add an item with both borders (that being footer and label) empty.

   ```rust
    demo_menu.bordered_label("--------------", Some("┌-"), Some("-┐"), false);
    demo_menu.bordered_action( "[X]", "" , Some("| DEMOMENU "), Some(" |"), true);
    demo_menu.bordered_label("--------------", Some("├-"), Some("-┤"), false);
    demo_menu.bordered_action("An action", "action", Some("| "), Some(" |"), true);
    demo_menu.bordered_label("A line", Some("| "), Some(" |"), true);
    demo_menu.bordered_action("Doggodoggo", "doggo", Some("| "), Some(" |"), true);
    demo_menu.bordered_label("Label", Some("| "), Some(" |"), true);
    demo_menu.bordered_label("--------------", Some("└-"), Some("-┘"), false);
   ```
If no selectable item (action) is added, and the menu is rendered, an "[X]" entry is added with no return id.

### 3. Render the TUI
Calling the ```enter_tui(/*...*/)``` and ```leave_tui(/*...*/)``` functions is necessary to be able to read raw input. These use crossterm to open a new window where the TUI is rendered and raw output is read.

The ```.render(/*...*/)``` takes the index of the item, which should be automatically selected when the menu is rendered. If the item at the index is not selectable, the first selectable item is selected instead. This index is independent of line wrap. The list of items is zero-indexed.
```rust
enter_tui()?;
let result = demo_menu.render(3)?;
leave_tui()?;
```
Note: Since only the label text is wrapped, make sure to not make the footer and header too long, as they do not wrap. If not even a single line would fit into the output terminal's width, a message saying "Unsupported size. Please resize." is shown. The TUI is also only re-rendered when an input is detected.

### 4. Await output
The ```demo_menu``` TUI created earlier should look like this:
```
┌----------------┐
| DEMOMENU   [X] |
├----------------┤
| > An action    |
| A line         |
|   Doggodoggo   |
| Label          |
└----------------┘
```
Note the cursor (```>```) being on the fourth line. This is because the earlier ```.render(3)``` call was passed the index 3, which equates to the 4th item due to the list of items being zero-indexed. In this menu, the ```[X]``` is also selectable due to it being an action, with the text before it being a header.

Actions have a padding of two spaces to allow for the cursor. It is heavily encouraged to keep this line difference consistent, as to make it obvious which entries are selectable and which are not. The cursor can be moved using the up and down arrow keys. To make a selection, press the Enter key.

Once the user makes their selection, the TUI will be closed and the ID of the selected item will be returned. For example, the following code will print "Doggoprint selected" if the user selects the "Doggodoggo" entry (due to its id being "doggo"):
```rust
// Previous code
if result == "doggo" { println!("Doggoprint selected"); }
```
