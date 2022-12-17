# 🦥 Sloth - The weird, slow and a bit moldy programming language

**Sloth** is an interpreted programming language, implemented in _Rust_. Its syntax is inspired by C-like languages, Rust, Python, and Lisp-like languages.  
I do not ensure the stability of the language: **use it at your own risk!**

## Syntax example
```
# Return the factorial of @0
define factorial: num -> num {
    @return = 1;
    i = 2;

    while <= i @0 {
        @return = * @return i;
        i = + i 1;
    };
}

define @main: num -> num {
    print(factorial(@0) "\n");
}
```

Some fundamentals of the Sloth syntax:
- Operations use the [Polish notation](https://en.wikipedia.org/wiki/Polish_notation)
- Special symbols are prefixed with `@`: `@main`, `@self`, `@return`...
- Functions return the content of the variable `@return`
- Methods can modify the value referenced by `@self`
- Functions/Methods arguments are named `@0`, `@1`, etc.

## Features

Sloth does not provide common features like `else` blocks, `for` loops, etc. There is no error management yet, no compiled module yet, etc.

However, you can overload operators! 🎉


## Installation

Install using **[cargo](https://github.com/rust-lang/cargo)**:
```
$ cargo install slothlang
```

I may provide executables in the future.

## License

Sloth is licensed under the [Apache-2.0 License](LICENSE.txt).