# Euleretal TODO list

## UI Bugs
- When the app state is restored from a previous run, changing colors of an
  Integrator has no effect on an existing integration.  Root cause is that
  deserialization multiplies reference-counted instances.
- "Integrations" Pop-Up should be restricted to be placed inside its parent
  canvas.
- Drop-Downs (Combo boxes) in "Integrations" Pop-Up are sometimes clipped away.
- Description tool tips sometimes break text too early → increase width.

## UI features
- For position contributions of `½a dt^2`, draw a curved trajectory.
- Layer 'Acceleration Field' should draw scaled vectors at each sampling point
  of a Step (transparent color, so they can be distinguished from
  contributions).
- Predefine a list of colors which can be used as defaults for new Step Sizes
  or Integrators.
- Button for auto-zoom.
- Render Integrator formulas in browser (KaTeX or MathJax).
- Dark and Light themes.
- When zooming, zoom away from pointer, but not from center of canvas.
- In lists with '+' button, put button at end of list (where the '-' button for
  the new entry will appear).
- In drop-downs/choosers, apply the chosen option immediately as preview on
  hover, without the need of clicking (design a new chooser for this).
- Re-enable hover delay.
- "Integrators" controls: show unused integrators separated from others, and
  collapsed.
- Add setting for `Inspector` to not scale velocities by dt (may be useful when
  comparing integrations with different step sizes).
- Add option to (continuously) synchronize the view point and scale of all
  canvases.
- Add button to move an integration from one canvas to another (possibly new
  one).
- Group Integrators by number of samples from acceletation field they require.
- More flexible canvas grid layouts.

## Functional Features
- Change selected ("linked") step sizes proportionally with a single slider
  (useful for testing convergence).
- Interactively change scenario start conditions (velocity&direction).
- Add more Integrators.
- Make canvas grid more flexible.
- Save&restore state automatically.
- Customizable Integrators.
- Save/load state in/from file.

## Clean Code
- write more unit tests for specific Integrators

## Performance
- use a different allocator for everything related to `Step`s.  While
  integrating, free-ing is not required.  When Integration result gets
  discarded, all allocated mem can be freed at once.
