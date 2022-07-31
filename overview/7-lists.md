# 7 - Lists

Sloth as a builtin type, or more precisely a _family_ of types called `list`. In reality, this type varies with the content of the list.  
In example, a list of numbers will have the `list[num]` type. A list of lists of strings will have the type `list[list[string]]`, etc.  
Note that unlike Python, a list can't hold values of different type.
  
## Creating a list
Lists can be created with a similar syntax to Python, without commas:
```
list_of_num = [1 2 3 4];
list_of_string = ["Hello" "," "world!"];
```

## Working with lists

In order to work with lists, you need to use some builtins. Thanksfully, those are imported by default to any program. You don't have to import them yourself!

> Note: In the following examples, the main function definition and builtin imports have being removed for clarity. Those program won't run in that state.

### Set

Set the value at the given index. If the index is higher than the length of the list, it just appends the value:

```
l = ["A" "B" "E" "D"];
l.set(2 "C");
l.set(10, "R");
print(l "\n");
```
```
>>> [A B C D R]
```

### Get

Return the value at the given index

```
l = ["A" "B" "C" "D"];
l.get(3);
print(l "\n");
```
```
>>> D
```

### Push

Add a new value to the list

```
l = ["A" "B" "C" "D"];
l.push("E");
print(l "\n");
```
```
>>> [A B C D E]
```

### Pull

Remove a value from the list at the given index, returning the removed value

```
l = ["A" "B" "C" "D" "R"];
removed = l.pull(3);
print(l "\n");
print(removed "\n");
```
```
>>> [A B C R]
    D
```