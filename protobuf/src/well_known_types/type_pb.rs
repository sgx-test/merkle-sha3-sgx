// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct Type {
    // message fields
    pub name: ::std::string::String,
    pub fields: ::protobuf::RepeatedField<Field>,
    pub oneofs: ::protobuf::RepeatedField<::std::string::String>,
    pub options: ::protobuf::RepeatedField<Option>,
    pub source_context: ::protobuf::SingularPtrField<::protobuf::well_known_types::SourceContext>,
    pub syntax: Syntax,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Type {}

impl Type {
    pub fn new() -> Type {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Type {
        static mut instance: ::protobuf::lazy::Lazy<Type> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Type,
        };
        unsafe {
            instance.get(Type::new)
        }
    }

    // string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // repeated .google.protobuf.Field fields = 2;

    pub fn clear_fields(&mut self) {
        self.fields.clear();
    }

    // Param is passed by value, moved
    pub fn set_fields(&mut self, v: ::protobuf::RepeatedField<Field>) {
        self.fields = v;
    }

    // Mutable pointer to the field.
    pub fn mut_fields(&mut self) -> &mut ::protobuf::RepeatedField<Field> {
        &mut self.fields
    }

    // Take field
    pub fn take_fields(&mut self) -> ::protobuf::RepeatedField<Field> {
        ::std::mem::replace(&mut self.fields, ::protobuf::RepeatedField::new())
    }

    pub fn get_fields(&self) -> &[Field] {
        &self.fields
    }

    fn get_fields_for_reflect(&self) -> &::protobuf::RepeatedField<Field> {
        &self.fields
    }

    fn mut_fields_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Field> {
        &mut self.fields
    }

    // repeated string oneofs = 3;

    pub fn clear_oneofs(&mut self) {
        self.oneofs.clear();
    }

    // Param is passed by value, moved
    pub fn set_oneofs(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.oneofs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_oneofs(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.oneofs
    }

    // Take field
    pub fn take_oneofs(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.oneofs, ::protobuf::RepeatedField::new())
    }

    pub fn get_oneofs(&self) -> &[::std::string::String] {
        &self.oneofs
    }

    fn get_oneofs_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.oneofs
    }

    fn mut_oneofs_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.oneofs
    }

    // repeated .google.protobuf.Option options = 4;

    pub fn clear_options(&mut self) {
        self.options.clear();
    }

    // Param is passed by value, moved
    pub fn set_options(&mut self, v: ::protobuf::RepeatedField<Option>) {
        self.options = v;
    }

    // Mutable pointer to the field.
    pub fn mut_options(&mut self) -> &mut ::protobuf::RepeatedField<Option> {
        &mut self.options
    }

    // Take field
    pub fn take_options(&mut self) -> ::protobuf::RepeatedField<Option> {
        ::std::mem::replace(&mut self.options, ::protobuf::RepeatedField::new())
    }

    pub fn get_options(&self) -> &[Option] {
        &self.options
    }

    fn get_options_for_reflect(&self) -> &::protobuf::RepeatedField<Option> {
        &self.options
    }

    fn mut_options_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Option> {
        &mut self.options
    }

    // .google.protobuf.SourceContext source_context = 5;

    pub fn clear_source_context(&mut self) {
        self.source_context.clear();
    }

    pub fn has_source_context(&self) -> bool {
        self.source_context.is_some()
    }

    // Param is passed by value, moved
    pub fn set_source_context(&mut self, v: ::protobuf::well_known_types::SourceContext) {
        self.source_context = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_source_context(&mut self) -> &mut ::protobuf::well_known_types::SourceContext {
        if self.source_context.is_none() {
            self.source_context.set_default();
        }
        self.source_context.as_mut().unwrap()
    }

    // Take field
    pub fn take_source_context(&mut self) -> ::protobuf::well_known_types::SourceContext {
        self.source_context.take().unwrap_or_else(|| ::protobuf::well_known_types::SourceContext::new())
    }

    pub fn get_source_context(&self) -> &::protobuf::well_known_types::SourceContext {
        self.source_context.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::SourceContext::default_instance())
    }

    fn get_source_context_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::SourceContext> {
        &self.source_context
    }

    fn mut_source_context_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::SourceContext> {
        &mut self.source_context
    }

    // .google.protobuf.Syntax syntax = 6;

    pub fn clear_syntax(&mut self) {
        self.syntax = Syntax::SYNTAX_PROTO2;
    }

    // Param is passed by value, moved
    pub fn set_syntax(&mut self, v: Syntax) {
        self.syntax = v;
    }

    pub fn get_syntax(&self) -> Syntax {
        self.syntax
    }

    fn get_syntax_for_reflect(&self) -> &Syntax {
        &self.syntax
    }

    fn mut_syntax_for_reflect(&mut self) -> &mut Syntax {
        &mut self.syntax
    }
}

