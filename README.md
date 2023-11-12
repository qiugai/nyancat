# Nyancat
Nyancat is a clone of cat, focused on fun!

## TO BUILD:
Clone the repository. 

```
cd nynacat/

cargo build
```
That's it! To run use the run.sh script.

```
./run.sh <args>
```

## EXAMPLES:
Give nyancat a string or a filepath

String:
```
./nyancat "Hello World!"
_Hello World!_
```
File Path:
```
./nyancat /home/jeff/coolfile.txt
_Man, this file is so cool!_
```

Reverse the given input: [-r | --reverse]
```
./nyancat --reverse racecar
_racecar_
```

Spaced out given input: [-s | --spaced]
```
./nyancat --spaced small
_s m a l l_
```

Color print given input: [-c <COLOR> | --color <COLOR>] colors={red, blue, green, yellow, purple, cyan, white}
```
./nyancat --color red
_Just imagine I'm red..._
```

## TO DO:

Pig Latin Translation:
```
./nyancat Hello World!
Ellohay Orldway!
```