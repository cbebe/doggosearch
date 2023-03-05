# Doggo Search

I couldn't for the life of me find 'DOG' in this puzzle, then I saw that people
even found 'DOGGO'. This is an (over-engineered) solution in Rust to find
'DOGGO'.

![Crossword puzzle consisting entirely of Ds, Os, and Gs](./doggo.jpeg)

# Running

Both approaches find the word at compile time based on the puzzle in `dog_lib::grid::PUZZLE`
and embeds the message in the executable.

## Regular binary

This approach uses `libc` for printing and bootstrapping the `main` function.
Release build comes out as 16 kB on my Linux machine.

```bash
git clone https://github.com/cbebe/doggosearch
cd doggosearch/dog_bin
cargo run --release --quiet
```

## Tiny Binary

Creates a tiny executable file and uses raw Linux syscalls to print the message
and exit. Based on [Tiny ELF
Modernized](https://nathanotterness.com/2021/10/tiny_elf_modernized.html).
Final executable comes out at 406 bytes with the default puzzle.

Requires `nasm` to assemble the tiny Linux executable.

Annotated assembly code is in [./dog_tiny/hello.asm](./dog_tiny/hello.asm).

```bash
git clone https://github.com/cbebe/doggosearch
cd doggosearch
make tiny
./doggosearch_tiny
```

# Results

<details>
  <summary>Result (Spoiler!!)</summary>

It's (1-indexed) column 8, row 3, going down and to the right.

I almost didn't believe that it was actually there until it printed
`Found DOGGO in Cell(2, 7), DownRight`

</details>
