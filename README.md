

Plot
- holds all non-GUI state: Axes, Axis, Traces (via Arc), ...
- does *not* hold window size, cursor position, etc.
- draw method to draw to cairo context
- methods to query objects/data coordinates for given device coordinatees
  (for zooming/scrolling the Axes, selecting Trace etc.)
- zoom_fit/zoom_at/zoom_rect methods for changing the view
-


PlotWidget (Component?)
- Holds the DrawingArea (or inherits from it?)
- Receives mouse/key events and calls Plot methods to query/update state etc.
-
-


Things to add:
- legend
- scrollbar?
- snapping cursor
- trace annotation/value marker
- delta (a-b) marker


```rust

let plt = Plot::builder()
             .figsize(400,300)
             .rows(3)
             .title("My title")
             .build();

plt.row(1).plot(wfm, Some("Label"));




```


