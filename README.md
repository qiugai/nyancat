Nyancat is a clone of cat, focused on fun!

TO BUILD:

git clone <ssh repo link>

cd nynacat/

cargo build

./run.sh <put args here>


--- Give nyancat a string or a filepath ---
String:
    - $./nyancat "Hello World!"
    -- Hello World!

File Path:
    - $./nyancat /home/jeff/coolfile.txt
    -- Man, this file is so cool!

--- Reverse input (-r | --reverse) ---
    - $./nyancat --reverse racecar
    -- racecar

--- Space out input (-s | --spaced) ---
    - $./nyancat --spaced small
    -- s m a l l

--- Color print input (-c <COLOR> | --color <COLOR>) {red, blue, green, yellow, purple, cyan, white} ---
    - $./nyancat --color red
    -- Just imagine I'm red...

=== TO DO ===

Pig Latin Translation:
     - $./nyancat Hello World!
    -- Ellohay Orldway!
