The solution (to part2's need of creating the final image) is actually derived from a lesser optimal solution and computing a look up table as a by product and added to the current folder as alignment.txt.

The discussion on this reddit post (https://www.reddit.com/r/adventofcode/comments/kgvx9g/2020_day_20_part_2_how_do_you_assemble_the_image/) is a precursor to this solution.

The idea is based on assigning an orientation to a tile with a sense of directionality like so (as also done by u/Key_Reindeer_414): 


      [0]
      s--e
    e #.## s
[3] | .#.# | [1]  (s = start, e = end)
    | ###. |
    s ###. e
      e--s
      [2]

This orientation is denoted as as `0` or #0. The three clockwise rotations of the above are denotes as orientations #1 through #3 like so:

"#1": 
      [3]
      s--e
    e ##.# s
[2] | ###. | [0]  (s = start, e = end)
    | ##.# |
    s ..## e
      e--s
      [1]

"#2":
      [2]
      s--e
    e .### s
[1] | .### | [3]  (s = start, e = end)
    | #.#. |
    s ##.# e
      e--s
      [0]

"#3":
      [1]
      s--e
    e ##.. s
[0] | #.## | [2]  (s = start, e = end)
    | .### |
    s #.## e
      e--s
      [3]

The fipped version of orientation #0 is thus:
      [2]
      e--s
    s ###. e
[3] | ###. | [1]  (s = start, e = end)
    | .#.# |
    e #.## s
      s--e
      [0]

Notice the reversal of the 's' and 'e' in the above. This is denoted as orientation #4. Orientations #5 through #7 can be achieved by likewise rotating orientation #4, **however**, for this, the rotations are done in ccw manner.

To create the bigger image, we start with a corner as suggested by u/aardvark1231 in the thread. We find the orientation we inserted it into the main image. Now we look at the adjaceny which has the list of all adjacent edges tiles along  with the "alignment", called the `Aligns` in the code, we have a tuple:

1. The orientation of the inserted tile.
2. Its edge that aligns with
3. the "other" edge of the adjacent tile
4. the flag if it aligns in a "flipped" (reversed) manner.

Given 8 possible orientations of the inserted tile, 4 edges as "from", 4 edges aligned "to" and 2 ways of alignment, there are 256 (20 lesser than the current state of the look-up table) possibilities. The mapping from the above tuple to the orientation which the adjacent tile must be for the alignment to work is represented by the look up table. 

Lazily, instead of coming up with the mapping on my own or generating a requisite example to generate it, I decided to run the problem for a few runs (since the choice of the corner is random) and generate it. As with many others, I just tried all orientation till a match occured. That's the reason for the missing 20 tuples. Had this generation been done in a more rigourous fashion (or indeed till the time I work out an analytic formulation for this mapping), the bigger image can be computed by accessing the look-up table for the aligining orientation of the adjacent tile instead of trying it out for each aligining tile pair for each input.



