package internal

import (
	"fmt"
	"testing"
)

func TestNewClientId(t *testing.T) {
	id := newClientId()
	fmt.Println("~~~", id)

	id = newClientId()
	fmt.Println("~~~", id)
}
