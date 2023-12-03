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
