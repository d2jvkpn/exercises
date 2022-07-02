package log

import (
	"fmt"
	"io"
	"io/ioutil"
	"os"
	"strings"
	"testing"

	api "proglog/api/v1"

	"github.com/stretchr/testify/require"
	"google.golang.org/protobuf/proto"
)

func TestLog(t *testing.T) {
	for scenario, fn := range map[string]func(t *testing.T, log *Log){
		"append and read a record succeeds": testAppendRead,
		"offset out of range error":         testOutOfRangeErr,
		"init with existing segments":       testInitExisting,
		"reader":                            testReader,
		"truncate":                          testTruncate,
	} {
		t.Run(scenario, func(t *testing.T) {
			// dir, err := ioutil.TempDir("", "store-test")
			// require.NoError(t, err)
			// defer os.RemoveAll(dir)
			dir := "tmp/" + strings.Replace(scenario, " ", "_", -1)
			os.RemoveAll(dir)
			os.MkdirAll(dir, 0755)

			c := Config{}
			c.Segment.MaxStoreBytes = 32
			log, err := NewLog(dir, c)
			require.NoError(t, err)
			fn(t, log)
		})
	}
}

func TestLogAppend(t *testing.T) {
	dir := "tmp/test_append"
	os.RemoveAll(dir)
	os.MkdirAll(dir, 0755)

	c := Config{}
	c.Segment.MaxStoreBytes = 1024
	log, err := NewLog(dir, c)
	require.NoError(t, err)

	for i := 0; i < 1024; i++ {
		record := &api.Record{Value: []byte("Hello")}

		off, err := log.Append(record)
		require.NoError(t, err)
		require.Equal(t, uint64(i), off)
		fmt.Println("~~~", i, off)

		read, err := log.Read(off)
		require.NoError(t, err)
		require.Equal(t, record.Value, read.Value)
	}

	bts, err := ioutil.ReadFile("tmp/test_append/0.index")
	require.NoError(t, err)
	fmt.Println(len(bts), bts)

	bts, err = ioutil.ReadFile("tmp/test_append/0.store")
	require.NoError(t, err)
	fmt.Println(len(bts), bts)
}

func testAppendRead(t *testing.T, log *Log) {
	record := &api.Record{Value: []byte("hello world")}

	off, err := log.Append(record)
	require.NoError(t, err)
	require.Equal(t, uint64(0), off)

	read, err := log.Read(off)
	require.NoError(t, err)
	require.Equal(t, record.Value, read.Value)
}

func testOutOfRangeErr(t *testing.T, log *Log) {
	read, err := log.Read(1)
	require.Nil(t, read)
	require.Error(t, err)
}

func testInitExisting(t *testing.T, o *Log) {
	record := &api.Record{Value: []byte("hello world")}

	for i := 0; i < 3; i++ {
		_, err := o.Append(record)
		require.NoError(t, err)
	}
	require.NoError(t, o.Close())

	off, err := o.LowestOffset()
	require.NoError(t, err)
	require.Equal(t, uint64(0), off)

	off, err = o.HighestOffset()
	require.NoError(t, err)
	require.Equal(t, uint64(2), off)

	n, err := NewLog(o.Dir, o.Config)
	require.NoError(t, err)

	off, err = n.LowestOffset()
	require.NoError(t, err)
	require.Equal(t, uint64(0), off)

	off, err = n.HighestOffset()
	require.NoError(t, err)
	require.Equal(t, uint64(2), off)
}

func testReader(t *testing.T, log *Log) {
	record := &api.Record{Value: []byte("hello world")}

	off, err := log.Append(record)
	require.NoError(t, err)
	require.Equal(t, uint64(0), off)

	reader := log.Reader()
	b, err := ioutil.ReadAll(reader)
	require.NoError(t, err)

	read := &api.Record{}
	err = proto.Unmarshal(b[lenWidth:], read)
	require.NoError(t, err)
	require.Equal(t, record.Value, read.Value)
}

func testTruncate(t *testing.T, log *Log) {
	record := &api.Record{Value: []byte("hello world")}
	for i := 0; i < 3; i++ {
		_, err := log.Append(record)
		require.NoError(t, err)
	}

	err := log.Truncate(1)
	require.NoError(t, err)

	_, err = log.Read(0)
	require.Error(t, err)
}

// echo "hello" > tmp/aa.txt
// echo "world" > tmp/bb.txt
func TestMultiReader(t *testing.T) {
	f1, err := os.Open("tmp/aa.txt")
	require.NoError(t, err)

	f2, err := os.Open("tmp/bb.txt")
	require.NoError(t, err)

	reader := io.MultiReader(f1, f2)

	bts := make([]byte, 6) // 1, 3, 5, 6, 8
	n, err := reader.Read(bts)
	require.NoError(t, err)
	fmt.Println("~~~ 1", n, string(bts))

	bts = make([]byte, 6)
	n, err = reader.Read(bts)
	require.NoError(t, err)
	fmt.Println("~~~ 2", n, string(bts))
}
