# 5 - Flow control

Sloth currently only as the 2 main flow control statement: **if** and **while**. The syntax is almost the same as in Rust, so an example for each should be just enough:

## If

**Syntax:**
```
if [COND] {
    ... statements ...
};
```
  
**Example:**
```
builtin strings::len;       # Allow to use the string.len() method
builtin io;                 # Allow to use the read() and print() functions


define main: -> num {
    value = read();         # Take an input string from the user

    if < value.len() 10 {
        print("Your input is less that 10 caracters!\n");
    };

    # No need for 'else' !
    if >= value.len() 10 {
        print("Your input is 10 or more caracters!\n");
    };
}
```

## While
```
while [COND] {
    ... statements ...
};
```
  
**Example:**
```
builtin io;                 # Allow to use the print() function


define main: num -> num {
    i = 0;
    while < i @0 {
        print(i "\n");
        i = + i 1;
    };
}
```