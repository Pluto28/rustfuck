# Why?

Honestly, just a toy project to get more acquainted with the
honorable rust language, slayer of CPPuses

Also note that this being a toy project, it will be very verbose and i'll try to
decompose my code into a lot of unecessary modules and add a lot of things
that most experienced programmers would not. That is because the main goal 
here is learning and nothing more.

# spec
     The intepreter provides for the programmer 30000 bytes of 1 byte blocks. 
Brainfuck has 8 operators in total, namely

|  op   |  description                                                               |
| ----- | -------------------------------------------------------------------------- |
|   >   |  increases memory pointer by 1                                             |
|   <   |  decreases memory pointer by 1                                             |
|   +   |  Increases the value pointed to by the memory pointer by 1                 |
|   -   |  Decreases the value pointed to by the memory pointer by 1                 |
|   ]   |  loop until the value at the current block(memory pointer) is 0            |
|   [   |  If value pointed by current block is not equal to 0, jump to ]            |
|   ,   |  Input a 1 byte character from the user                                    |
|   .   |  Output the 1 byte character at the pointed to memory block to the stdout  |


### So what does it do?

This retina burning piece of software, comparable to an ecclipse, is a brainfuck 
interpreter. I tried not to use any
dependencies that would hinder learning something interesting, and so i implemented
the lexer and parser myself, which is probably a very bad idea for anything
serious given my knowledge level. But you would need to be a special kinda
of nutjob to use brainfuck for anything serious anyways, so this is no point
of contention.

### How to compile?
Run 

```cargo build --release```


### What?
This project was not programmed with the intention of being mantained. After
i finish it i will probably be done, use it at your own risk. 
