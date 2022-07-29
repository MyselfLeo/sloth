# 1 - Introduction

**Sloth** is an interpreted programming language implemented in _Rust_. As you'll see, its syntax is not the best and its performance is not the best either.  
Sloth programs uses the extension `.slo`.

## How to run a program

Quick answer:
```
$ sloth [OPTIONS] <PROGRAM PATH> [PROGRAM ARGUMENTS]
```

You can get the list of options available using `sloth --help`.


## Base of every program

Every program in Sloth **must** have a `main` function:
```
define main: -> num {
    [your code here]
}
```

The `num` here is the return type of the function. We'll see it after, but for now, remember that **the main function is required to return a Number (num) value.**  
However, its inputs can vary based on what you need. Compared to other programming languages, you don't need to parse the command-line inputs yourself: the interpreter does it for you. Example:

```
define main: num -> num {
    print(@0 "\n");
}
```

<details>
<summary>Result</summary>
<p>
```
$ sloth input.slo
>>> INVALID ARGUMENTS: Given 0 command-line argument(s), but the main function requires 1 argument(s):  num

$ sloth input.slo 12
>>> 12

$ sloth input.slo "test
>>> INVALID ARGUMENTS: Error while parsing command-line arguments: Cannot convert 'test' into a Number value
```
</p>
</details>