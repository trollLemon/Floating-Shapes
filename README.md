# Floating-Shapes
Floating Shapes is a program that generates shapes that bounce around until you delete them. 

# Before Running

you need to have sdl2_gfx installed on your OS or the program will crash:

### To install:

## Linux
on Arch based distros, you can find sld2_gfx in the AUR: 
```bash
$ yay sdl2_gfx
```

## Windows 
//TODO:


# Controles
'Tab': Create a new shape
'q': increase width (or radius) for next shape
'w': increase height (or radius) for next shape
'e': decrease width (or radius) for next shape
'r': decrease height (or radius) for next shape
'd': delete last created shape
'c': clear the screen

## How To Run
do 
```bash
$ cargo run
```
to launch the program from the terminal, or you can compile the code yourself if you'd like: 
```bash
$ cargo build
```