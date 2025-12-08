Input ← $ L68
        $ L30
        $ R48
        $ L5
        $ R60
        $ L55
        $ L1
        $ L99
        $ R14
        $ L82

Input ← &fras "01-input.txt"

⊜□ ⊸≠ @\n Input # Split input by lines

# For each element replace it with that number of 1s or -1s (right and left)
≡(□ ⍣(˜↯ ¯1 ⊂ [] ⋕ °(⊂@L)|˜↯ 1 ⊂ [] ⋕ °(⊂@R)) °□) 

# join all the lists together into a big list
/◇⊂

# Prepend with 50
⊂ 50

# Add while maintaining intermediate results
\(+)

# Any step that's mod100 is passing through the origin
◿ 100

# Count them
= 0
/+
