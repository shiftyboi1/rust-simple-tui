# RUST SIMPLE TUI
This is a lightweight tool designed for making quick and simple terminal UIs in Rust.
___
## Features
- **Simplicity** - Instead of making a full-screen TUi requiring management, react-simple-tui renders a simple UI within a new view in the user's terminal. This makes the UI adaptable to the user's style. The UI is shown using a single command, returning an ID of the selected value, which makes it's implementation trivial.
- **Labels and Actions** - A label allows the addition of a non-selectable item into the UI, which can be used as a description, information or decoration.
- **Header** - An optional string to be rendered at the start of each item in the menu. This will be rendered before the selection cursor.
- **Footer** - An optional string to be rendered at the end of each item. The library gets the length of the longest line in the menu, and aligns the footers to all start after it, handling alignment automatically.
- **Auto-wrap** - The library automatically handles text wrap, wrapping only the content of each line.

## Making a TUI
### 1. Instantiate the Menu object:
```let mut demo_menu: Menu = Menu::default();```
### 2. Add your items
   ```rust
    demo_menu.bordered_label("--------------", Some("┌-"), Some("-┐"));
    demo_menu.bordered_action( "[X]", "" ,Some("| DEMOMENU "), Some(" |"));
    demo_menu.bordered_label("--------------", Some("├-"), Some("-┤"));
    demo_menu.bordered_action("An action", "action", Some("| "), Some(" |"));
    demo_menu.bordered_label("A line", Some("| "), Some(" |"));
    demo_menu.bordered_action("Doggodoggo", "doggo", Some("| "), Some(" |"));
    demo_menu.bordered_label("Label", Some("| "), Some(" |"));
    demo_menu.bordered_label("--------------", Some("└-"), Some("-┘"));
   ```
If no selectable item (action) is added, and the menu is rendered, an "[X]" entry is added with no return id.

Every action has an id, which is the value that is returned when the user makes their selection. Labels do not have an id due to not being selectable. 

Note: ```.action(/*...*/)``` and ```.label(/*...*/)```  add an item with both borders (that being footer and label) empty.

### 3. Render the TUI
Calling the ```enter_tui(/*...*/)``` and ```leave_tui(/*...*/)``` functions is necessary to be able to read raw input. These use crossteerm to open a new window where the TUI is rendered and raw output is read.

The ```.render(/*...*/)```'s parameter is the index, which should be automatically selected when the menu is rendered. If the item at the index is not selectable, the first selectable item is selected instad.
```rust
enter_tui()?;
let result = demo_menu.render(3)?;
leave_tui()?;
```

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
Once the user makes their selection, the TUI will be closed and the ID of the selected item will be returned.
```rust
// Previous code
if result == "doggo" { println!("Doggoprint selected"); }
```
