package internal

import (
	"fmt"
	"path/filepath"
	"testing"
)

func TestRE(t *testing.T) {
	if _FilenameRE.Match([]byte("Hello, world!")) {
		t.Fatal("1")
	}

	if !_FilenameRE.Match([]byte("..")) {
		t.Fatal("2")
	}
}

func TestFilePath(t *testing.T) {
	fmt.Println(filepath.Ext("a.tar.gz"))

	fmt.Println(filepath.Ext("a"))

	fmt.Println(filepath.Ext("..."))
}
