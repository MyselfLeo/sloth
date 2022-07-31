# 4 - Functions and methods

Function and method definition is quite simple is Sloth. You define its name, the potential input types that it requires, and its output type. You can define methods for specific types too.

## Basic example

```
define square: num -> num {
    @return = * @0 @0;
}
```

This function returns the square of the number passed in parameter. Let's study that:
- `define square:` We define a function (not a method) named _square_. Basic.
- `num -> num` The function takes a _Number_ value as an input, and return a _Number_ value as its output. Note that there can be 0 or more input types, for multiple arguments (`num string ->` for example) but **1 output type and 1 output type only**.
- `* @0 @0` This expression compute the square of the first input. You guessed it, `@0` is the first input value. `@1` is the second, `@2` is the third, etc.
- `@return = ...` Set the value of the `@return` variable to the square. `@return` is the variable whose value will be returned and the end of the function execution. There is no 'return' statement like in other languages.
  
This function would be called like that: `square(3)`.
  
## Methods

Methods are very similar to functions, but can be defined for specific types only. Let's create a `pow` method for the type _Number_: it returns the number raised at a given integer:

```
define pow for num: num -> num {
    @return = 1;
    i = 0;
    while < i @0 {
        @return = * @return @self;
        i = + i 1;
    };
}
```

- `define pow for num:` We define a method, to be called on values of type _Number_, called _pow_.
- `num -> num` The function takes a _Number_ value as an input, and return a _Number_ value as its output.
- Note that the `@return` variable is used like any other variable. Only its value at the end of the function execution is important.
- `@self` is the value on which is called the method. If the method was called on a variable (`value.pow(3)` instead of `3.pow(3)`), then modifying the value in `@self` would also modify the value on which it was called.