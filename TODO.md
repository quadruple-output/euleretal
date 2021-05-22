# Euleretal TODO list

## Bugs
- "Integrations" Pop-Up should be restricted to be placed inside its parent
  canvas
- Drop-Downs (Combo boxes) in "Integrations" Pop-Up are sometimes clipped away
- Description tool tips sometimes break text too early → increase width

## UI features
- predefine list of colors which can be used as defaults for new Step Sizes or
  Integrators
- support for mobile devices (touch gestures)
- Button for auto-zoom
- render Integrator formulas in browser (KaTeX or MathJax)
- Dark and Light themes
- when zooming, zoom away from pointer, but not from center of canvas

## Functional Features
- change all step sizes proportionally with a single slider (to test convergence)
- interactively change scenario start conditions (velocity&direction)
- add more Integrators
- make canvas grid more flexible
- save/restore state automatically
- customizable Integrators
- save/load state in file

## Clean Code
- Stritly separate `core` from `ui`. `ui` should not publish anything.  Some
  `core` types must get a `ui`-twin with additional data relevant for `ui` only
  (like color, label, and stroke, but also `ChangeTracker`)
- Create a version of Vec3, based on R32, so we can `#[derive(Hash)]` for it.