impl ::protobuf::Message for Type {
    fn is_initialized(&self) -> bool {
        for v in &self.fields {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.options {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.source_context {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.fields)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.oneofs)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.options)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.source_context)?;
                },
                6 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.syntax, 6, &mut self.unknown_fields)?
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        for value in &self.fields {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.oneofs {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        for value in &self.options {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.source_context.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.syntax != Syntax::SYNTAX_PROTO2 {
            my_size += ::protobuf::rt::enum_size(6, self.syntax);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        for v in &self.fields {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.oneofs {
            os.write_string(3, &v)?;
        };
        for v in &self.options {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.source_context.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.syntax != Syntax::SYNTAX_PROTO2 {
            os.write_enum(6, self.syntax.value())?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Type {
    fn new() -> Type {
        Type::new()
    }

    fn descriptor_static(_: ::std::option::Option<Type>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    Type::get_name_for_reflect,
                    Type::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Field>>(
                    "fields",
                    Type::get_fields_for_reflect,
                    Type::mut_fields_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "oneofs",
                    Type::get_oneofs_for_reflect,
                    Type::mut_oneofs_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Option>>(
                    "options",
                    Type::get_options_for_reflect,
                    Type::mut_options_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::SourceContext>>(
                    "source_context",
                    Type::get_source_context_for_reflect,
                    Type::mut_source_context_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Syntax>>(
                    "syntax",
                    Type::get_syntax_for_reflect,
                    Type::mut_syntax_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Type>(
                    "Type",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Type {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_fields();
        self.clear_oneofs();
        self.clear_options();
        self.clear_source_context();
        self.clear_syntax();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Type {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Type {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Field {
    // message fields
    pub kind: Field_Kind,
    pub cardinality: Field_Cardinality,
    pub number: i32,
    pub name: ::std::string::String,
    pub type_url: ::std::string::String,
    pub oneof_index: i32,
    pub packed: bool,
    pub options: ::protobuf::RepeatedField<Option>,
    pub json_name: ::std::string::String,
    pub default_value: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Field {}

impl Field {
    pub fn new() -> Field {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Field {
        static mut instance: ::protobuf::lazy::Lazy<Field> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Field,
        };
        unsafe {
            instance.get(Field::new)
        }
    }

    // .google.protobuf.Field.Kind kind = 1;

    pub fn clear_kind(&mut self) {
        self.kind = Field_Kind::TYPE_UNKNOWN;
    }

    // Param is passed by value, moved
    pub fn set_kind(&mut self, v: Field_Kind) {
        self.kind = v;
    }

    pub fn get_kind(&self) -> Field_Kind {
        self.kind
    }

    fn get_kind_for_reflect(&self) -> &Field_Kind {
        &self.kind
    }

    fn mut_kind_for_reflect(&mut self) -> &mut Field_Kind {
        &mut self.kind
    }

    // .google.protobuf.Field.Cardinality cardinality = 2;

    pub fn clear_cardinality(&mut self) {
        self.cardinality = Field_Cardinality::CARDINALITY_UNKNOWN;
    }

    // Param is passed by value, moved
    pub fn set_cardinality(&mut self, v: Field_Cardinality) {
        self.cardinality = v;
    }

    pub fn get_cardinality(&self) -> Field_Cardinality {
        self.cardinality
    }

    fn get_cardinality_for_reflect(&self) -> &Field_Cardinality {
        &self.cardinality
    }

    fn mut_cardinality_for_reflect(&mut self) -> &mut Field_Cardinality {
        &mut self.cardinality
    }

    // int32 number = 3;

    pub fn clear_number(&mut self) {
        self.number = 0;
    }

    // Param is passed by value, moved
    pub fn set_number(&mut self, v: i32) {
        self.number = v;
    }

    pub fn get_number(&self) -> i32 {
        self.number
    }

    fn get_number_for_reflect(&self) -> &i32 {
        &self.number
    }

    fn mut_number_for_reflect(&mut self) -> &mut i32 {
        &mut self.number
    }

    // string name = 4;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // string type_url = 6;

    pub fn clear_type_url(&mut self) {
        self.type_url.clear();
    }

    // Param is passed by value, moved
    pub fn set_type_url(&mut self, v: ::std::string::String) {
        self.type_url = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_type_url(&mut self) -> &mut ::std::string::String {
        &mut self.type_url
    }

    // Take field
    pub fn take_type_url(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.type_url, ::std::string::String::new())
    }

    pub fn get_type_url(&self) -> &str {
        &self.type_url
    }

    fn get_type_url_for_reflect(&self) -> &::std::string::String {
        &self.type_url
    }

    fn mut_type_url_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.type_url
    }

    // int32 oneof_index = 7;

    pub fn clear_oneof_index(&mut self) {
        self.oneof_index = 0;
    }

    // Param is passed by value, moved
    pub fn set_oneof_index(&mut self, v: i32) {
        self.oneof_index = v;
    }

    pub fn get_oneof_index(&self) -> i32 {
        self.oneof_index
    }

    fn get_oneof_index_for_reflect(&self) -> &i32 {
        &self.oneof_index
    }

    fn mut_oneof_index_for_reflect(&mut self) -> &mut i32 {
        &mut self.oneof_index
    }

    // bool packed = 8;

    pub fn clear_packed(&mut self) {
        self.packed = false;
    }

    // Param is passed by value, moved
    pub fn set_packed(&mut self, v: bool) {
        self.packed = v;
    }

    pub fn get_packed(&self) -> bool {
        self.packed
    }

    fn get_packed_for_reflect(&self) -> &bool {
        &self.packed
    }

    fn mut_packed_for_reflect(&mut self) -> &mut bool {
        &mut self.packed
    }

    // repeated .google.protobuf.Option options = 9;

    pub fn clear_options(&mut self) {
        self.options.clear();
    }

    // Param is passed by value, moved
    pub fn set_options(&mut self, v: ::protobuf::RepeatedField<Option>) {
        self.options = v;
    }

    // Mutable pointer to the field.
    pub fn mut_options(&mut self) -> &mut ::protobuf::RepeatedField<Option> {
        &mut self.options
    }

    // Take field
    pub fn take_options(&mut self) -> ::protobuf::RepeatedField<Option> {
        ::std::mem::replace(&mut self.options, ::protobuf::RepeatedField::new())
    }

    pub fn get_options(&self) -> &[Option] {
        &self.options
    }

    fn get_options_for_reflect(&self) -> &::protobuf::RepeatedField<Option> {
        &self.options
    }

    fn mut_options_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Option> {
        &mut self.options
    }

    // string json_name = 10;

    pub fn clear_json_name(&mut self) {
        self.json_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_json_name(&mut self, v: ::std::string::String) {
        self.json_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_json_name(&mut self) -> &mut ::std::string::String {
        &mut self.json_name
    }

    // Take field
    pub fn take_json_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.json_name, ::std::string::String::new())
    }

    pub fn get_json_name(&self) -> &str {
        &self.json_name
    }

    fn get_json_name_for_reflect(&self) -> &::std::string::String {
        &self.json_name
    }

    fn mut_json_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.json_name
    }

    // string default_value = 11;

    pub fn clear_default_value(&mut self) {
        self.default_value.clear();
    }

    // Param is passed by value, moved
    pub fn set_default_value(&mut self, v: ::std::string::String) {
        self.default_value = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_default_value(&mut self) -> &mut ::std::string::String {
        &mut self.default_value
    }

    // Take field
    pub fn take_default_value(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.default_value, ::std::string::String::new())
    }

    pub fn get_default_value(&self) -> &str {
        &self.default_value
    }

    fn get_default_value_for_reflect(&self) -> &::std::string::String {
        &self.default_value
    }

    fn mut_default_value_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.default_value
    }
}

impl ::protobuf::Message for Field {
    fn is_initialized(&self) -> bool {
        for v in &self.options {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.kind, 1, &mut self.unknown_fields)?
                },
                2 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.cardinality, 2, &mut self.unknown_fields)?
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.number = tmp;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.type_url)?;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.oneof_index = tmp;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.packed = tmp;
                },
                9 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.options)?;
                },
                10 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.json_name)?;
                },
                11 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.default_value)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.kind != Field_Kind::TYPE_UNKNOWN {
            my_size += ::protobuf::rt::enum_size(1, self.kind);
        }
        if self.cardinality != Field_Cardinality::CARDINALITY_UNKNOWN {
            my_size += ::protobuf::rt::enum_size(2, self.cardinality);
        }
        if self.number != 0 {
            my_size += ::protobuf::rt::value_size(3, self.number, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.name);
        }
        if !self.type_url.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.type_url);
        }
        if self.oneof_index != 0 {
            my_size += ::protobuf::rt::value_size(7, self.oneof_index, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.packed != false {
            my_size += 2;
        }
        for value in &self.options {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if !self.json_name.is_empty() {
            my_size += ::protobuf::rt::string_size(10, &self.json_name);
        }
        if !self.default_value.is_empty() {
            my_size += ::protobuf::rt::string_size(11, &self.default_value);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.kind != Field_Kind::TYPE_UNKNOWN {
            os.write_enum(1, self.kind.value())?;
        }
        if self.cardinality != Field_Cardinality::CARDINALITY_UNKNOWN {
            os.write_enum(2, self.cardinality.value())?;
        }
        if self.number != 0 {
            os.write_int32(3, self.number)?;
        }
        if !self.name.is_empty() {
            os.write_string(4, &self.name)?;
        }
        if !self.type_url.is_empty() {
            os.write_string(6, &self.type_url)?;
        }
        if self.oneof_index != 0 {
            os.write_int32(7, self.oneof_index)?;
        }
        if self.packed != false {
            os.write_bool(8, self.packed)?;
        }
        for v in &self.options {
            os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if !self.json_name.is_empty() {
            os.write_string(10, &self.json_name)?;
        }
        if !self.default_value.is_empty() {
            os.write_string(11, &self.default_value)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Field {
    fn new() -> Field {
        Field::new()
    }

    fn descriptor_static(_: ::std::option::Option<Field>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Field_Kind>>(
                    "kind",
                    Field::get_kind_for_reflect,
                    Field::mut_kind_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Field_Cardinality>>(
                    "cardinality",
                    Field::get_cardinality_for_reflect,
                    Field::mut_cardinality_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "number",
                    Field::get_number_for_reflect,
                    Field::mut_number_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    Field::get_name_for_reflect,
                    Field::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "type_url",
                    Field::get_type_url_for_reflect,
                    Field::mut_type_url_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "oneof_index",
                    Field::get_oneof_index_for_reflect,
                    Field::mut_oneof_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "packed",
                    Field::get_packed_for_reflect,
                    Field::mut_packed_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Option>>(
                    "options",
                    Field::get_options_for_reflect,
                    Field::mut_options_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "json_name",
                    Field::get_json_name_for_reflect,
                    Field::mut_json_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "default_value",
                    Field::get_default_value_for_reflect,
                    Field::mut_default_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Field>(
                    "Field",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Field {
    fn clear(&mut self) {
        self.clear_kind();
        self.clear_cardinality();
        self.clear_number();
        self.clear_name();
        self.clear_type_url();
        self.clear_oneof_index();
        self.clear_packed();
        self.clear_options();
        self.clear_json_name();
        self.clear_default_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Field {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Field {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Field_Kind {
    TYPE_UNKNOWN = 0,
    TYPE_DOUBLE = 1,
    TYPE_FLOAT = 2,
    TYPE_INT64 = 3,
    TYPE_UINT64 = 4,
    TYPE_INT32 = 5,
    TYPE_FIXED64 = 6,
    TYPE_FIXED32 = 7,
    TYPE_BOOL = 8,
    TYPE_STRING = 9,
    TYPE_GROUP = 10,
    TYPE_MESSAGE = 11,
    TYPE_BYTES = 12,
    TYPE_UINT32 = 13,
    TYPE_ENUM = 14,
    TYPE_SFIXED32 = 15,
    TYPE_SFIXED64 = 16,
    TYPE_SINT32 = 17,
    TYPE_SINT64 = 18,
}

impl ::protobuf::ProtobufEnum for Field_Kind {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Field_Kind> {
        match value {
            0 => ::std::option::Option::Some(Field_Kind::TYPE_UNKNOWN),
            1 => ::std::option::Option::Some(Field_Kind::TYPE_DOUBLE),
            2 => ::std::option::Option::Some(Field_Kind::TYPE_FLOAT),
            3 => ::std::option::Option::Some(Field_Kind::TYPE_INT64),
            4 => ::std::option::Option::Some(Field_Kind::TYPE_UINT64),
            5 => ::std::option::Option::Some(Field_Kind::TYPE_INT32),
            6 => ::std::option::Option::Some(Field_Kind::TYPE_FIXED64),
            7 => ::std::option::Option::Some(Field_Kind::TYPE_FIXED32),
            8 => ::std::option::Option::Some(Field_Kind::TYPE_BOOL),
            9 => ::std::option::Option::Some(Field_Kind::TYPE_STRING),
            10 => ::std::option::Option::Some(Field_Kind::TYPE_GROUP),
            11 => ::std::option::Option::Some(Field_Kind::TYPE_MESSAGE),
            12 => ::std::option::Option::Some(Field_Kind::TYPE_BYTES),
            13 => ::std::option::Option::Some(Field_Kind::TYPE_UINT32),
            14 => ::std::option::Option::Some(Field_Kind::TYPE_ENUM),
            15 => ::std::option::Option::Some(Field_Kind::TYPE_SFIXED32),
            16 => ::std::option::Option::Some(Field_Kind::TYPE_SFIXED64),
            17 => ::std::option::Option::Some(Field_Kind::TYPE_SINT32),
            18 => ::std::option::Option::Some(Field_Kind::TYPE_SINT64),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Field_Kind] = &[
            Field_Kind::TYPE_UNKNOWN,
            Field_Kind::TYPE_DOUBLE,
            Field_Kind::TYPE_FLOAT,
            Field_Kind::TYPE_INT64,
            Field_Kind::TYPE_UINT64,
            Field_Kind::TYPE_INT32,
            Field_Kind::TYPE_FIXED64,
            Field_Kind::TYPE_FIXED32,
            Field_Kind::TYPE_BOOL,
            Field_Kind::TYPE_STRING,
            Field_Kind::TYPE_GROUP,
            Field_Kind::TYPE_MESSAGE,
            Field_Kind::TYPE_BYTES,
            Field_Kind::TYPE_UINT32,
            Field_Kind::TYPE_ENUM,
            Field_Kind::TYPE_SFIXED32,
            Field_Kind::TYPE_SFIXED64,
            Field_Kind::TYPE_SINT32,
            Field_Kind::TYPE_SINT64,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<Field_Kind>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Field_Kind", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Field_Kind {
}

impl ::std::default::Default for Field_Kind {
    fn default() -> Self {
        Field_Kind::TYPE_UNKNOWN
    }
}

impl ::protobuf::reflect::ProtobufValue for Field_Kind {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Field_Cardinality {
    CARDINALITY_UNKNOWN = 0,
    CARDINALITY_OPTIONAL = 1,
    CARDINALITY_REQUIRED = 2,
    CARDINALITY_REPEATED = 3,
}

impl ::protobuf::ProtobufEnum for Field_Cardinality {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Field_Cardinality> {
        match value {
            0 => ::std::option::Option::Some(Field_Cardinality::CARDINALITY_UNKNOWN),
            1 => ::std::option::Option::Some(Field_Cardinality::CARDINALITY_OPTIONAL),
            2 => ::std::option::Option::Some(Field_Cardinality::CARDINALITY_REQUIRED),
            3 => ::std::option::Option::Some(Field_Cardinality::CARDINALITY_REPEATED),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Field_Cardinality] = &[
            Field_Cardinality::CARDINALITY_UNKNOWN,
            Field_Cardinality::CARDINALITY_OPTIONAL,
            Field_Cardinality::CARDINALITY_REQUIRED,
            Field_Cardinality::CARDINALITY_REPEATED,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<Field_Cardinality>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Field_Cardinality", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Field_Cardinality {
}

impl ::std::default::Default for Field_Cardinality {
    fn default() -> Self {
        Field_Cardinality::CARDINALITY_UNKNOWN
    }
}

impl ::protobuf::reflect::ProtobufValue for Field_Cardinality {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Enum {
    // message fields
    pub name: ::std::string::String,
    pub enumvalue: ::protobuf::RepeatedField<EnumValue>,
    pub options: ::protobuf::RepeatedField<Option>,
    pub source_context: ::protobuf::SingularPtrField<::protobuf::well_known_types::SourceContext>,
    pub syntax: Syntax,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Enum {}

impl Enum {
    pub fn new() -> Enum {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Enum {
        static mut instance: ::protobuf::lazy::Lazy<Enum> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Enum,
        };
        unsafe {
            instance.get(Enum::new)
        }
    }

    // string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // repeated .google.protobuf.EnumValue enumvalue = 2;

    pub fn clear_enumvalue(&mut self) {
        self.enumvalue.clear();
    }

    // Param is passed by value, moved
    pub fn set_enumvalue(&mut self, v: ::protobuf::RepeatedField<EnumValue>) {
        self.enumvalue = v;
    }

    // Mutable pointer to the field.
    pub fn mut_enumvalue(&mut self) -> &mut ::protobuf::RepeatedField<EnumValue> {
        &mut self.enumvalue
    }

    // Take field
    pub fn take_enumvalue(&mut self) -> ::protobuf::RepeatedField<EnumValue> {
        ::std::mem::replace(&mut self.enumvalue, ::protobuf::RepeatedField::new())
    }

    pub fn get_enumvalue(&self) -> &[EnumValue] {
        &self.enumvalue
    }

    fn get_enumvalue_for_reflect(&self) -> &::protobuf::RepeatedField<EnumValue> {
        &self.enumvalue
    }

    fn mut_enumvalue_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<EnumValue> {
        &mut self.enumvalue
    }

    // repeated .google.protobuf.Option options = 3;

    pub fn clear_options(&mut self) {
        self.options.clear();
    }

    // Param is passed by value, moved
    pub fn set_options(&mut self, v: ::protobuf::RepeatedField<Option>) {
        self.options = v;
    }

    // Mutable pointer to the field.
    pub fn mut_options(&mut self) -> &mut ::protobuf::RepeatedField<Option> {
        &mut self.options
    }

    // Take field
    pub fn take_options(&mut self) -> ::protobuf::RepeatedField<Option> {
        ::std::mem::replace(&mut self.options, ::protobuf::RepeatedField::new())
    }

    pub fn get_options(&self) -> &[Option] {
        &self.options
    }

    fn get_options_for_reflect(&self) -> &::protobuf::RepeatedField<Option> {
        &self.options
    }

    fn mut_options_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Option> {
        &mut self.options
    }

    // .google.protobuf.SourceContext source_context = 4;

    pub fn clear_source_context(&mut self) {
        self.source_context.clear();
    }

    pub fn has_source_context(&self) -> bool {
        self.source_context.is_some()
    }

    // Param is passed by value, moved
    pub fn set_source_context(&mut self, v: ::protobuf::well_known_types::SourceContext) {
        self.source_context = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_source_context(&mut self) -> &mut ::protobuf::well_known_types::SourceContext {
        if self.source_context.is_none() {
            self.source_context.set_default();
        }
        self.source_context.as_mut().unwrap()
    }

    // Take field
    pub fn take_source_context(&mut self) -> ::protobuf::well_known_types::SourceContext {
        self.source_context.take().unwrap_or_else(|| ::protobuf::well_known_types::SourceContext::new())
    }

    pub fn get_source_context(&self) -> &::protobuf::well_known_types::SourceContext {
        self.source_context.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::SourceContext::default_instance())
    }

    fn get_source_context_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::SourceContext> {
        &self.source_context
    }

    fn mut_source_context_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::SourceContext> {
        &mut self.source_context
    }

    // .google.protobuf.Syntax syntax = 5;

    pub fn clear_syntax(&mut self) {
        self.syntax = Syntax::SYNTAX_PROTO2;
    }

    // Param is passed by value, moved
    pub fn set_syntax(&mut self, v: Syntax) {
        self.syntax = v;
    }

    pub fn get_syntax(&self) -> Syntax {
        self.syntax
    }

    fn get_syntax_for_reflect(&self) -> &Syntax {
        &self.syntax
    }

    fn mut_syntax_for_reflect(&mut self) -> &mut Syntax {
        &mut self.syntax
    }
}

impl ::protobuf::Message for Enum {
    fn is_initialized(&self) -> bool {
        for v in &self.enumvalue {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.options {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.source_context {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.enumvalue)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.options)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.source_context)?;
                },
                5 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.syntax, 5, &mut self.unknown_fields)?
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        for value in &self.enumvalue {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.options {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.source_context.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.syntax != Syntax::SYNTAX_PROTO2 {
            my_size += ::protobuf::rt::enum_size(5, self.syntax);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        for v in &self.enumvalue {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.options {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.source_context.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.syntax != Syntax::SYNTAX_PROTO2 {
            os.write_enum(5, self.syntax.value())?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Enum {
    fn new() -> Enum {
        Enum::new()
    }

    fn descriptor_static(_: ::std::option::Option<Enum>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    Enum::get_name_for_reflect,
                    Enum::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<EnumValue>>(
                    "enumvalue",
                    Enum::get_enumvalue_for_reflect,
                    Enum::mut_enumvalue_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Option>>(
                    "options",
                    Enum::get_options_for_reflect,
                    Enum::mut_options_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::SourceContext>>(
                    "source_context",
                    Enum::get_source_context_for_reflect,
                    Enum::mut_source_context_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Syntax>>(
                    "syntax",
                    Enum::get_syntax_for_reflect,
                    Enum::mut_syntax_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Enum>(
                    "Enum",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Enum {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_enumvalue();
        self.clear_options();
        self.clear_source_context();
        self.clear_syntax();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Enum {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Enum {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct EnumValue {
    // message fields
    pub name: ::std::string::String,
    pub number: i32,
    pub options: ::protobuf::RepeatedField<Option>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for EnumValue {}

impl EnumValue {
    pub fn new() -> EnumValue {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static EnumValue {
        static mut instance: ::protobuf::lazy::Lazy<EnumValue> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const EnumValue,
        };
        unsafe {
            instance.get(EnumValue::new)
        }
    }

    // string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // int32 number = 2;

    pub fn clear_number(&mut self) {
        self.number = 0;
    }

    // Param is passed by value, moved
    pub fn set_number(&mut self, v: i32) {
        self.number = v;
    }

    pub fn get_number(&self) -> i32 {
        self.number
    }

    fn get_number_for_reflect(&self) -> &i32 {
        &self.number
    }

    fn mut_number_for_reflect(&mut self) -> &mut i32 {
        &mut self.number
    }

    // repeated .google.protobuf.Option options = 3;

    pub fn clear_options(&mut self) {
        self.options.clear();
    }

    // Param is passed by value, moved
    pub fn set_options(&mut self, v: ::protobuf::RepeatedField<Option>) {
        self.options = v;
    }

    // Mutable pointer to the field.
    pub fn mut_options(&mut self) -> &mut ::protobuf::RepeatedField<Option> {
        &mut self.options
    }

    // Take field
    pub fn take_options(&mut self) -> ::protobuf::RepeatedField<Option> {
        ::std::mem::replace(&mut self.options, ::protobuf::RepeatedField::new())
    }

    pub fn get_options(&self) -> &[Option] {
        &self.options
    }

    fn get_options_for_reflect(&self) -> &::protobuf::RepeatedField<Option> {
        &self.options
    }

    fn mut_options_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Option> {
        &mut self.options
    }
}

impl ::protobuf::Message for EnumValue {
    fn is_initialized(&self) -> bool {
        for v in &self.options {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.number = tmp;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.options)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        if self.number != 0 {
            my_size += ::protobuf::rt::value_size(2, self.number, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.options {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        if self.number != 0 {
            os.write_int32(2, self.number)?;
        }
        for v in &self.options {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for EnumValue {
    fn new() -> EnumValue {
        EnumValue::new()
    }

    fn descriptor_static(_: ::std::option::Option<EnumValue>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    EnumValue::get_name_for_reflect,
                    EnumValue::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "number",
                    EnumValue::get_number_for_reflect,
                    EnumValue::mut_number_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Option>>(
                    "options",
                    EnumValue::get_options_for_reflect,
                    EnumValue::mut_options_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<EnumValue>(
                    "EnumValue",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for EnumValue {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_number();
        self.clear_options();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for EnumValue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EnumValue {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Option {
    // message fields
    pub name: ::std::string::String,
    pub value: ::protobuf::SingularPtrField<::protobuf::well_known_types::Any>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Option {}

impl Option {
    pub fn new() -> Option {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Option {
        static mut instance: ::protobuf::lazy::Lazy<Option> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Option,
        };
        unsafe {
            instance.get(Option::new)
        }
    }

    // string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // .google.protobuf.Any value = 2;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::protobuf::well_known_types::Any) {
        self.value = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::protobuf::well_known_types::Any {
        if self.value.is_none() {
            self.value.set_default();
        }
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> ::protobuf::well_known_types::Any {
        self.value.take().unwrap_or_else(|| ::protobuf::well_known_types::Any::new())
    }

    pub fn get_value(&self) -> &::protobuf::well_known_types::Any {
        self.value.as_ref().unwrap_or_else(|| ::protobuf::well_known_types::Any::default_instance())
    }

    fn get_value_for_reflect(&self) -> &::protobuf::SingularPtrField<::protobuf::well_known_types::Any> {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<::protobuf::well_known_types::Any> {
        &mut self.value
    }
}

impl ::protobuf::Message for Option {
    fn is_initialized(&self) -> bool {
        for v in &self.value {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.value)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        if let Some(ref v) = self.value.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        if let Some(ref v) = self.value.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Option {
    fn new() -> Option {
        Option::new()
    }

    fn descriptor_static(_: ::std::option::Option<Option>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    Option::get_name_for_reflect,
                    Option::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Any>>(
                    "value",
                    Option::get_value_for_reflect,
                    Option::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Option>(
                    "Option",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Option {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Option {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Option {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Syntax {
    SYNTAX_PROTO2 = 0,
    SYNTAX_PROTO3 = 1,
}

impl ::protobuf::ProtobufEnum for Syntax {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Syntax> {
        match value {
            0 => ::std::option::Option::Some(Syntax::SYNTAX_PROTO2),
            1 => ::std::option::Option::Some(Syntax::SYNTAX_PROTO3),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Syntax] = &[
            Syntax::SYNTAX_PROTO2,
            Syntax::SYNTAX_PROTO3,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<Syntax>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Syntax", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Syntax {
}

impl ::std::default::Default for Syntax {
    fn default() -> Self {
        Syntax::SYNTAX_PROTO2
    }
}

impl ::protobuf::reflect::ProtobufValue for Syntax {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1agoogle/protobuf/type.proto\x12\x0fgoogle.protobuf\x1a\x19google/pr\
    otobuf/any.proto\x1a$google/protobuf/source_context.proto\"\x8d\x02\n\
    \x04Type\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04name\x12.\n\x06fields\
    \x18\x02\x20\x03(\x0b2\x16.google.protobuf.FieldR\x06fields\x12\x16\n\
    \x06oneofs\x18\x03\x20\x03(\tR\x06oneofs\x121\n\x07options\x18\x04\x20\
    \x03(\x0b2\x17.google.protobuf.OptionR\x07options\x12E\n\x0esource_conte\
    xt\x18\x05\x20\x01(\x0b2\x1e.google.protobuf.SourceContextR\rsourceConte\
    xt\x12/\n\x06syntax\x18\x06\x20\x01(\x0e2\x17.google.protobuf.SyntaxR\
    \x06syntax\"\xb4\x06\n\x05Field\x12/\n\x04kind\x18\x01\x20\x01(\x0e2\x1b\
    .google.protobuf.Field.KindR\x04kind\x12D\n\x0bcardinality\x18\x02\x20\
    \x01(\x0e2\".google.protobuf.Field.CardinalityR\x0bcardinality\x12\x16\n\
    \x06number\x18\x03\x20\x01(\x05R\x06number\x12\x12\n\x04name\x18\x04\x20\
    \x01(\tR\x04name\x12\x19\n\x08type_url\x18\x06\x20\x01(\tR\x07typeUrl\
    \x12\x1f\n\x0boneof_index\x18\x07\x20\x01(\x05R\noneofIndex\x12\x16\n\
    \x06packed\x18\x08\x20\x01(\x08R\x06packed\x121\n\x07options\x18\t\x20\
    \x03(\x0b2\x17.google.protobuf.OptionR\x07options\x12\x1b\n\tjson_name\
    \x18\n\x20\x01(\tR\x08jsonName\x12#\n\rdefault_value\x18\x0b\x20\x01(\tR\
    \x0cdefaultValue\"\xc8\x02\n\x04Kind\x12\x10\n\x0cTYPE_UNKNOWN\x10\0\x12\
    \x0f\n\x0bTYPE_DOUBLE\x10\x01\x12\x0e\n\nTYPE_FLOAT\x10\x02\x12\x0e\n\nT\
    YPE_INT64\x10\x03\x12\x0f\n\x0bTYPE_UINT64\x10\x04\x12\x0e\n\nTYPE_INT32\
    \x10\x05\x12\x10\n\x0cTYPE_FIXED64\x10\x06\x12\x10\n\x0cTYPE_FIXED32\x10\
    \x07\x12\r\n\tTYPE_BOOL\x10\x08\x12\x0f\n\x0bTYPE_STRING\x10\t\x12\x0e\n\
    \nTYPE_GROUP\x10\n\x12\x10\n\x0cTYPE_MESSAGE\x10\x0b\x12\x0e\n\nTYPE_BYT\
    ES\x10\x0c\x12\x0f\n\x0bTYPE_UINT32\x10\r\x12\r\n\tTYPE_ENUM\x10\x0e\x12\
    \x11\n\rTYPE_SFIXED32\x10\x0f\x12\x11\n\rTYPE_SFIXED64\x10\x10\x12\x0f\n\
    \x0bTYPE_SINT32\x10\x11\x12\x0f\n\x0bTYPE_SINT64\x10\x12\"t\n\x0bCardina\
    lity\x12\x17\n\x13CARDINALITY_UNKNOWN\x10\0\x12\x18\n\x14CARDINALITY_OPT\
    IONAL\x10\x01\x12\x18\n\x14CARDINALITY_REQUIRED\x10\x02\x12\x18\n\x14CAR\
    DINALITY_REPEATED\x10\x03\"\xff\x01\n\x04Enum\x12\x12\n\x04name\x18\x01\
    \x20\x01(\tR\x04name\x128\n\tenumvalue\x18\x02\x20\x03(\x0b2\x1a.google.\
    protobuf.EnumValueR\tenumvalue\x121\n\x07options\x18\x03\x20\x03(\x0b2\
    \x17.google.protobuf.OptionR\x07options\x12E\n\x0esource_context\x18\x04\
    \x20\x01(\x0b2\x1e.google.protobuf.SourceContextR\rsourceContext\x12/\n\
    \x06syntax\x18\x05\x20\x01(\x0e2\x17.google.protobuf.SyntaxR\x06syntax\"\
    j\n\tEnumValue\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04name\x12\x16\n\
    \x06number\x18\x02\x20\x01(\x05R\x06number\x121\n\x07options\x18\x03\x20\
    \x03(\x0b2\x17.google.protobuf.OptionR\x07options\"H\n\x06Option\x12\x12\
    \n\x04name\x18\x01\x20\x01(\tR\x04name\x12*\n\x05value\x18\x02\x20\x01(\
    \x0b2\x14.google.protobuf.AnyR\x05value*.\n\x06Syntax\x12\x11\n\rSYNTAX_\
    PROTO2\x10\0\x12\x11\n\rSYNTAX_PROTO3\x10\x01BL\n\x13com.google.protobuf\
    B\tTypeProtoP\x01\xf8\x01\x01\xa2\x02\x03GPB\xaa\x02\x1eGoogle.Protobuf.\
    WellKnownTypesJ\xba:\n\x07\x12\x05\x1e\0\xb3\x01\x01\n\xcc\x0c\n\x01\x0c\
    \x12\x03\x1e\0\x122\xc1\x0c\x20Protocol\x20Buffers\x20-\x20Google's\x20d\
    ata\x20interchange\x20format\n\x20Copyright\x202008\x20Google\x20Inc.\
    \x20\x20All\x20rights\x20reserved.\n\x20https://developers.google.com/pr\
    otocol-buffers/\n\n\x20Redistribution\x20and\x20use\x20in\x20source\x20a\
    nd\x20binary\x20forms,\x20with\x20or\x20without\n\x20modification,\x20ar\
    e\x20permitted\x20provided\x20that\x20the\x20following\x20conditions\x20\
    are\n\x20met:\n\n\x20\x20\x20\x20\x20*\x20Redistributions\x20of\x20sourc\
    e\x20code\x20must\x20retain\x20the\x20above\x20copyright\n\x20notice,\
    \x20this\x20list\x20of\x20conditions\x20and\x20the\x20following\x20discl\
    aimer.\n\x20\x20\x20\x20\x20*\x20Redistributions\x20in\x20binary\x20form\
    \x20must\x20reproduce\x20the\x20above\n\x20copyright\x20notice,\x20this\
    \x20list\x20of\x20conditions\x20and\x20the\x20following\x20disclaimer\n\
    \x20in\x20the\x20documentation\x20and/or\x20other\x20materials\x20provid\
    ed\x20with\x20the\n\x20distribution.\n\x20\x20\x20\x20\x20*\x20Neither\
    \x20the\x20name\x20of\x20Google\x20Inc.\x20nor\x20the\x20names\x20of\x20\
    its\n\x20contributors\x20may\x20be\x20used\x20to\x20endorse\x20or\x20pro\
    mote\x20products\x20derived\x20from\n\x20this\x20software\x20without\x20\
    specific\x20prior\x20written\x20permission.\n\n\x20THIS\x20SOFTWARE\x20I\
    S\x20PROVIDED\x20BY\x20THE\x20COPYRIGHT\x20HOLDERS\x20AND\x20CONTRIBUTOR\
    S\n\x20\"AS\x20IS\"\x20AND\x20ANY\x20EXPRESS\x20OR\x20IMPLIED\x20WARRANT\
    IES,\x20INCLUDING,\x20BUT\x20NOT\n\x20LIMITED\x20TO,\x20THE\x20IMPLIED\
    \x20WARRANTIES\x20OF\x20MERCHANTABILITY\x20AND\x20FITNESS\x20FOR\n\x20A\
    \x20PARTICULAR\x20PURPOSE\x20ARE\x20DISCLAIMED.\x20IN\x20NO\x20EVENT\x20\
    SHALL\x20THE\x20COPYRIGHT\n\x20OWNER\x20OR\x20CONTRIBUTORS\x20BE\x20LIAB\
    LE\x20FOR\x20ANY\x20DIRECT,\x20INDIRECT,\x20INCIDENTAL,\n\x20SPECIAL,\
    \x20EXEMPLARY,\x20OR\x20CONSEQUENTIAL\x20DAMAGES\x20(INCLUDING,\x20BUT\
    \x20NOT\n\x20LIMITED\x20TO,\x20PROCUREMENT\x20OF\x20SUBSTITUTE\x20GOODS\
    \x20OR\x20SERVICES;\x20LOSS\x20OF\x20USE,\n\x20DATA,\x20OR\x20PROFITS;\
    \x20OR\x20BUSINESS\x20INTERRUPTION)\x20HOWEVER\x20CAUSED\x20AND\x20ON\
    \x20ANY\n\x20THEORY\x20OF\x20LIABILITY,\x20WHETHER\x20IN\x20CONTRACT,\
    \x20STRICT\x20LIABILITY,\x20OR\x20TORT\n\x20(INCLUDING\x20NEGLIGENCE\x20\
    OR\x20OTHERWISE)\x20ARISING\x20IN\x20ANY\x20WAY\x20OUT\x20OF\x20THE\x20U\
    SE\n\x20OF\x20THIS\x20SOFTWARE,\x20EVEN\x20IF\x20ADVISED\x20OF\x20THE\
    \x20POSSIBILITY\x20OF\x20SUCH\x20DAMAGE.\n\n\x08\n\x01\x02\x12\x03\x20\
    \x08\x17\n\t\n\x02\x03\0\x12\x03\"\x07\"\n\t\n\x02\x03\x01\x12\x03#\x07-\
    \n\x08\n\x01\x08\x12\x03%\0;\n\x0b\n\x04\x08\xe7\x07\0\x12\x03%\0;\n\x0c\
    \n\x05\x08\xe7\x07\0\x02\x12\x03%\x07\x17\n\r\n\x06\x08\xe7\x07\0\x02\0\
    \x12\x03%\x07\x17\n\x0e\n\x07\x08\xe7\x07\0\x02\0\x01\x12\x03%\x07\x17\n\
    \x0c\n\x05\x08\xe7\x07\0\x07\x12\x03%\x1a:\n\x08\n\x01\x08\x12\x03&\0\
    \x1f\n\x0b\n\x04\x08\xe7\x07\x01\x12\x03&\0\x1f\n\x0c\n\x05\x08\xe7\x07\
    \x01\x02\x12\x03&\x07\x17\n\r\n\x06\x08\xe7\x07\x01\x02\0\x12\x03&\x07\
    \x17\n\x0e\n\x07\x08\xe7\x07\x01\x02\0\x01\x12\x03&\x07\x17\n\x0c\n\x05\
    \x08\xe7\x07\x01\x03\x12\x03&\x1a\x1e\n\x08\n\x01\x08\x12\x03'\0,\n\x0b\
    \n\x04\x08\xe7\x07\x02\x12\x03'\0,\n\x0c\n\x05\x08\xe7\x07\x02\x02\x12\
    \x03'\x07\x13\n\r\n\x06\x08\xe7\x07\x02\x02\0\x12\x03'\x07\x13\n\x0e\n\
    \x07\x08\xe7\x07\x02\x02\0\x01\x12\x03'\x07\x13\n\x0c\n\x05\x08\xe7\x07\
    \x02\x07\x12\x03'\x16+\n\x08\n\x01\x08\x12\x03(\0*\n\x0b\n\x04\x08\xe7\
    \x07\x03\x12\x03(\0*\n\x0c\n\x05\x08\xe7\x07\x03\x02\x12\x03(\x07\x1b\n\
    \r\n\x06\x08\xe7\x07\x03\x02\0\x12\x03(\x07\x1b\n\x0e\n\x07\x08\xe7\x07\
    \x03\x02\0\x01\x12\x03(\x07\x1b\n\x0c\n\x05\x08\xe7\x07\x03\x07\x12\x03(\
    \x1e)\n\x08\n\x01\x08\x12\x03)\0\"\n\x0b\n\x04\x08\xe7\x07\x04\x12\x03)\
    \0\"\n\x0c\n\x05\x08\xe7\x07\x04\x02\x12\x03)\x07\x1a\n\r\n\x06\x08\xe7\
    \x07\x04\x02\0\x12\x03)\x07\x1a\n\x0e\n\x07\x08\xe7\x07\x04\x02\0\x01\
    \x12\x03)\x07\x1a\n\x0c\n\x05\x08\xe7\x07\x04\x03\x12\x03)\x1d!\n\x08\n\
    \x01\x08\x12\x03*\0!\n\x0b\n\x04\x08\xe7\x07\x05\x12\x03*\0!\n\x0c\n\x05\
    \x08\xe7\x07\x05\x02\x12\x03*\x07\x18\n\r\n\x06\x08\xe7\x07\x05\x02\0\
    \x12\x03*\x07\x18\n\x0e\n\x07\x08\xe7\x07\x05\x02\0\x01\x12\x03*\x07\x18\
    \n\x0c\n\x05\x08\xe7\x07\x05\x07\x12\x03*\x1b\x20\n-\n\x02\x04\0\x12\x04\
    -\0:\x01\x1a!\x20A\x20protocol\x20buffer\x20message\x20type.\n\n\n\n\x03\
    \x04\0\x01\x12\x03-\x08\x0c\n0\n\x04\x04\0\x02\0\x12\x03/\x02\x12\x1a#\
    \x20The\x20fully\x20qualified\x20message\x20name.\n\n\r\n\x05\x04\0\x02\
    \0\x04\x12\x04/\x02-\x0e\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03/\x02\x08\n\
    \x0c\n\x05\x04\0\x02\0\x01\x12\x03/\t\r\n\x0c\n\x05\x04\0\x02\0\x03\x12\
    \x03/\x10\x11\n\"\n\x04\x04\0\x02\x01\x12\x031\x02\x1c\x1a\x15\x20The\
    \x20list\x20of\x20fields.\n\n\x0c\n\x05\x04\0\x02\x01\x04\x12\x031\x02\n\
    \n\x0c\n\x05\x04\0\x02\x01\x06\x12\x031\x0b\x10\n\x0c\n\x05\x04\0\x02\
    \x01\x01\x12\x031\x11\x17\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x031\x1a\x1b\
    \nO\n\x04\x04\0\x02\x02\x12\x033\x02\x1d\x1aB\x20The\x20list\x20of\x20ty\
    pes\x20appearing\x20in\x20`oneof`\x20definitions\x20in\x20this\x20type.\
    \n\n\x0c\n\x05\x04\0\x02\x02\x04\x12\x033\x02\n\n\x0c\n\x05\x04\0\x02\
    \x02\x05\x12\x033\x0b\x11\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x033\x12\x18\
    \n\x0c\n\x05\x04\0\x02\x02\x03\x12\x033\x1b\x1c\n+\n\x04\x04\0\x02\x03\
    \x12\x035\x02\x1e\x1a\x1e\x20The\x20protocol\x20buffer\x20options.\n\n\
    \x0c\n\x05\x04\0\x02\x03\x04\x12\x035\x02\n\n\x0c\n\x05\x04\0\x02\x03\
    \x06\x12\x035\x0b\x11\n\x0c\n\x05\x04\0\x02\x03\x01\x12\x035\x12\x19\n\
    \x0c\n\x05\x04\0\x02\x03\x03\x12\x035\x1c\x1d\n\"\n\x04\x04\0\x02\x04\
    \x12\x037\x02#\x1a\x15\x20The\x20source\x20context.\n\n\r\n\x05\x04\0\
    \x02\x04\x04\x12\x047\x025\x1e\n\x0c\n\x05\x04\0\x02\x04\x06\x12\x037\
    \x02\x0f\n\x0c\n\x05\x04\0\x02\x04\x01\x12\x037\x10\x1e\n\x0c\n\x05\x04\
    \0\x02\x04\x03\x12\x037!\"\n!\n\x04\x04\0\x02\x05\x12\x039\x02\x14\x1a\
    \x14\x20The\x20source\x20syntax.\n\n\r\n\x05\x04\0\x02\x05\x04\x12\x049\
    \x027#\n\x0c\n\x05\x04\0\x02\x05\x06\x12\x039\x02\x08\n\x0c\n\x05\x04\0\
    \x02\x05\x01\x12\x039\t\x0f\n\x0c\n\x05\x04\0\x02\x05\x03\x12\x039\x12\
    \x13\n0\n\x02\x04\x01\x12\x05=\0\x8a\x01\x01\x1a#\x20A\x20single\x20fiel\
    d\x20of\x20a\x20message\x20type.\n\n\n\n\x03\x04\x01\x01\x12\x03=\x08\r\
    \n\"\n\x04\x04\x01\x04\0\x12\x04?\x02f\x03\x1a\x14\x20Basic\x20field\x20\
    types.\n\n\x0c\n\x05\x04\x01\x04\0\x01\x12\x03?\x07\x0b\n$\n\x06\x04\x01\
    \x04\0\x02\0\x12\x03A\x04\x1c\x1a\x15\x20Field\x20type\x20unknown.\n\n\
    \x0e\n\x07\x04\x01\x04\0\x02\0\x01\x12\x03A\x04\x10\n\x0e\n\x07\x04\x01\
    \x04\0\x02\0\x02\x12\x03A\x1a\x1b\n#\n\x06\x04\x01\x04\0\x02\x01\x12\x03\
    C\x04\x1c\x1a\x14\x20Field\x20type\x20double.\n\n\x0e\n\x07\x04\x01\x04\
    \0\x02\x01\x01\x12\x03C\x04\x0f\n\x0e\n\x07\x04\x01\x04\0\x02\x01\x02\
    \x12\x03C\x1a\x1b\n\"\n\x06\x04\x01\x04\0\x02\x02\x12\x03E\x04\x1c\x1a\
    \x13\x20Field\x20type\x20float.\n\n\x0e\n\x07\x04\x01\x04\0\x02\x02\x01\
    \x12\x03E\x04\x0e\n\x0e\n\x07\x04\x01\x04\0\x02\x02\x02\x12\x03E\x1a\x1b\
    \n\"\n\x06\x04\x01\x04\0\x02\x03\x12\x03G\x04\x1c\x1a\x13\x20Field\x20ty\
    pe\x20int64.\n\n\x0e\n\x07\x04\x01\x04\0\x02\x03\x01\x12\x03G\x04\x0e\n\
    \x0e\n\x07\x04\x01\x04\0\x02\x03\x02\x12\x03G\x1a\x1b\n#\n\x06\x04\x01\
    \x04\0\x02\x04\x12\x03I\x04\x1c\x1a\x14\x20Field\x20type\x20uint64.\n\n\
    \x0e\n\x07\x04\x01\x04\0\x02\x04\x01\x12\x03I\x04\x0f\n\x0e\n\x07\x04\
    \x01\x04\0\x02\x04\x02\x12\x03I\x1a\x1b\n\"\n\x06\x04\x01\x04\0\x02\x05\
    \x12\x03K\x04\x1c\x1a\x13\x20Field\x20type\x20int32.\n\n\x0e\n\x07\x04\
    \x01\x04\0\x02\x05\x01\x12\x03K\x04\x0e\n\x0e\n\x07\x04\x01\x04\0\x02\
    \x05\x02\x12\x03K\x1a\x1b\n$\n\x06\x04\x01\x04\0\x02\x06\x12\x03M\x04\
    \x1c\x1a\x15\x20Field\x20type\x20fixed64.\n\n\x0e\n\x07\x04\x01\x04\0\
    \x02\x06\x01\x12\x03M\x04\x10\n\x0e\n\x07\x04\x01\x04\0\x02\x06\x02\x12\
    \x03M\x1a\x1b\n$\n\x06\x04\x01\x04\0\x02\x07\x12\x03O\x04\x1c\x1a\x15\
    \x20Field\x20type\x20fixed32.\n\n\x0e\n\x07\x04\x01\x04\0\x02\x07\x01\
    \x12\x03O\x04\x10\n\x0e\n\x07\x04\x01\x04\0\x02\x07\x02\x12\x03O\x1a\x1b\
    \n!\n\x06\x04\x01\x04\0\x02\x08\x12\x03Q\x04\x1c\x1a\x12\x20Field\x20typ\
    e\x20bool.\n\n\x0e\n\x07\x04\x01\x04\0\x02\x08\x01\x12\x03Q\x04\r\n\x0e\
    \n\x07\x04\x01\x04\0\x02\x08\x02\x12\x03Q\x1a\x1b\n#\n\x06\x04\x01\x04\0\
    \x02\t\x12\x03S\x04\x1c\x1a\x14\x20Field\x20type\x20string.\n\n\x0e\n\
    \x07\x04\x01\x04\0\x02\t\x01\x12\x03S\x04\x0f\n\x0e\n\x07\x04\x01\x04\0\
    \x02\t\x02\x12\x03S\x1a\x1b\nF\n\x06\x04\x01\x04\0\x02\n\x12\x03U\x04\
    \x1d\x1a7\x20Field\x20type\x20group.\x20Proto2\x20syntax\x20only,\x20and\
    \x20deprecated.\n\n\x0e\n\x07\x04\x01\x04\0\x02\n\x01\x12\x03U\x04\x0e\n\
    \x0e\n\x07\x04\x01\x04\0\x02\n\x02\x12\x03U\x1a\x1c\n$\n\x06\x04\x01\x04\
    \0\x02\x0b\x12\x03W\x04\x1d\x1a\x15\x20Field\x20type\x20message.\n\n\x0e\
    \n\x07\x04\x01\x04\0\x02\x0b\x01\x12\x03W\x04\x10\n\x0e\n\x07\x04\x01\
    \x04\0\x02\x0b\x02\x12\x03W\x1a\x1c\n\"\n\x06\x04\x01\x04\0\x02\x0c\x12\
    \x03Y\x04\x1d\x1a\x13\x20Field\x20type\x20bytes.\n\n\x0e\n\x07\x04\x01\
    \x04\0\x02\x0c\x01\x12\x03Y\x04\x0e\n\x0e\n\x07\x04\x01\x04\0\x02\x0c\
    \x02\x12\x03Y\x1a\x1c\n#\n\x06\x04\x01\x04\0\x02\r\x12\x03[\x04\x1d\x1a\
    \x14\x20Field\x20type\x20uint32.\n\n\x0e\n\x07\x04\x01\x04\0\x02\r\x01\
    \x12\x03[\x04\x0f\n\x0e\n\x07\x04\x01\x04\0\x02\r\x02\x12\x03[\x1a\x1c\n\
    !\n\x06\x04\x01\x04\0\x02\x0e\x12\x03]\x04\x1d\x1a\x12\x20Field\x20type\
    \x20enum.\n\n\x0e\n\x07\x04\x01\x04\0\x02\x0e\x01\x12\x03]\x04\r\n\x0e\n\
    \x07\x04\x01\x04\0\x02\x0e\x02\x12\x03]\x1a\x1c\n%\n\x06\x04\x01\x04\0\
    \x02\x0f\x12\x03_\x04\x1d\x1a\x16\x20Field\x20type\x20sfixed32.\n\n\x0e\
    \n\x07\x04\x01\x04\0\x02\x0f\x01\x12\x03_\x04\x11\n\x0e\n\x07\x04\x01\
    \x04\0\x02\x0f\x02\x12\x03_\x1a\x1c\n%\n\x06\x04\x01\x04\0\x02\x10\x12\
    \x03a\x04\x1d\x1a\x16\x20Field\x20type\x20sfixed64.\n\n\x0e\n\x07\x04\
    \x01\x04\0\x02\x10\x01\x12\x03a\x04\x11\n\x0e\n\x07\x04\x01\x04\0\x02\
    \x10\x02\x12\x03a\x1a\x1c\n#\n\x06\x04\x01\x04\0\x02\x11\x12\x03c\x04\
    \x1d\x1a\x14\x20Field\x20type\x20sint32.\n\n\x0e\n\x07\x04\x01\x04\0\x02\
    \x11\x01\x12\x03c\x04\x0f\n\x0e\n\x07\x04\x01\x04\0\x02\x11\x02\x12\x03c\
    \x1a\x1c\n#\n\x06\x04\x01\x04\0\x02\x12\x12\x03e\x04\x1d\x1a\x14\x20Fiel\
    d\x20type\x20sint64.\n\n\x0e\n\x07\x04\x01\x04\0\x02\x12\x01\x12\x03e\
    \x04\x0f\n\x0e\n\x07\x04\x01\x04\0\x02\x12\x02\x12\x03e\x1a\x1c\nC\n\x04\
    \x04\x01\x04\x01\x12\x04i\x02r\x03\x1a5\x20Whether\x20a\x20field\x20is\
    \x20optional,\x20required,\x20or\x20repeated.\n\n\x0c\n\x05\x04\x01\x04\
    \x01\x01\x12\x03i\x07\x12\n5\n\x06\x04\x01\x04\x01\x02\0\x12\x03k\x04\
    \x1c\x1a&\x20For\x20fields\x20with\x20unknown\x20cardinality.\n\n\x0e\n\
    \x07\x04\x01\x04\x01\x02\0\x01\x12\x03k\x04\x17\n\x0e\n\x07\x04\x01\x04\
    \x01\x02\0\x02\x12\x03k\x1a\x1b\n%\n\x06\x04\x01\x04\x01\x02\x01\x12\x03\
    m\x04\x1d\x1a\x16\x20For\x20optional\x20fields.\n\n\x0e\n\x07\x04\x01\
    \x04\x01\x02\x01\x01\x12\x03m\x04\x18\n\x0e\n\x07\x04\x01\x04\x01\x02\
    \x01\x02\x12\x03m\x1b\x1c\n9\n\x06\x04\x01\x04\x01\x02\x02\x12\x03o\x04\
    \x1d\x1a*\x20For\x20required\x20fields.\x20Proto2\x20syntax\x20only.\n\n\
    \x0e\n\x07\x04\x01\x04\x01\x02\x02\x01\x12\x03o\x04\x18\n\x0e\n\x07\x04\
    \x01\x04\x01\x02\x02\x02\x12\x03o\x1b\x1c\n%\n\x06\x04\x01\x04\x01\x02\
    \x03\x12\x03q\x04\x1d\x1a\x16\x20For\x20repeated\x20fields.\n\n\x0e\n\
    \x07\x04\x01\x04\x01\x02\x03\x01\x12\x03q\x04\x18\n\x0e\n\x07\x04\x01\
    \x04\x01\x02\x03\x02\x12\x03q\x1b\x1c\n\x1e\n\x04\x04\x01\x02\0\x12\x03u\
    \x02\x10\x1a\x11\x20The\x20field\x20type.\n\n\r\n\x05\x04\x01\x02\0\x04\
    \x12\x04u\x02r\x04\n\x0c\n\x05\x04\x01\x02\0\x06\x12\x03u\x02\x06\n\x0c\
    \n\x05\x04\x01\x02\0\x01\x12\x03u\x07\x0b\n\x0c\n\x05\x04\x01\x02\0\x03\
    \x12\x03u\x0e\x0f\n%\n\x04\x04\x01\x02\x01\x12\x03w\x02\x1e\x1a\x18\x20T\
    he\x20field\x20cardinality.\n\n\r\n\x05\x04\x01\x02\x01\x04\x12\x04w\x02\
    u\x10\n\x0c\n\x05\x04\x01\x02\x01\x06\x12\x03w\x02\r\n\x0c\n\x05\x04\x01\
    \x02\x01\x01\x12\x03w\x0e\x19\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03w\
    \x1c\x1d\n\x20\n\x04\x04\x01\x02\x02\x12\x03y\x02\x13\x1a\x13\x20The\x20\
    field\x20number.\n\n\r\n\x05\x04\x01\x02\x02\x04\x12\x04y\x02w\x1e\n\x0c\
    \n\x05\x04\x01\x02\x02\x05\x12\x03y\x02\x07\n\x0c\n\x05\x04\x01\x02\x02\
    \x01\x12\x03y\x08\x0e\n\x0c\n\x05\x04\x01\x02\x02\x03\x12\x03y\x11\x12\n\
    \x1e\n\x04\x04\x01\x02\x03\x12\x03{\x02\x12\x1a\x11\x20The\x20field\x20n\
    ame.\n\n\r\n\x05\x04\x01\x02\x03\x04\x12\x04{\x02y\x13\n\x0c\n\x05\x04\
    \x01\x02\x03\x05\x12\x03{\x02\x08\n\x0c\n\x05\x04\x01\x02\x03\x01\x12\
    \x03{\t\r\n\x0c\n\x05\x04\x01\x02\x03\x03\x12\x03{\x10\x11\n\x96\x01\n\
    \x04\x04\x01\x02\x04\x12\x03~\x02\x16\x1a\x88\x01\x20The\x20field\x20typ\
    e\x20URL,\x20without\x20the\x20scheme,\x20for\x20message\x20or\x20enumer\
    ation\n\x20types.\x20Example:\x20`\"type.googleapis.com/google.protobuf.\
    Timestamp\"`.\n\n\r\n\x05\x04\x01\x02\x04\x04\x12\x04~\x02{\x12\n\x0c\n\
    \x05\x04\x01\x02\x04\x05\x12\x03~\x02\x08\n\x0c\n\x05\x04\x01\x02\x04\
    \x01\x12\x03~\t\x11\n\x0c\n\x05\x04\x01\x02\x04\x03\x12\x03~\x14\x15\n\
    \xa5\x01\n\x04\x04\x01\x02\x05\x12\x04\x81\x01\x02\x18\x1a\x96\x01\x20Th\
    e\x20index\x20of\x20the\x20field\x20type\x20in\x20`Type.oneofs`,\x20for\
    \x20message\x20or\x20enumeration\n\x20types.\x20The\x20first\x20type\x20\
    has\x20index\x201;\x20zero\x20means\x20the\x20type\x20is\x20not\x20in\
    \x20the\x20list.\n\n\x0e\n\x05\x04\x01\x02\x05\x04\x12\x05\x81\x01\x02~\
    \x16\n\r\n\x05\x04\x01\x02\x05\x05\x12\x04\x81\x01\x02\x07\n\r\n\x05\x04\
    \x01\x02\x05\x01\x12\x04\x81\x01\x08\x13\n\r\n\x05\x04\x01\x02\x05\x03\
    \x12\x04\x81\x01\x16\x17\nF\n\x04\x04\x01\x02\x06\x12\x04\x83\x01\x02\
    \x12\x1a8\x20Whether\x20to\x20use\x20alternative\x20packed\x20wire\x20re\
    presentation.\n\n\x0f\n\x05\x04\x01\x02\x06\x04\x12\x06\x83\x01\x02\x81\
    \x01\x18\n\r\n\x05\x04\x01\x02\x06\x05\x12\x04\x83\x01\x02\x06\n\r\n\x05\
    \x04\x01\x02\x06\x01\x12\x04\x83\x01\x07\r\n\r\n\x05\x04\x01\x02\x06\x03\
    \x12\x04\x83\x01\x10\x11\n,\n\x04\x04\x01\x02\x07\x12\x04\x85\x01\x02\
    \x1e\x1a\x1e\x20The\x20protocol\x20buffer\x20options.\n\n\r\n\x05\x04\
    \x01\x02\x07\x04\x12\x04\x85\x01\x02\n\n\r\n\x05\x04\x01\x02\x07\x06\x12\
    \x04\x85\x01\x0b\x11\n\r\n\x05\x04\x01\x02\x07\x01\x12\x04\x85\x01\x12\
    \x19\n\r\n\x05\x04\x01\x02\x07\x03\x12\x04\x85\x01\x1c\x1d\n$\n\x04\x04\
    \x01\x02\x08\x12\x04\x87\x01\x02\x18\x1a\x16\x20The\x20field\x20JSON\x20\
    name.\n\n\x0f\n\x05\x04\x01\x02\x08\x04\x12\x06\x87\x01\x02\x85\x01\x1e\
    \n\r\n\x05\x04\x01\x02\x08\x05\x12\x04\x87\x01\x02\x08\n\r\n\x05\x04\x01\
    \x02\x08\x01\x12\x04\x87\x01\t\x12\n\r\n\x05\x04\x01\x02\x08\x03\x12\x04\
    \x87\x01\x15\x17\nX\n\x04\x04\x01\x02\t\x12\x04\x89\x01\x02\x1c\x1aJ\x20\
    The\x20string\x20value\x20of\x20the\x20default\x20value\x20of\x20this\
    \x20field.\x20Proto2\x20syntax\x20only.\n\n\x0f\n\x05\x04\x01\x02\t\x04\
    \x12\x06\x89\x01\x02\x87\x01\x18\n\r\n\x05\x04\x01\x02\t\x05\x12\x04\x89\
    \x01\x02\x08\n\r\n\x05\x04\x01\x02\t\x01\x12\x04\x89\x01\t\x16\n\r\n\x05\
    \x04\x01\x02\t\x03\x12\x04\x89\x01\x19\x1b\n%\n\x02\x04\x02\x12\x06\x8d\
    \x01\0\x98\x01\x01\x1a\x17\x20Enum\x20type\x20definition.\n\n\x0b\n\x03\
    \x04\x02\x01\x12\x04\x8d\x01\x08\x0c\n\x1f\n\x04\x04\x02\x02\0\x12\x04\
    \x8f\x01\x02\x12\x1a\x11\x20Enum\x20type\x20name.\n\n\x0f\n\x05\x04\x02\
    \x02\0\x04\x12\x06\x8f\x01\x02\x8d\x01\x0e\n\r\n\x05\x04\x02\x02\0\x05\
    \x12\x04\x8f\x01\x02\x08\n\r\n\x05\x04\x02\x02\0\x01\x12\x04\x8f\x01\t\r\
    \n\r\n\x05\x04\x02\x02\0\x03\x12\x04\x8f\x01\x10\x11\n'\n\x04\x04\x02\
    \x02\x01\x12\x04\x91\x01\x02#\x1a\x19\x20Enum\x20value\x20definitions.\n\
    \n\r\n\x05\x04\x02\x02\x01\x04\x12\x04\x91\x01\x02\n\n\r\n\x05\x04\x02\
    \x02\x01\x06\x12\x04\x91\x01\x0b\x14\n\r\n\x05\x04\x02\x02\x01\x01\x12\
    \x04\x91\x01\x15\x1e\n\r\n\x05\x04\x02\x02\x01\x03\x12\x04\x91\x01!\"\n(\
    \n\x04\x04\x02\x02\x02\x12\x04\x93\x01\x02\x1e\x1a\x1a\x20Protocol\x20bu\
    ffer\x20options.\n\n\r\n\x05\x04\x02\x02\x02\x04\x12\x04\x93\x01\x02\n\n\
    \r\n\x05\x04\x02\x02\x02\x06\x12\x04\x93\x01\x0b\x11\n\r\n\x05\x04\x02\
    \x02\x02\x01\x12\x04\x93\x01\x12\x19\n\r\n\x05\x04\x02\x02\x02\x03\x12\
    \x04\x93\x01\x1c\x1d\n#\n\x04\x04\x02\x02\x03\x12\x04\x95\x01\x02#\x1a\
    \x15\x20The\x20source\x20context.\n\n\x0f\n\x05\x04\x02\x02\x03\x04\x12\
    \x06\x95\x01\x02\x93\x01\x1e\n\r\n\x05\x04\x02\x02\x03\x06\x12\x04\x95\
    \x01\x02\x0f\n\r\n\x05\x04\x02\x02\x03\x01\x12\x04\x95\x01\x10\x1e\n\r\n\
    \x05\x04\x02\x02\x03\x03\x12\x04\x95\x01!\"\n\"\n\x04\x04\x02\x02\x04\
    \x12\x04\x97\x01\x02\x14\x1a\x14\x20The\x20source\x20syntax.\n\n\x0f\n\
    \x05\x04\x02\x02\x04\x04\x12\x06\x97\x01\x02\x95\x01#\n\r\n\x05\x04\x02\
    \x02\x04\x06\x12\x04\x97\x01\x02\x08\n\r\n\x05\x04\x02\x02\x04\x01\x12\
    \x04\x97\x01\t\x0f\n\r\n\x05\x04\x02\x02\x04\x03\x12\x04\x97\x01\x12\x13\
    \n&\n\x02\x04\x03\x12\x06\x9b\x01\0\xa2\x01\x01\x1a\x18\x20Enum\x20value\
    \x20definition.\n\n\x0b\n\x03\x04\x03\x01\x12\x04\x9b\x01\x08\x11\n\x20\
    \n\x04\x04\x03\x02\0\x12\x04\x9d\x01\x02\x12\x1a\x12\x20Enum\x20value\
    \x20name.\n\n\x0f\n\x05\x04\x03\x02\0\x04\x12\x06\x9d\x01\x02\x9b\x01\
    \x13\n\r\n\x05\x04\x03\x02\0\x05\x12\x04\x9d\x01\x02\x08\n\r\n\x05\x04\
    \x03\x02\0\x01\x12\x04\x9d\x01\t\r\n\r\n\x05\x04\x03\x02\0\x03\x12\x04\
    \x9d\x01\x10\x11\n\"\n\x04\x04\x03\x02\x01\x12\x04\x9f\x01\x02\x13\x1a\
    \x14\x20Enum\x20value\x20number.\n\n\x0f\n\x05\x04\x03\x02\x01\x04\x12\
    \x06\x9f\x01\x02\x9d\x01\x12\n\r\n\x05\x04\x03\x02\x01\x05\x12\x04\x9f\
    \x01\x02\x07\n\r\n\x05\x04\x03\x02\x01\x01\x12\x04\x9f\x01\x08\x0e\n\r\n\
    \x05\x04\x03\x02\x01\x03\x12\x04\x9f\x01\x11\x12\n(\n\x04\x04\x03\x02\
    \x02\x12\x04\xa1\x01\x02\x1e\x1a\x1a\x20Protocol\x20buffer\x20options.\n\
    \n\r\n\x05\x04\x03\x02\x02\x04\x12\x04\xa1\x01\x02\n\n\r\n\x05\x04\x03\
    \x02\x02\x06\x12\x04\xa1\x01\x0b\x11\n\r\n\x05\x04\x03\x02\x02\x01\x12\
    \x04\xa1\x01\x12\x19\n\r\n\x05\x04\x03\x02\x02\x03\x12\x04\xa1\x01\x1c\
    \x1d\ng\n\x02\x04\x04\x12\x06\xa6\x01\0\xab\x01\x01\x1aY\x20A\x20protoco\
    l\x20buffer\x20option,\x20which\x20can\x20be\x20attached\x20to\x20a\x20m\
    essage,\x20field,\n\x20enumeration,\x20etc.\n\n\x0b\n\x03\x04\x04\x01\
    \x12\x04\xa6\x01\x08\x0e\nA\n\x04\x04\x04\x02\0\x12\x04\xa8\x01\x02\x12\
    \x1a3\x20The\x20option's\x20name.\x20For\x20example,\x20`\"java_package\
    \"`.\n\n\x0f\n\x05\x04\x04\x02\0\x04\x12\x06\xa8\x01\x02\xa6\x01\x10\n\r\
    \n\x05\x04\x04\x02\0\x05\x12\x04\xa8\x01\x02\x08\n\r\n\x05\x04\x04\x02\0\
    \x01\x12\x04\xa8\x01\t\r\n\r\n\x05\x04\x04\x02\0\x03\x12\x04\xa8\x01\x10\
    \x11\nI\n\x04\x04\x04\x02\x01\x12\x04\xaa\x01\x02\x10\x1a;\x20The\x20opt\
    ion's\x20value.\x20For\x20example,\x20`\"com.google.protobuf\"`.\n\n\x0f\
    \n\x05\x04\x04\x02\x01\x04\x12\x06\xaa\x01\x02\xa8\x01\x12\n\r\n\x05\x04\
    \x04\x02\x01\x06\x12\x04\xaa\x01\x02\x05\n\r\n\x05\x04\x04\x02\x01\x01\
    \x12\x04\xaa\x01\x06\x0b\n\r\n\x05\x04\x04\x02\x01\x03\x12\x04\xaa\x01\
    \x0e\x0f\nI\n\x02\x05\0\x12\x06\xae\x01\0\xb3\x01\x01\x1a;\x20The\x20syn\
    tax\x20in\x20which\x20a\x20protocol\x20buffer\x20element\x20is\x20define\
    d.\n\n\x0b\n\x03\x05\0\x01\x12\x04\xae\x01\x05\x0b\n\x20\n\x04\x05\0\x02\
    \0\x12\x04\xb0\x01\x02\x14\x1a\x12\x20Syntax\x20`proto2`.\n\n\r\n\x05\
    \x05\0\x02\0\x01\x12\x04\xb0\x01\x02\x0f\n\r\n\x05\x05\0\x02\0\x02\x12\
    \x04\xb0\x01\x12\x13\n\x20\n\x04\x05\0\x02\x01\x12\x04\xb2\x01\x02\x14\
    \x1a\x12\x20Syntax\x20`proto3`.\n\n\r\n\x05\x05\0\x02\x01\x01\x12\x04\
    \xb2\x01\x02\x0f\n\r\n\x05\x05\0\x02\x01\x02\x12\x04\xb2\x01\x12\x13b\
    \x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
