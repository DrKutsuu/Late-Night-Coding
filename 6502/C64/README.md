This is 6502 code written for use in the Vice C64 Emulator (x64)
Assembled using `dasm` to create a program file.

Usage:
1. Write your 6502 program
2. Run `dasm file.asm -v2` to assemble it to `a.out` with verbosity of 2
3. Start your C64 emulator
4. Ensure Drive 8 is hooked up as the current directory (directory mode may help here)
5. Run `LOAD"A.OUT",8,1` to load the program from drive 8, the 1 ensures the program is loaded into the address defined by the `org` instruction (i.e `$1000` is Hex for 4096).
6. Run the program for real using `SYS 4096`
