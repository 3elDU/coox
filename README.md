# Copper Oxide

**Just because you can, doesn’t mean you should** - the motto of this project.

An attempt in writing a browser engine for the terminal.

This is me trying to have fun, so please bear in mind. This is not a serious web engine.

Writing a browser engine is monumental work, and I'm not trying to do that
"from scratch". I am using other people's work for large parts of the project.
For example, I don't want to write a JavaScript engine, nor do I want to write an
HTML parser. For that I'm using existing libraries.

I *would* like to glue all of that together and see how far I can go.

# Core idea & limitations

What if I used an existing JavaScript interpreter (Boa in my case), an existing HTML parser, an existing (put a library here) and made a browser with that? That runs in the terminal.

You don't have to think long in order to realize the shortcomings of running applications intended for proper screens inside the terminal. First of all, terminal is character-based. There are no pixels, while the primary size unit on the web is a pixel.

All characters are the same size, and there is no way to show different font sizes. Font styling is limited to very basic operations, such as underlines, italics and bold characters. There are colors, though!

Characters in the terminal generally have a 1:2 width to height ratio, and I'd like to have proper CSS layout in some future, so there needs to be a conversion between CSS units and terminal characters. For square blocks one character in size, we can use the half-block element (U+2584): ▄, but I don't plan to render any complex "sub-pixels" using braille characters or whatever. As such, all boxes need to be clamped in size to the minimum of 1/2 character.

With that, layouts such as flexbox should actually be simpler to implement, because measuring character size is essentially a free operation.

Not all terminals (and monospace fonts) are made equal, so the 1:2 ratio is not universal. I do want to make this configurable by the user to better suit different fonts.

# Non-goals

1. Security. JavaScript is sandboxed, obviously, and I'd only implement the regular Web apis, but there would be no advanced security mechanisms, such as process isolation.
2. Rendering video, canvas, WebGL, 3D transforms, SVG and other complex graphics
3. Performance. I have the benefit of writing in a compiled language, and I won't hesitate doing simple optimizations, but having fun is more important than optimizing.
4. Implementing every Web API. As stated with videos above, some features simply don't make sense in the terminal. This expands to and CSS too.

# Goals

1. Running JavaScript.
1. Compliance with Web standards.
1. Running Web apps. This is one of the reasons why I started this project. All of the terminal browsers fall into two categories:
    - Custom engine, doesn't run any JS
    - Just a Chromium|Firefox wrapper with custom rendering
1. Website layout should look similar to a desktop browser.