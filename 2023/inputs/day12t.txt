???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
# ### # ######
????.#...#... 4,1,1
????.######..#####. 1,6,5
#    ######  #####
?###???????? 3,2,1
..??###??.. 5 => 3 (center + wiggle left + wiggle right)

// what if we rle encode the input?
(?, 3) + (., 1) + (#, 3)

??##
// if first_g = ? and 2nd_G = # and tally[0] < 2nd_G.count and first_g.count - 1 >= tally[0]
=> #.## + shift tally

// first solution must have contiguous blocks of '#' to the rightmost as possible
