# 2 - Types

Sloth being an interpreted language, its types are quite simple:

| Type name   | Type identifier | Examples                                                                 | Default value |
|-------------|-----------------|--------------------------------------------------------------------------|---------------|
| Boolean     | bool            | **true**, **false**                                                      | false         |
| Number      | num             | **1**, **1.3**, **-45**, **3.14519**                                     | 0.0           |
| String      | string          | **"Hello, world!"**, **""**, **"E"**                                     | ""            |
| Lists[type] | list[type]      | **[1 2 3 4 5]** _(list[num])_, **[\["a"] ["b"]]** _(list[list[string]])_ | []            |

## Lists

As you can see, lists are defined with the type of their content. Unlike Python, your list can't contain values of different type.