package log

import (
	"fmt"
	"io"
	"io/ioutil"
	"os"
	"testing"

	"github.com/stretchr/testify/require"
	"github.com/tysonmote/gommap"
)

var (
	noErr = require.NoError
)

func TestIndex(t *testing.T) {
	f, err := ioutil.TempFile(os.TempDir(), "index_test")
	fmt.Println(">>>", f.Name())
	require.NoError(t, err)
	defer os.Remove(f.Name())

	c := Config{}
	c.Segment.MaxIndexBytes = 1024
	idx, err := newIndex(f, c)
	require.NoError(t, err)
	fmt.Println("~~~", len(idx.mmap))
	_, _, err = idx.Read(-1)
	require.Error(t, err)
	require.Equal(t, f.Name(), idx.Name())

	entries := []struct {
		Off uint32
		Pos uint64
	}{
		{Off: 0, Pos: 0},
		{Off: 1, Pos: 10},
	}

	for _, want := range entries {
		err = idx.Write(want.Off, want.Pos)
		require.NoError(t, err)
		_, pos, err := idx.Read(int64(want.Off))
		require.NoError(t, err)
		require.Equal(t, want.Pos, pos)
	}
	// index and scanner should error when reading past existing entries
	_, _, err = idx.Read(int64(len(entries)))
	require.Equal(t, io.EOF, err)
	_ = idx.Close()
	// index should build its state from the existing file
	f, _ = os.OpenFile(f.Name(), os.O_RDWR, 0600)
	idx, err = newIndex(f, c)
	require.NoError(t, err)
	off, pos, err := idx.Read(-1)
	require.NoError(t, err)
	require.Equal(t, uint32(1), off)
	require.Equal(t, entries[1].Pos, pos)
}

func TestFileTruncate(t *testing.T) {
	err := os.Truncate("wk_data/a.sh", 8)
	noErr(t, err)
}

func TestGommap(t *testing.T) {
	fp := "tmp/mmap.data"
	file, err := os.Create(fp)
	noErr(t, err)
	defer file.Close()

	fi, err := os.Stat(fp)
	noErr(t, err)
	fmt.Println("~~~ 1", fi.Size())

	err = os.Truncate(file.Name(), 1024)
	noErr(t, err)

	// fi, err = os.Stat(fp)
	// noErr(t, err)
	// fmt.Println("~~~ 2", fi.Size())

	mmap, err := gommap.Map(
		file.Fd(),
		gommap.PROT_READ|gommap.PROT_WRITE,
		gommap.MAP_SHARED,
	)
	/// fmt.Println("~~~ 3", len(mmap))

	off, pos := uint32(32), uint64(64)
	enc.PutUint32(mmap[0:4], off)
	fmt.Println(">>> 2:", mmap[:32])
	err = mmap.Sync(gommap.MS_SYNC)
	noErr(t, err)

	enc.PutUint64(mmap[4:12], pos)
	err = mmap.Sync(gommap.MS_SYNC)
	noErr(t, err)
	fmt.Println(">>> 3:", mmap[:32])
}
