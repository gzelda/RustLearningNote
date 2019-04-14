# Note


## 疑惑点
- **1.** 变量可以被覆盖，比如:
``` 
    let mut x: i32 = 1;
    x = 7;
    let x = x;
```  
x最终变为immutable，并且bound to 7

好奇点在书中写到原有mutable的x并没有在作用域内被销毁，需要了解一下rust对于**内存分配的机制**


