# 🦥 Sloth - The weird, slow and a bit moldy programming language

**Sloth** is an interpreted programming language, implemented in _Rust_. Its syntax is inspired by C-like languages, Python, and Lisp-like languages.  
It does not have a precise goal, except maybe existing, and it is quite slow, so you may not want to use it for something other than experimentation.

## Syntax example ([examples/factorial.slo](https://github.com/MyselfLeo/sloth/blob/master/examples/factorial.slo))
```
builtin io;

# Return the factorial of @0
define factorial: num -> num {
    @return = 1;
    i = 2;

    while <= i @0 {
        @return = * @return i;
        i = + i 1;
    };
}

define main: num -> num {
    print(factorial(@0) "\n");
}
```
The syntax, logic and specifications of Sloth are described in the documentation.

## Installation

Sloth is currently in development and cannot be downloaded yet. However, being a Rust program, it is very likely that it will be available using **[cargo](https://github.com/rust-lang/cargo)**.

## Credit
© Copyright MyselfLeo, all right reserved 🦥
