// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.27.1-devel
// 	protoc        v3.21.2
// source: api/v1/log.proto

package log_v1

import (
	protoreflect "google.golang.org/protobuf/reflect/protoreflect"
	protoimpl "google.golang.org/protobuf/runtime/protoimpl"
	reflect "reflect"
	sync "sync"
)

const (
	// Verify that this generated code is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(20 - protoimpl.MinVersion)
	// Verify that runtime/protoimpl is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(protoimpl.MaxVersion - 20)
)

type Record struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Value  []byte `protobuf:"bytes,1,opt,name=value,proto3" json:"value,omitempty"`
	Offset uint64 `protobuf:"varint,2,opt,name=offset,proto3" json:"offset,omitempty"`
}

func (x *Record) Reset() {
	*x = Record{}
	if protoimpl.UnsafeEnabled {
		mi := &file_api_v1_log_proto_msgTypes[0]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *Record) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*Record) ProtoMessage() {}

func (x *Record) ProtoReflect() protoreflect.Message {
	mi := &file_api_v1_log_proto_msgTypes[0]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use Record.ProtoReflect.Descriptor instead.
func (*Record) Descriptor() ([]byte, []int) {
	return file_api_v1_log_proto_rawDescGZIP(), []int{0}
}

func (x *Record) GetValue() []byte {
	if x != nil {
		return x.Value
	}
	return nil
}

func (x *Record) GetOffset() uint64 {
	if x != nil {
		return x.Offset
	}
	return 0
}

var File_api_v1_log_proto protoreflect.FileDescriptor

var file_api_v1_log_proto_rawDesc = []byte{
	0x0a, 0x10, 0x61, 0x70, 0x69, 0x2f, 0x76, 0x31, 0x2f, 0x6c, 0x6f, 0x67, 0x2e, 0x70, 0x72, 0x6f,
	0x74, 0x6f, 0x12, 0x06, 0x6c, 0x6f, 0x67, 0x2e, 0x76, 0x31, 0x22, 0x36, 0x0a, 0x06, 0x52, 0x65,
	0x63, 0x6f, 0x72, 0x64, 0x12, 0x14, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x01, 0x20,
	0x01, 0x28, 0x0c, 0x52, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x12, 0x16, 0x0a, 0x06, 0x6f, 0x66,
	0x66, 0x73, 0x65, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x06, 0x6f, 0x66, 0x66, 0x73,
	0x65, 0x74, 0x42, 0x0e, 0x5a, 0x0c, 0x78, 0x2f, 0x61, 0x70, 0x69, 0x2f, 0x6c, 0x6f, 0x67, 0x5f,
	0x76, 0x31, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
}

var (
	file_api_v1_log_proto_rawDescOnce sync.Once
	file_api_v1_log_proto_rawDescData = file_api_v1_log_proto_rawDesc
)

func file_api_v1_log_proto_rawDescGZIP() []byte {
	file_api_v1_log_proto_rawDescOnce.Do(func() {
		file_api_v1_log_proto_rawDescData = protoimpl.X.CompressGZIP(file_api_v1_log_proto_rawDescData)
	})
	return file_api_v1_log_proto_rawDescData
}

var file_api_v1_log_proto_msgTypes = make([]protoimpl.MessageInfo, 1)
var file_api_v1_log_proto_goTypes = []interface{}{
	(*Record)(nil), // 0: log.v1.Record
}
var file_api_v1_log_proto_depIdxs = []int32{
	0, // [0:0] is the sub-list for method output_type
	0, // [0:0] is the sub-list for method input_type
	0, // [0:0] is the sub-list for extension type_name
	0, // [0:0] is the sub-list for extension extendee
	0, // [0:0] is the sub-list for field type_name
}

func init() { file_api_v1_log_proto_init() }
func file_api_v1_log_proto_init() {
	if File_api_v1_log_proto != nil {
		return
	}
	if !protoimpl.UnsafeEnabled {
		file_api_v1_log_proto_msgTypes[0].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*Record); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
	}
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: file_api_v1_log_proto_rawDesc,
			NumEnums:      0,
			NumMessages:   1,
			NumExtensions: 0,
			NumServices:   0,
		},
		GoTypes:           file_api_v1_log_proto_goTypes,
		DependencyIndexes: file_api_v1_log_proto_depIdxs,
		MessageInfos:      file_api_v1_log_proto_msgTypes,
	}.Build()
	File_api_v1_log_proto = out.File
	file_api_v1_log_proto_rawDesc = nil
	file_api_v1_log_proto_goTypes = nil
	file_api_v1_log_proto_depIdxs = nil
}
