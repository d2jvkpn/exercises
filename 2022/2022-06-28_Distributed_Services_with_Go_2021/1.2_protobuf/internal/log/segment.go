package log

import (
	"fmt"
	"os"
	"path"

	api "x/api/v1"

	"google.golang.org/protobuf/proto"
)

type segment struct {
	store      *store
	index      *index
	baseOffset uint64
	nextOffset uint64
	config     Config
}

func newSegment(dir string, baseOffset uint64, c Config) (seg *segment, err error) {
	var (
		off       uint32
		storeFile *os.File
		indexFile *os.File
	)

	seg = &segment{baseOffset: baseOffset, config: c}

	if storeFile, err = os.OpenFile(
		path.Join(dir, fmt.Sprintf("%d%s", baseOffset, ".store")),
		os.O_RDWR|os.O_CREATE|os.O_APPEND, 0644,
	); err != nil {
		return nil, err
	}

	if seg.store, err = newStore(storeFile); err != nil {
		return nil, err
	}

	if indexFile, err = os.OpenFile(
		path.Join(dir, fmt.Sprintf("%d%s", baseOffset, ".index")),
		os.O_RDWR|os.O_CREATE, 0644,
	); err != nil {
		return nil, err
	}

	if seg.index, err = newIndex(indexFile, c); err != nil {
		return nil, err
	}

	if off, _, err = seg.index.Read(-1); err != nil { // io.EOF
		seg.nextOffset = baseOffset
	} else {
		seg.nextOffset = baseOffset + uint64(off) + 1
	}

	return seg, nil
}

func (s *segment) Append(record *api.Record) (offset uint64, err error) {
	var (
		pos uint64
		bts []byte
	)

	offset = s.nextOffset
	record.Offset = offset

	if bts, err = proto.Marshal(record); err != nil {
		return 0, err
	}

	if _, pos, err = s.store.Append(bts); err != nil {
		return 0, err
	}

	// index offsets are relative to base offset
	err = s.index.Write(uint32(s.nextOffset-uint64(s.baseOffset)), pos)
	if err != nil {
		return 0, err
	}
	s.nextOffset++

	return offset, nil
}

func (s *segment) IsMaxed() bool {
	t1 := s.store.size >= s.config.Segment.MaxStoreBytes
	t2 := s.index.size >= s.config.Segment.MaxIndexBytes
	return t1 || t2
}

func (s *segment) Remove() (err error) {
	if err = s.Close(); err != nil {
		return err
	}

	if err = os.Remove(s.index.Name()); err != nil {
		return err
	}

	if err = os.Remove(s.store.Name()); err != nil {
		return err
	}

	return nil
}

func (s *segment) Close() (err error) {
	if err = s.index.Close(); err != nil {
		return err
	}

	if err = s.store.Close(); err != nil {
		return err
	}

	return nil
}

func (s *segment) Read(off uint64) (record *api.Record, err error) {
	var (
		pos uint64
		bts []byte
	)

	if _, pos, err = s.index.Read(int64(off - s.baseOffset)); err != nil {
		return nil, err
	}

	if bts, err = s.store.Read(pos); err != nil {
		return nil, err
	}

	record = &api.Record{}
	if err = proto.Unmarshal(bts, record); err != nil {
		return nil, err
	}

	return record, nil
}
