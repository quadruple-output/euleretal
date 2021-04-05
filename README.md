# euleretal _(Euler et. al.)_

## Overview

This is an interactive visualization tool for exploring the qualities of
some discrete methods for solving second order differential equations.
Practically, this is about calculating the trajectory of rigid bodies (masses)
accelerated in different kinds of force fields.  (But let's forget about _masses_ and _forces_ and simply talk about _acceleration fields._)

The idea is to compare the precisions of different algorithms of, for example
the [Euler](https://en.wikipedia.org/wiki/Euler_method) and
[Runge-Kutta](https://en.wikipedia.org/wiki/Runge%E2%80%93Kutta_methods)
family, and their suitability for different force fields.

This is work in progress, and in an early stage.  You can [test the application
online](https://quadruple-output.github.io/euleretal/).  _(Hint: Drag the graph
with the mouse or zoom in/out by holding down the CTRL or CMD key and moving
the mouse up/down.)_


## Disclaimer  

I am not a mathematician or expert in this area.  (If I was, I
wouldn't have had the need to write this tool in the first place.)  It is
likely that I use non-standard or wrong names for some of the methods, and I
might implement them in the wrong way.  If you have suggestions, do not
hesitate to create an
[Issue](https://github.com/quadruple-output/euleretal/issues),
[Discussion](https://github.com/quadruple-output/euleretal/discussions), or
[send me an email](57874618+quadruple-output@users.noreply.github.com).


## Why?

I wanted to write a 3D-Simulation of particles being attracted or repelled by
other particles or objects in a simple world.  I quickly found that my
particles diverted from the paths I would have expected them to take.  This
became obvious when two masses came too close to each other, or when I created
a “wall” which was supposed to repell any particles with a force which should
asymptotically approach infinity (I can hear you loughing!). 

So I read about different methods like Euler or Runge-Kutta _(todo: citation
needed)_, but it was hard for me to understand all the details or follow the
theoretical arguments.  I wanted to develop an intuitive idea about why some
Runge-Kutta methods make sense, and I found myself drawing lines on paper.
Alas, these lines did not help me because I was not able to draw them with
sufficient precision in order to understand whether my ideas would work out.

…and then there was the moment where I realized that the Euler method was plainly wrong when it comes to second order integration:

```
v' = v + a(s) dt
s' = s + v' dt
```

Inserting the first formula into the second, we get:

```
s' = s + v dt + a(s) dt²
```

But I knew from high school physics that the following is correct when `a` is
constant:

```
s' = s + v dt + ½ a dt²
```

This means that the Euler method is not even correct for the trivial case of
constant acceleration — WTF!  The literature I found so far, often made a
statement similar to this one on
[Wikipedia](https://en.wikipedia.org/wiki/Numerical_methods_for_ordinary_differential_equations):
_“Without loss of generality to higher-order systems, we restrict ourselves to
first-order differential equations.”_ Well, while this may be true in theory, I
now know that I can loose big in practice if I apply first order methods to my
second order problem.

Finally, I came up with some formulas on my own.  I am 100% sure that others 
have used them before, but I do not have the mathematical background to name
them.  This is where my frustration turned into curiosity and fun.  I wanted
to see the formulas in action, so I started this little project.


## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE.txt) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT.txt) or http://opensource.org/licenses/MIT)

at your option.


## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
  be dual licensed as above, without any additional terms or conditions.