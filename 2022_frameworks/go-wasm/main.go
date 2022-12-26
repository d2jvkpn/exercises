package main

import (
	"fmt"
	"syscall/js"
)

var (
	Global   = js.Global()
	Document = Global.Get("document")
	Body     = Document.Get("body")
)

func main() {
	fmt.Println("~~~ Go WebAssembly Tutorial")

	Global.Set("myFunc", js.FuncOf(myFunc))

	hello := Document.Call("createElement", "h2")
	hello.Set("innerText", "Go WebAssembly Course")
	Body.Call("appendChild", hello)
}

func myFunc(this js.Value, inputs []js.Value) any {
	fmt.Println(">>> My Func Executed")
	return nil
}
