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

### Day 7: Camel Cards
I liked this one. I expected part 2 to be something like adding straight as a possible hand,
but I think the joker one was easier. I originally implemented scoring the hand value in
a different way, but switched to counting duplicates when I saw part 2.

### Day 8: Haunted Wasteland
It hurts my brain any time we have to build graphs and stuff in Rust.
I did brute force part 2, and I know there is a way to do it without.
Something, something about the least common multiplier, but I couldn't figure out
how to offset those by the time it takes to get to the first Z node.
Whatever, we're done with this one...

### Day 9: Mirage Maintenance
I liked this one. It was one of the easier problems recently.
It comes together really nicely when you just break it down into steps.

### Day 10: Pipe Maze
Kinda meh problem and implementation. I don't link how much code I ended up with.
I think the worst part is the "Figure out what shape the start square is".
That could probably be written very differently.

### Day 11: Cosmic Expansion
Fun problem. Not that difficult. It was all about counting the number of expansions we crossed,
instead of materializing them on the map, and actually calculating distances.

### Day 12: The one we skipped
I tried this one in C#, but eventually gave up.
Part 1 was not that bad, but part 2 too difficult. I might return to this one later.

### Day 13: Point of Incidence
Fun one. Important thing here for part 2 was to make sure to ignore the original reflection,
because changing/fixing the smudge, might not invalidate the original one. It worked
as long as the new reflection was earlier in the search than the original. It was easy
enough to fix once I realized what was going on.

### Day 16: The Floor Will Be Lava
One of those "Find the repeating pattern"-type problems. I confused myself a bit reversing
the vector of results. I should have just left it in the original order, but I wanted to
put it in the same order I was thinking in.
