package main

import (
	"fmt"
	"reflect"
	"strings"
)

func main() {
	//
	var (
		a int32  = 42
		b string = "forty two"
	)

	fmt.Println(reflect.TypeOf(a), reflect.TypeOf(b))

	//
	t := T{42, "forty two"}
	typeT := reflect.TypeOf(t)
	fmt.Println(typeT)

	for i := 0; i < typeT.NumField(); i++ {
		field := typeT.Field(i)
		fmt.Println("~~~ 1", field.Name, field.Type, field)
	}

	valueT := reflect.ValueOf(t)
	fmt.Println(valueT.Kind(), valueT)

	for i := 0; i < valueT.NumField(); i++ {
		field := valueT.Field(i)
		fmt.Println("~~~ 2", field.Kind(), field)
	}

	//
	var ptrAdder *Adder
	adderType := reflect.TypeOf(ptrAdder).Elem()
	c := Calculator{}
	calcType := reflect.TypeOf(c)
	calcTypePtr := reflect.TypeOf(&c)

	fmt.Println("~~~", reflect.Struct, reflect.Ptr)
	fmt.Println(calcType, calcType.Kind(), calcType.Implements(adderType))
	fmt.Println(calcTypePtr, calcTypePtr.Kind(), calcTypePtr.Implements(adderType))
}

type T struct {
	A int32
	B string
}

type Adder interface {
	Add(int, int) int
}

type Calculator struct{}

func (c *Calculator) Add(a int, b int) int {
	return a + b
}

func ValuePrint(i interface{}) {
	v := reflect.ValueOf(i)

	switch v.Kind() {
	case reflect.Int32:
		fmt.Println("Int32 with value", v.Int())
	case reflect.String:
		fmt.Println("String with value", v.String())
	default:
		fmt.Println("unknown type")
	}
}
