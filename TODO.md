# TODO: Performance Improvements

- Remove unnecessary continuous redraws

  - [X] Stop calling queue_draw() on a 16ms timer unless there are dynamic updates or animations.
  - [X] Only redraw when the DOM changes or user interacts.
- Avoid cloning large data structures
  - [x] Do not clone the entire DOM (root_dom.clone()) if not needed.
  - [ ] Avoid creating GTK widgets for empty nodes, comments, or invisible elements.
- Reuse widgets when updating the DOM
  - [ ] Instead of recreating widgets on updates, update existing widgets’ content to save memory.
  - [ ] Optimize CSS provider usage
- Create a single global CssProvider instead of creating multiple providers.

  - [ ] Avoid reloading CSS unnecessarily.
  - [ ] Use WeakRef for large GTK objects in closures
- Replace strong clones in closures with WeakRef to prevent reference cycles and memory leaks.

  - [ ] Profile memory usage
  - [ ] Measure memory and CPU usage using tools like valgrind, perf, or Windows Task Manager.
- Identify large allocations or repeated resource usage.

  - [ ] Consider HTML parsing optimizations
  - [ ] Avoid storing unnecessary intermediate data structures.
- Only parse and store attributes/nodes you need for rendering.

  - [ ] Reduce string allocations
  - [ ] Reuse strings or use Cow`<str>` for DOM text content when possible.
- Consider throttling expensive operations

  - [ ] For tasks like window positioning or monitoring, do them once or at low frequency.
  - [ ] Avoid repeating operations every frame unnecessarily.

- Make an GUI to manage the widgets
  - [ ] Refactor the code need a frame where it shows all the widgets founded in the `widget` folder
  - [ ] Activate or deactivate widgets

# PLANED
- [ ] Custom CSS for styling
- [ ] JavaScript for interactivity
- [ ] Real-time updates ❌
- [ ] Event system for buttons
- [ ] More HTML elements (input, textarea, etc.)
- [ ] Animations and transitions
- [ ] Predefined themes
- [ ] Create an VSCODE extension for an *.ytml files
  - Snippes & more...