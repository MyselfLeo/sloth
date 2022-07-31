# 6 - Builtins
  
In almost every examples so far, you have seen the `builtin` keyword. What is it?
  
Builtins are functions and methods, written in Rust (not in Sloth, that's why they're _"builtin"_), that you can add to the function scope of the program. They are grouped in _modules_, like `io`, `strings`, `numbers`, etc.  
You can import everything from a module (`builtin io;`) or only an element of it (`builtin io:print;`).
  
Note that some modules are imported by default. One in particular is the `lists` module, containing the methods required to work with the `list` type (more on that later). However, you can ask the interpreter to not import them, with the `--nodefault` argument. You can use the `--functions` argument to get a list of every functions in the program scope after building it. Here's an example with a simple [Hello, world!](../examples/hello_world.slo):

### Without --nodefault
```
$ slothlang hello_world.slo --functions
```
```
FUNCTION NAME            OWNER TYPE     MODULE         INPUT TYPES              OUTPUT TYPE    
main                     -              -                                       num            
print                    -              io             -                        num            
read                     -              io             -                        string         
get                      list[any]      lists          -                        any            
len                      list[any]      lists          -                        num            
pull                     list[any]      lists          -                        any            
push                     list[any]      lists          -                        num            
set                      list[any]      lists          -                        num
```

### With --nodefault
```
$ slothlang hello_world.slo --functions --nodefault
```
```
FUNCTION NAME            OWNER TYPE     MODULE         INPUT TYPES              OUTPUT TYPE    
main                     -              -                                       num            
print                    -              io             -                        num            
read                     -              io             -                        string
```


## List of builtin modules

WIP