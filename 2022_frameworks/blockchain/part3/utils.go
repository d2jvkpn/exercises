package main

import (
	"bytes"
	"encoding/binary"
)

func IntToHex[T int64 | int](num T) []byte {
	buff := new(bytes.Buffer)
	_ = binary.Write(buff, binary.BigEndian, int64(num)) // ignore error

	return buff.Bytes()
}
