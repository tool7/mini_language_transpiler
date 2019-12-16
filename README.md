# *"mini language"* transpiler
***
### Transpiler from a simple programming language to Rust source code, based on [LLVM tutorial](https://github.com/jauhien/iron-kaleidoscope).
This is not a fully implemented programming language nor intended for usage. The language only contains several basic constructs. Let's call it a *"mini language"*. Transpiler converts those constructs to Rust code which can then be built and executed.
Motivation behind the project is strictly for my personal educational purpose regarding general processes that go into implementation of programming languages.

***
### Transpiler usage:
1. Install [Rust](https://www.rust-lang.org/).
2. Download the project.
3. Project folder contains **./example** folder where **input.txt** is located. That file contains a few commands written in a *"mini language"*.
4. Navigate to project directory using terminal and run **cargo run** command.
5. Program will output a file called **output.<span></span>rs** to the same **./example** folder. The file will contain Rust code generated from simple instructions written in a *"mini language"*.
6. If the syntax is correct, build and run **output.<span></span>rs** program using **rustc** or **cargo** commands.
***

### Example code transpile:
![screenshots](https://github.com/tool7/mini_language_transpiler/blob/master/images/screenshot-input-output.png?raw=true)
