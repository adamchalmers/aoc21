I was torn between representing Snailfish numbers as a tree or as a string. The tree representation
makes calculating magnitudes easy, and the string representation makes applying the "explode"
reduction easy. Eventually I decided to parse the numbers into a string, apply the reductions, then
convert the string to a tree and find the magnitude. I used a Nom parser to convert my linear 
(string) representation into a tree.

But then I realized, you don't actually need a tree to calculate magnitude, you just need recursion.
The tree is a nice representation of the recursive structure, but we only ever need it once, for
calculating magnitudes, after which it's discarded. So, I repurposed the Tree parser. Instead of
returning a tree, it just adds up the magnitude as it goes, returning a u16. This was the first time
I had used Nom to "consume" a data structure, instead of just parsing it into some nice 
representation.
