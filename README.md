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
> Hello World!
```
File Path:
```
./nyancat /home/jeff/coolfile.txt
> Man, this file is so cool!
```

Reverse the given input: [-r | --reverse]
```
./nyancat --reverse racecar
 >racecar
```

Spaced out given input: [-s | --spaced]
```
./nyancat --spaced small
> s m a l l
```

Color print given input: [-c <COLOR> | --color <COLOR>] colors={red, blue, green, yellow, purple, cyan, white}
```
./nyancat --color red
> Just imagine I'm red...
```

## TO DO:

Pig Latin Translation:
```
./nyancat Hello World!
> Ellohay Orldway!
```