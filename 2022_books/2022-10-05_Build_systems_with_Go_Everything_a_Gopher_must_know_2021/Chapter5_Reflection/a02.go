package main

import (
	"fmt"
	"reflect"
	"strings"
)

type T struct {
	A string `tag1:"a" tag2:"AA"`
	B int32  `tag1:"b" tag2:"BB"`
	c string
}

func main() {
	t := T{"hello", 42, "bye"}
	fmt.Println(t)

	//
	valueT := reflect.ValueOf(&t).Elem() // !!

	for i := 0; i < valueT.NumField(); i++ {
		f := valueT.Field(i)
		if f.Kind() != reflect.String || !f.CanSet() {
			continue
		}

		current := f.String()
		f.SetString(strings.ToUpper(current))

	}
	fmt.Println(t)

	//
	typeT := reflect.TypeOf(t)
	for i := 0; i < typeT.NumField(); i++ {
		f := typeT.Field(i)
		t := f.Tag
		v, _ := t.Lookup("tag1")
		fmt.Printf("filed %q tag is: %s\n    tag1: %s\n", f.Name, t, v)
	}

	fA, _ := typeT.FieldByName("A")
	fmt.Println("~~~", fA.Tag.Get("tag2"))
}

/*
- Reflection goes from interface value to reflection object.
- Reflection goes from reflection object to interface value.
- To modify a reflection object, the value must be settable.
*/
