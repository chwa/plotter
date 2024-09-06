
Plot object
- multiple subplots/rows with shared x axis
-


Graphics elements:
- Axes with primary/secondary x/y axis (with labels), chart background, grid, Title
    - Cairo clip for drawing on the chart area only
    - Axis
    - Grid
- Legend
- Trace?
- Cursor that snaps to all elements, selection on click?
- Cursor x/y coordinates
- Trace annotation/marker
- Scrollbar?

all have a draw(&self, &cx) method

```rust

let plt = Plot::builder()
             .figsize(400,300)
             .rows(3)
             .title("My title")
             .build();

plt.row(1).plot(wfm, Some("Label"));




```



- Plot:
  - title
  - collection of axis
  - grid
  - collection of traces
  - collection of annotations/markers?

- axis:
  - rectangle in pixel coordinates (or position/start/stop to define a horiz/vert line)
  - 4 orientations (place on any edge of the chart)
  - label (optional)
  - lin/log
  - normal/inverted
  - major ticker (with labels)
  - minor ticker (optional)
