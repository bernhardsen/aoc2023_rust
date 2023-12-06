# Advent of Code 2023

I'm using advent of code to learn a few languages this year. Rust is one of them.
Don't expect great code, I am still learning.

---
### Day 1: Trebuchet?!
Straight forward. Not much to this one.

### Day 2: Cube Conundrum
Initially started parsing the input of this one with regular expressions,
but decided it was better/easier to just manually split strings.
After the input has been parsed, solving the problem was trivial.

### Day 3: Gear Ratios
This was the third language I solve this in, so I decided to go with a slightly
different approach than the others. Instead of building a 2D map similar to the input,
I decided to make a collection of parts and gears, and run collision detection on them.
I think this would make the solution easier to expand with more functions in the future.

### Day 4: Scratchcards
This one I did Rust first, and I tried to do it as quick as possible.
The problem was simple enough that I skipped tests completely
(except some temporary tests I later deleted). I instead added day4_test.txt
as an alternative input file, and ran against that to figure out if I got it right.

I did some slight cleanup/refactoring after the task was done, but not too much.
The parse_cards function is pretty much as it was and I definitely think it
could be easier to read.

### Day 5: If You Give A Seed A Fertilizer
For me the most difficult part of this one was all the maps inside folds inside maps inside whatever.
Keeping track of what is nested where was so much harder than the first implementation,
that I did in C#.

### Day 6: Wait For It
This one was kinda fun. Used some algebra to make it super fast,
because I expected part 2 to be way harder than it was.
Only problem I had was that initially I hard coded the inputs, since it was so short.
I had made a typo, and was getting the wrong answer.
Other than that, this task was very easy. 
