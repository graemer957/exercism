```text
total_letters: 1

A           in: 0 / out: 0 / letter: 1
```

```text
total_letters: 2

.A.         in: 0 / out: 1 / letter: 1
B.B         in: 1 / out: 0 / letter: 2
.A.         in: 0 / out: 1 / letter: 1
```

```text
total_letters: 3

..A..       in: 0 / out: 2 / letter: 1
.B.B.       in: 1 / out: 1 / letter: 2
C...C       in: 3 / out: 0 / letter: 3
.B.B.
..A..
```

```text
total_letters: 4
out_spacing = total_letters - letter_number
in_spacing = (letter_number + (letter_number - 1)) - 2
    - special case letter 1 = 0
    - special case letter 2 = 1

...A...     in: 0 / out: 3 / letter: 1
..B.B..     in: 1 / out: 2 / letter: 2
.C...C.     in: 3 / out: 1 / letter: 3
D.....D     in: 5 / out: 0 / letter: 4
.C...C.
..B.B..
...A...
```

```text
total_letters: 5

....A....   in: 0 / out: 4 / letter: 1
...B.B...   in: 1 / out: 3 / letter: 2
..C...C..   in: 3 / out: 2 / letter: 3
.D.....D.   in: 5 / out: 1 / letter: 4
E.......E   in: 7 / out: 0 / letter: 5
.D.....D.
..C...C..
...B.B...
....A....
```
