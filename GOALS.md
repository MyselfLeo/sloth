# nSloth: For real this time

## Goals:
- Better built-in function calls with macros (example: call_built_in!(name), using crate _paste_)
- Something like an _Object_ trait to be used by classes for example
- Global heap for functions and objects, each with a unique ID stored by scopes in hashmaps (name -> id). Then request global heap the object/function with given id
- Better parser: each Token must have a **start** and **end** coordinate (line & column)