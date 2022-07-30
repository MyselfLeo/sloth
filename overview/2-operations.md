# 2 - Operations

Sloth's operation are written using the [Polish notation](https://en.wikipedia.org/wiki/Polish_notation). No parenthesis are required, as each operator have a known number of operands. Here's some examples:
```
+ 1 2       ->       3
- 10 3      ->       7
/ 10 + 5 5  ->       1
! true      ->       false
```

## Operators

Sloth uses most of the common operators, but some varies. Here's a comparaison between C operators and Sloth operators:

|     C     |   +   |   -   |   *   |   /   |   >   |   <   |   >=   |   <=   |   &&  |  \|\|  |   !   |
|:---------:|:-----:|:-----:|:-----:|:-----:|:-----:|:-----:|:------:|:------:|:-----:|:------:|:-----:|
| **Sloth** | **+** | **-** | **\*** | **/** | **>** | **<** | **>=** | **<=** | **&** | **\|** | **!** |


## Limitations

As of now, blending operations and function calls can get messy. Support of parenthesis to 'clean' your operations is planned, but for now, you can use temporary variables to divide your operations.