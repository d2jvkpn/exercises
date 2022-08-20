// pi.go
package main

import (
	"fmt"
)

func main() {
	var π = 22 / 7.0
	fmt.Println(π)
}

/*
3.142857142857143

- π is valid variable name as it's a unicode identifier
- 22 will be compiled as float rather than an integer
- the flollowing won't comile, a is treated as an integer
```go
  a, b := 22, 7.0
  var π = a / b
```
*/
