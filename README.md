# 🎨 Desktop Widgets - Documentation

Customizable desktop widget system using Rust, GTK4, and simplified YTML markup.

## 📋 Table of Contents

1. [Installation](#installation)
2. [Project Structure](#project-structure)
3. [Create Your First Widget](#create-your-first-widget)
4. [YTML Format](#ytml-format)
5. [Window Configuration](#window-configuration)
6. [Supported Tags](#supported-tags)
7. [Complete Examples](#complete-examples)
8. [Troubleshooting](#troubleshooting)

---

## 🚀 Installation

### Prerequisites

* **Rust** (version 1.70 or higher)
* **GTK4** installed on your system
    * Windows: [GTK4 for Windows](https://gtk.org/)
    * Linux: `sudo apt install libgtk-4-dev`
    * macOS: `brew install gtk4`

### Build the Project

```bash
# Clone or download the project
cd your_project

# Build in development mode
cargo build

# Build in release mode (optimized)
cargo build --release
```

### Folder Structure

After building, create the `widget/` folder next to the executable:

```
your_project/
├── widget/              ← Create this folder
│   ├── widget1.ytml     ← Your widgets here
│   ├── widget2.ytml
│   └── assets/          ← Images and resources
│       └── rust.png
└── target/
    └── release/
        └── your_program.exe
```

---

## 📁 Project Structure

```
project/
├── src/
│   ├── main.rs                    # Main entry point
│   ├── parser/
│   │   ├── mod.rs
│   │   └── ytml_parser.rs         # YTML parser
│   └── renderer/
│       ├── mod.rs
│       └── gtk_renderer.rs        # GTK renderer
│
├── widget/                         # 📂 YOUR WIDGETS HERE
│   ├── clock.ytml                 # Clock widget
│   ├── notes.ytml                 # Notes widget
│   ├── weather.ytml               # Weather widget
│   └── assets/                    # Resources (images, etc.)
│       ├── icon.png
│       └── background.jpg
│
├── Cargo.toml
└── README.md
```

---

## 🎯 Create Your First Widget

### 1. Create the YTML file

Create `widget/hello.ytml`:

```ytml
<config>
    <window width="300" height="200" />
    <decorations enabled="false" />
    <resizable enabled="false" />
</config>

<body>
    <div>
        <h1>Hello World!</h1>
        <p>My first desktop widget</p>
        <button>Click Me!</button>
    </div>
</body>
```

### 2. Run the application

```bash
cargo run
```

Your widget will appear on the desktop! 🎉

---

## 📝 YTML Format

### What is YTML?

**YTML** (Yet another Markup Language) is a custom internal format specifically designed for desktop widget definitions. It combines the simplicity of HTML with features optimized for GTK4 rendering.

### Key Features of YTML

**Simplified Structure**: Unlike HTML, YTML uses a more restrictive and uniform syntax to ensure predictable parsing.

**Built-in Validation**: The YTML parser automatically validates the structure and properly closes incomplete tags.

**Typed Attributes**: Attributes in YTML always use quotes and are validated according to their type (number, boolean, string).

**Whitespace Tolerant**: YTML ignores extra whitespace, allowing for better code readability.

**GTK4 Optimized**: Each YTML element maps directly to a corresponding GTK4 widget, eliminating unnecessary conversions.

### Basic YTML Syntax

```ytml
<!-- Comments in YTML -->
<element attribute="value">
    <!-- Nested content -->
</element>

<!-- Self-closing tags -->
<element attribute="value" />
```

### Required Structure

Every YTML file must contain exactly two root sections:

```ytml
<config>
    <!-- Window configuration and behavior -->
</config>

<body>
    <!-- Widget visual content -->
</body>
```

### YTML Validation Rules

The YTML specification implements the following rules:

**Recognized Tags**: Only predefined tags are accepted (`config`, `window`, `decorations`, `transparent`, `resizable`, `body`, `div`, `h1-h6`, `p`, `button`, `img`).

**Correct Nesting**: The `<config>` section can only contain configuration tags (`<window>`, `<decorations>`, `<transparent>`, `<resizable>`). The `<body>` section can only contain visual elements (`<div>`, `<h1-h6>`, `<p>`, `<button>`, `<img>`).

**Mandatory Attributes**: Certain elements require specific attributes (e.g., `<img>` requires `src`).

**Boolean Values**: Boolean attributes must be exactly `"true"` or `"false"` (in quotes).

**Numeric Values**: Attributes expecting numbers must be valid integers.

---

## ⚙️ Window Configuration

### `<config>` Tag

Defines how the widget window behaves and looks.

#### `<window>`

Window size and position:

```ytml
<window width="400" height="300" />
```

With specific position:

```ytml
<window width="400" height="300" x="100" y="100" />
```

**Attributes:**

* `width`: Width in pixels (default: 800)
* `height`: Height in pixels (default: 600)
* `x`: Horizontal position (optional, default: centered)
* `y`: Vertical position (optional, default: centered)

#### `<decorations>`

Show/hide window borders:

```ytml
<decorations enabled="false" />  <!-- No borders or buttons -->
<decorations enabled="true" />   <!-- With borders and buttons -->
```

#### `<transparent>`

Enable transparency (experimental):

```ytml
<transparent enabled="true" />
```

#### `<resizable>`

Allow window resizing:

```ytml
<resizable enabled="true" />   <!-- Resizable -->
<resizable enabled="false" />  <!-- Fixed size -->
```

### Complete Config Example

```ytml
<config>
    <window width="500" height="400" x="50" y="50" />
    <decorations enabled="false" />
    <transparent enabled="false" />
    <resizable enabled="true" />
</config>
```

---

## 🏷️ Supported Tags

### Headers

```ytml
<h1>Main Title</h1>
<h2>Subtitle</h2>
<h3>Header Level 3</h3>
<h4>Header Level 4</h4>
<h5>Header Level 5</h5>
<h6>Header Level 6</h6>
```

**Font Sizes:**

* h1: 32px
* h2: 28px
* h3: 24px
* h4: 20px
* h5: 18px
* h6: 16px

### Paragraphs

```ytml
<p>This is a normal text paragraph.</p>
```

### Images

```ytml
<img src="assets/logo.png" width="200" />
```

**Attributes:**

* `src`: Relative path to executable folder (required)
* `width`: Width in pixels (default: 350)

### Buttons

```ytml
<button>Click Me!</button>
<button id="btn-primary" width="200">Wide Button</button>
<button id="btn-secondary" height="50">Tall Button</button>
```

**Attributes:**

* `id`: Unique identifier (optional)
* `width`: Width in pixels (optional)
* `height`: Height in pixels (optional)

### Containers

```ytml
<div>
    <h1>Group of elements</h1>
    <p>Containers group other elements</p>
</div>

<div id="sidebar">
    <button>Button 1</button>
    <button>Button 2</button>
</div>
```

**Attributes:**

* `id`: Unique identifier (optional)

**Features:**

* Automatic vertical orientation
* 10px spacing between elements
* 10px margins on all sides

---

## 📚 Complete Examples

### Simple Clock Widget

`widget/clock.ytml`:

```ytml
<config>
    <window width="250" height="150" x="20" y="20" />
    <decorations enabled="false" />
    <resizable enabled="false" />
</config>

<body>
    <div id="clock-container">
        <h1>🕐 12:34</h1>
        <p>Tuesday, Nov 19 2024</p>
    </div>
</body>
```

### Notes Widget

`widget/notes.ytml`:

```ytml
<config>
    <window width="350" height="400" x="300" y="20" />
    <decorations enabled="false" />
    <resizable enabled="true" />
</config>

<body>
    <div>
        <h2>📝 Quick Notes</h2>
        
        <div id="note-1">
            <h4>Shopping</h4>
            <p>- Milk<br/>- Bread<br/>- Eggs</p>
        </div>
        
        <div id="note-2">
            <h4>To-Do</h4>
            <p>- Finish project<br/>- Call doctor</p>
        </div>
        
        <button width="300">Add Note</button>
    </div>
</body>
```

### Profile Widget

`widget/profile.ytml`:

```ytml
<config>
    <window width="300" height="350" x="700" y="20" />
    <decorations enabled="false" />
    <resizable enabled="false" />
</config>

<body>
    <div id="profile">
        <h2>👤 My Profile</h2>
        
        <img src="assets/avatar.png" width="150" />
        
        <h3>John Doe</h3>
        <p>Software Developer</p>
        
        <div id="buttons">
            <button width="250">Edit Profile</button>
            <button width="250">Settings</button>
        </div>
    </div>
</body>
```

### Dashboard Widget

`widget/dashboard.ytml`:

```ytml
<config>
    <window width="600" height="500" x="100" y="100" />
    <decorations enabled="false" />
    <resizable enabled="true" />
</config>

<body>
    <div>
        <h1>📊 Dashboard</h1>
        
        <div id="stats">
            <h3>Today's Stats</h3>
            <p>• 45 tasks completed</p>
            <p>• 3 hours of coding</p>
            <p>• 12 commits</p>
        </div>
        
        <div id="quick-actions">
            <h3>Quick Actions</h3>
            <button width="250">New Task</button>
            <button width="250">View Calendar</button>
            <button width="250">Open Project</button>
        </div>
        
        <img src="assets/chart.png" width="500" />
    </div>
</body>
```

---

## 🔧 Troubleshooting

### ❌ "No valid widgets found in widget/ folder"

**Solution:**

1. Verify that the `widget/` folder exists next to the executable
2. Make sure files have `.ytml` extension
3. Check that YTML files are valid

### ❌ "Error parsing YTML"

**Common causes:**

* Missing `<config>` or `<body>` section
* Unclosed tags
* Attributes without quotes

**Correct format:**

```ytml
<button width="200">Text</button>  ✅
<button width=200>Text</button>    ❌
<window width="400" height="300" /> ✅
<window width="400"height="300" /> ❌
```

### ❌ "Image without src" or image not showing

**Solution:**

1. Verify the path is relative to the executable
2. The `assets/` folder must be next to the executable

```
target/release/
├── your_program.exe
├── widget/
│   └── my_widget.ytml
└── assets/              ← Images go here
    └── image.png
```

In YTML use:

```ytml
<img src="assets/image.png" width="200" />
```

### ❌ Widget doesn't appear on desktop (Windows)

**Possible causes:**

1. Windows Defender or antivirus blocking
2. GTK4 not installed correctly
3. Insufficient permissions

**Solution:**

* Run as administrator the first time
* Make sure GTK4 is in PATH

### ❌ "Could not connect to a display"

**On Linux/WSL:**

```bash
export DISPLAY=:0
```

### 💡 Widget shows but doesn't update

**Solution:**

* Widgets are static by default
* For dynamic content, you'll need to modify the Rust code
* Restart the application after editing YTML files

---

## 🎨 Design Tips

### 1. **Keep widgets small**

* Recommended sizes: 200-500px width
* Avoid making widgets too large

### 2. **Use descriptive IDs**

```ytml
<button id="save-button">Save</button>
<div id="notification-panel">...</div>
```

### 3. **Organize your content**

```ytml
<div id="header">
    <h1>Title</h1>
</div>

<div id="content">
    <p>Main content</p>
</div>

<div id="footer">
    <button>Action</button>
</div>
```

### 4. **Group related elements**

```ytml
<div id="button-group">
    <button>Option 1</button>
    <button>Option 2</button>
    <button>Option 3</button>
</div>
```

---

## 🚀 Next Steps

### Planned Features

* [ ] Custom CSS for styling
* [ ] JavaScript for interactivity
* [ ] Real-time updates
* [ ] Event system for buttons
* [ ] More HTML elements (input, textarea, etc.)
* [ ] Animations and transitions
* [ ] Predefined themes

### Contributing

Have ideas or improvements? Contributions are welcome!

---

## 📖 Quick Reference

### Execution Commands

```bash
cargo run                    # Development
cargo build --release        # Production
./target/release/program     # Execute
```

### Minimal Widget Structure

```ytml
<config>
    <window width="300" height="200" />
    <decorations enabled="false" />
</config>

<body>
    <div>
        <h1>My Widget</h1>
    </div>
</body>
```

### Most Used Attributes

| Tag          | Attributes          | Example                                                |
| ------------ | ------------------- | ------------------------------------------------------ |
| `<window>` | width, height, x, y | `<window width="400" height="300" x="100" y="50" />` |
| `<button>` | id, width, height   | `<button id="btn1" width="200">Click</button>`       |
| `<img>`    | src, width          | `<img src="assets/logo.png" width="150" />`          |
| `<div>`    | id                  | `<div id="container">...</div>`                      |

---

## 📞 Support

Problems or questions? Open an issue in the project repository.

---

**Happy widget building! 🎉**