# Floating-Shapes
Floating Shapes is a program that generates shapes that bounce around until you delete them. 

# Before Building From Source

you need to have sdl2_gfx installed on your OS or the program will not build.

## Linux
on Arch based distros, you can find sld2_gfx in the AUR: 
```bash
$ yay sdl2_gfx
```

## Windows 
//TODO:


## building

go into the repo directory and run 

```bash
$ cargo build
```
now you can run the program.




# Controls
'Tab': Create a new shape
'q': increase width (or radius) for next shape
'w': increase height (or radius) for next shape
'e': decrease width (or radius) for next shape
'r': decrease height (or radius) for next shape
'd': delete last created shape
'c': clear the screen


## why

This project is really just for me to practice Rust and graphical stuff.

