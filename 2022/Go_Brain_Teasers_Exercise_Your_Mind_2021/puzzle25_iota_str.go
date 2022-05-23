// iota_str.go
package main

import (
	"encoding/json"
	"fmt"
	"strings"
)

type FilePerm uint16 // 16 flags are enough

const (
	Read FilePerm = 1 << iota
	Write
	Execute
)

// String implements fmt.Stringer interface
func (p FilePerm) String() string {
	switch p {
	case Read:
		return "read"
	case Write:
		return "write"
	case Execute:
		return "execute"
	}

	return fmt.Sprintf("unknown FilePerm number: %d", p) // don't use %s here :)
}

func (p FilePerm) MarshalJSON() (bts []byte, err error) {
	str := p.String()
	if strings.HasPrefix(str, "unknown") {
		return nil, fmt.Errorf(str)
	}

	return []byte(`"` + str + `"`), nil
}

func (p *FilePerm) UnmarshalJSON(data []byte) (err error) {
	str := string(data)

	switch str {
	case "read":
		*p = Read
	case `"write"`:
		*p = Write
	case `"execute"`:
		*p = Execute
	default:
		return fmt.Errorf("unknown FilePerm string: %s", str)
	}

	fmt.Printf("~~~ %d\n", p) // !?!? 824633803064

	return nil
}

func main() {
	fmt.Printf("%[1]s, %[1]d, %s\n", Execute, Execute.String()) // execute, 4, execute

	type Data struct {
		P FilePerm `json:"p"`
	}

	d := Data{Read}
	if bts, err := json.Marshal(d); err != nil {
		fmt.Println(">>> 1 error:", err)
	} else {
		fmt.Println(">>> 1 result:", string(bts))
	}

	var d2 Data
	if err := json.Unmarshal([]byte(`{"p": "execute"}`), &d); err != nil {
		fmt.Println(">>> 2 error:", err)
	} else {
		fmt.Println(">>> 2 result:", d2.P) // >>> 2 result: unknown FilePerm number: 0
	}
}
