#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use spacetimedb_bindgen::spacetimedb;
use spacetimedb_bindings::hash::Hash;
pub struct Position {
    pub pos_x: f32,
    pub pos_y: f32,
    pub pos_z: f32,
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for Position {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = match _serde::Serializer::serialize_struct(
                __serializer,
                "Position",
                false as usize + 1 + 1 + 1,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "pos_x",
                &self.pos_x,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "pos_y",
                &self.pos_y,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "pos_z",
                &self.pos_z,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for Position {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                __field0,
                __field1,
                __field2,
                __ignore,
            }
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "field identifier")
                }
                fn visit_u64<__E>(self, __value: u64) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        2u64 => _serde::__private::Ok(__Field::__field2),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "pos_x" => _serde::__private::Ok(__Field::__field0),
                        "pos_y" => _serde::__private::Ok(__Field::__field1),
                        "pos_z" => _serde::__private::Ok(__Field::__field2),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"pos_x" => _serde::__private::Ok(__Field::__field0),
                        b"pos_y" => _serde::__private::Ok(__Field::__field1),
                        b"pos_z" => _serde::__private::Ok(__Field::__field2),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                }
            }
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<Position>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = Position;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct Position")
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 =
                        match match _serde::de::SeqAccess::next_element::<f32>(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct Position with 3 elements",
                                ));
                            }
                        };
                    let __field1 =
                        match match _serde::de::SeqAccess::next_element::<f32>(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct Position with 3 elements",
                                ));
                            }
                        };
                    let __field2 =
                        match match _serde::de::SeqAccess::next_element::<f32>(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    2usize,
                                    &"struct Position with 3 elements",
                                ));
                            }
                        };
                    _serde::__private::Ok(Position {
                        pos_x: __field0,
                        pos_y: __field1,
                        pos_z: __field2,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut __field2: _serde::__private::Option<f32> = _serde::__private::None;
                    while let _serde::__private::Some(__key) =
                        match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        }
                    {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("pos_x"),
                                    );
                                }
                                __field0 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("pos_y"),
                                    );
                                }
                                __field1 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field2 => {
                                if _serde::__private::Option::is_some(&__field2) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("pos_z"),
                                    );
                                }
                                __field2 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            _ => {
                                let _ = match _serde::de::MapAccess::next_value::<
                                    _serde::de::IgnoredAny,
                                >(&mut __map)
                                {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                };
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("pos_x") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("pos_y") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field2 = match __field2 {
                        _serde::__private::Some(__field2) => __field2,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("pos_z") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    _serde::__private::Ok(Position {
                        pos_x: __field0,
                        pos_y: __field1,
                        pos_z: __field2,
                    })
                }
            }
            const FIELDS: &'static [&'static str] = &["pos_x", "pos_y", "pos_z"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "Position",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<Position>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[allow(non_snake_case)]
fn __get_struct_schema__Position() -> spacetimedb_bindings::TypeDef {
    return spacetimedb_bindings::TypeDef::Tuple {
        0: spacetimedb_bindings::TupleDef {
            elements: <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    spacetimedb_bindings::ElementDef {
                        tag: 0u8,
                        element_type: spacetimedb_bindings::TypeDef::F32,
                    },
                    spacetimedb_bindings::ElementDef {
                        tag: 1u8,
                        element_type: spacetimedb_bindings::TypeDef::F32,
                    },
                    spacetimedb_bindings::ElementDef {
                        tag: 2u8,
                        element_type: spacetimedb_bindings::TypeDef::F32,
                    },
                ]),
            ),
        },
    };
}
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __create_type__Position(arg_ptr: usize, arg_size: usize) {
    unsafe {
        let ptr = arg_ptr as *mut u8;
        let def = __get_struct_schema__Position();
        let mut bytes = Vec::from_raw_parts(ptr, 0, arg_size);
        def.encode(&mut bytes);
    }
}
#[allow(non_snake_case)]
fn __tuple_to_struct__Position(value: spacetimedb_bindings::TupleValue) -> Option<Position> {
    let elements_arr = value.elements;
    return match (
        elements_arr[0usize].clone(),
        elements_arr[1usize].clone(),
        elements_arr[2usize].clone(),
    ) {
        (
            spacetimedb_bindings::TypeValue::F32(field_0),
            spacetimedb_bindings::TypeValue::F32(field_1),
            spacetimedb_bindings::TypeValue::F32(field_2),
        ) => Some(Position {
            pos_x: field_0,
            pos_y: field_1,
            pos_z: field_2,
        }),
        _ => None,
    };
}
#[allow(non_snake_case)]
fn __struct_to_tuple__Position(value: Position) -> spacetimedb_bindings::TypeValue {
    return spacetimedb_bindings::TypeValue::Tuple(spacetimedb_bindings::TupleValue {
        elements: <[_]>::into_vec(
            #[rustc_box]
            ::alloc::boxed::Box::new([
                spacetimedb_bindings::TypeValue::F32(value.pos_x),
                spacetimedb_bindings::TypeValue::F32(value.pos_y),
                spacetimedb_bindings::TypeValue::F32(value.pos_z),
            ]),
        ),
    });
}
pub struct Rotation {
    pub rot_x: f32,
    pub rot_y: f32,
    pub rot_z: f32,
    pub rot_w: f32,
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for Rotation {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = match _serde::Serializer::serialize_struct(
                __serializer,
                "Rotation",
                false as usize + 1 + 1 + 1 + 1,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "rot_x",
                &self.rot_x,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "rot_y",
                &self.rot_y,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "rot_z",
                &self.rot_z,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "rot_w",
                &self.rot_w,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for Rotation {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                __field0,
                __field1,
                __field2,
                __field3,
                __ignore,
            }
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "field identifier")
                }
                fn visit_u64<__E>(self, __value: u64) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        2u64 => _serde::__private::Ok(__Field::__field2),
                        3u64 => _serde::__private::Ok(__Field::__field3),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "rot_x" => _serde::__private::Ok(__Field::__field0),
                        "rot_y" => _serde::__private::Ok(__Field::__field1),
                        "rot_z" => _serde::__private::Ok(__Field::__field2),
                        "rot_w" => _serde::__private::Ok(__Field::__field3),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"rot_x" => _serde::__private::Ok(__Field::__field0),
                        b"rot_y" => _serde::__private::Ok(__Field::__field1),
                        b"rot_z" => _serde::__private::Ok(__Field::__field2),
                        b"rot_w" => _serde::__private::Ok(__Field::__field3),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                }
            }
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<Rotation>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = Rotation;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct Rotation")
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 =
                        match match _serde::de::SeqAccess::next_element::<f32>(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct Rotation with 4 elements",
                                ));
                            }
                        };
                    let __field1 =
                        match match _serde::de::SeqAccess::next_element::<f32>(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct Rotation with 4 elements",
                                ));
                            }
                        };
                    let __field2 =
                        match match _serde::de::SeqAccess::next_element::<f32>(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    2usize,
                                    &"struct Rotation with 4 elements",
                                ));
                            }
                        };
                    let __field3 =
                        match match _serde::de::SeqAccess::next_element::<f32>(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    3usize,
                                    &"struct Rotation with 4 elements",
                                ));
                            }
                        };
                    _serde::__private::Ok(Rotation {
                        rot_x: __field0,
                        rot_y: __field1,
                        rot_z: __field2,
                        rot_w: __field3,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut __field2: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut __field3: _serde::__private::Option<f32> = _serde::__private::None;
                    while let _serde::__private::Some(__key) =
                        match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        }
                    {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("rot_x"),
                                    );
                                }
                                __field0 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("rot_y"),
                                    );
                                }
                                __field1 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field2 => {
                                if _serde::__private::Option::is_some(&__field2) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("rot_z"),
                                    );
                                }
                                __field2 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field3 => {
                                if _serde::__private::Option::is_some(&__field3) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("rot_w"),
                                    );
                                }
                                __field3 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            _ => {
                                let _ = match _serde::de::MapAccess::next_value::<
                                    _serde::de::IgnoredAny,
                                >(&mut __map)
                                {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                };
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("rot_x") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("rot_y") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field2 = match __field2 {
                        _serde::__private::Some(__field2) => __field2,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("rot_z") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field3 = match __field3 {
                        _serde::__private::Some(__field3) => __field3,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("rot_w") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    _serde::__private::Ok(Rotation {
                        rot_x: __field0,
                        rot_y: __field1,
                        rot_z: __field2,
                        rot_w: __field3,
                    })
                }
            }
            const FIELDS: &'static [&'static str] = &["rot_x", "rot_y", "rot_z", "rot_w"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "Rotation",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<Rotation>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[allow(non_snake_case)]
fn __get_struct_schema__Rotation() -> spacetimedb_bindings::TypeDef {
    return spacetimedb_bindings::TypeDef::Tuple {
        0: spacetimedb_bindings::TupleDef {
            elements: <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    spacetimedb_bindings::ElementDef {
                        tag: 0u8,
                        element_type: spacetimedb_bindings::TypeDef::F32,
                    },
                    spacetimedb_bindings::ElementDef {
                        tag: 1u8,
                        element_type: spacetimedb_bindings::TypeDef::F32,
                    },
                    spacetimedb_bindings::ElementDef {
                        tag: 2u8,
                        element_type: spacetimedb_bindings::TypeDef::F32,
                    },
                    spacetimedb_bindings::ElementDef {
                        tag: 3u8,
                        element_type: spacetimedb_bindings::TypeDef::F32,
                    },
                ]),
            ),
        },
    };
}
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __create_type__Rotation(arg_ptr: usize, arg_size: usize) {
    unsafe {
        let ptr = arg_ptr as *mut u8;
        let def = __get_struct_schema__Rotation();
        let mut bytes = Vec::from_raw_parts(ptr, 0, arg_size);
        def.encode(&mut bytes);
    }
}
#[allow(non_snake_case)]
fn __tuple_to_struct__Rotation(value: spacetimedb_bindings::TupleValue) -> Option<Rotation> {
    let elements_arr = value.elements;
    return match (
        elements_arr[0usize].clone(),
        elements_arr[1usize].clone(),
        elements_arr[2usize].clone(),
        elements_arr[3usize].clone(),
    ) {
        (
            spacetimedb_bindings::TypeValue::F32(field_0),
            spacetimedb_bindings::TypeValue::F32(field_1),
            spacetimedb_bindings::TypeValue::F32(field_2),
            spacetimedb_bindings::TypeValue::F32(field_3),
        ) => Some(Rotation {
            rot_x: field_0,
            rot_y: field_1,
            rot_z: field_2,
            rot_w: field_3,
        }),
        _ => None,
    };
}
#[allow(non_snake_case)]
fn __struct_to_tuple__Rotation(value: Rotation) -> spacetimedb_bindings::TypeValue {
    return spacetimedb_bindings::TypeValue::Tuple(spacetimedb_bindings::TupleValue {
        elements: <[_]>::into_vec(
            #[rustc_box]
            ::alloc::boxed::Box::new([
                spacetimedb_bindings::TypeValue::F32(value.rot_x),
                spacetimedb_bindings::TypeValue::F32(value.rot_y),
                spacetimedb_bindings::TypeValue::F32(value.rot_z),
                spacetimedb_bindings::TypeValue::F32(value.rot_w),
            ]),
        ),
    });
}
pub struct Character {
    #[primary_key]
    ident: CharacterIdentity,
    character_name: String,
}
impl Character {
    #[allow(unused_variables)]
    pub fn insert(ins: Character) {
        unsafe {
            spacetimedb_bindings::insert(
                2u32,
                spacetimedb_bindings::TupleValue {
                    elements: <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([
                            __struct_to_tuple__CharacterIdentity(ins.ident),
                            spacetimedb_bindings::TypeValue::String(ins.character_name),
                        ]),
                    ),
                },
            );
        }
    }
    #[allow(unused_variables)]
    pub fn delete(f: fn(Character) -> bool) -> usize {
        0
    }
    #[allow(unused_variables)]
    pub fn update(value: Character) -> bool {
        false
    }
    #[allow(unused_variables)]
    #[allow(non_snake_case)]
    pub fn filter_ident_eq(ident: CharacterIdentity) -> Option<Character> {
        let table_iter = Character::iter();
        if let Some(table_iter) = table_iter {
            for row in table_iter {
                let data_tuple = row.elements[0usize].clone();
                if let spacetimedb_bindings::TypeValue::Tuple(data) = data_tuple.clone() {
                    let data: Option<CharacterIdentity> =
                        __tuple_to_struct__CharacterIdentity(data);
                    if let None = data {
                        {
                            :: std :: io :: _print (:: core :: fmt :: Arguments :: new_v1 (& ["Internal server error, converting CharacterIdentity tuple to struct!\n"] , & [])) ;
                        };
                        continue;
                    }
                    let data = data.unwrap();
                    let equatable = spacetimedb_bindings::EqTypeValue::try_from(data_tuple);
                    match equatable {
                        Ok(value) => {
                            let value = __tuple_to_struct__Character(row);
                            if let Some(value) = value {
                                return Some(value);
                            }
                        }
                        Err(E) => {
                            {
                                ::std::io::_print(::core::fmt::Arguments::new_v1(
                                    &["This type is not equatable: ", " Error:", "\n"],
                                    &[
                                        ::core::fmt::ArgumentV1::new_display(
                                            &"spacetimedb_bindings :: TypeValue :: Tuple",
                                        ),
                                        ::core::fmt::ArgumentV1::new_display(&E),
                                    ],
                                ));
                            };
                            return None;
                        }
                    }
                }
            }
        }
        return None;
    }
    #[allow(unused_variables)]
    #[allow(non_snake_case)]
    pub fn update_ident_eq(ident: CharacterIdentity, new_value: Character) -> bool {
        Character::delete_ident_eq(ident);
        Character::insert(new_value);
        true
    }
    #[allow(unused_variables)]
    #[allow(non_snake_case)]
    pub fn delete_ident_eq(ident: CharacterIdentity) -> bool {
        let data = ident;
        let data = spacetimedb_bindings::TypeValue::Tuple(data);
        let equatable = spacetimedb_bindings::EqTypeValue::try_from(data);
        match equatable {
            Ok(value) => {
                let result = spacetimedb_bindings::delete_eq(1, 0, value);
                match result {
                    None => {
                        {
                            ::std::io::_print(::core::fmt::Arguments::new_v1(
                                &["Internal server error on equatable type: ", "\n"],
                                &[::core::fmt::ArgumentV1::new_display(
                                    &"spacetimedb_bindings :: TypeValue :: Tuple",
                                )],
                            ));
                        };
                        false
                    }
                    Some(count) => count > 0,
                }
            }
            Err(E) => {
                {
                    ::std::io::_print(::core::fmt::Arguments::new_v1(
                        &["This type is not equatable: ", " Error:", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(
                                &"spacetimedb_bindings :: TypeValue :: Tuple",
                            ),
                            ::core::fmt::ArgumentV1::new_display(&E),
                        ],
                    ));
                };
                false
            }
        }
    }
    #[allow(unused_variables)]
    pub fn iter() -> Option<spacetimedb_bindings::TableIter> {
        spacetimedb_bindings::__iter__(2u32)
    }
    #[allow(non_snake_case)]
    #[allow(unused_variables)]
    pub fn filter_character_name_eq(character_name: String) -> Vec<Character> {
        let mut result = Vec::<Character>::new();
        let table_iter = Character::iter();
        if let Some(table_iter) = table_iter {
            for row in table_iter {
                let tuple_data = row.elements[1usize].clone();
                if let spacetimedb_bindings::TypeValue::String(data) = tuple_data.clone() {
                    let equatable = spacetimedb_bindings::EqTypeValue::try_from(tuple_data);
                    match equatable {
                        Ok(value) => {
                            let value = __tuple_to_struct__Character(row);
                            if let Some(value) = value {
                                result.push(value);
                            }
                        }
                        Err(E) => {
                            {
                                :: std :: io :: _print (:: core :: fmt :: Arguments :: new_v1 (& ["This type is not equatable: filter_character_name_eq Error:" , "\n"] , & [:: core :: fmt :: ArgumentV1 :: new_display (& E)])) ;
                            };
                            return result;
                        }
                    }
                }
            }
        }
        return result;
    }
}
#[allow(non_snake_case)]
fn __get_struct_schema__Character() -> spacetimedb_bindings::TypeDef {
    return spacetimedb_bindings::TypeDef::Tuple {
        0: spacetimedb_bindings::TupleDef {
            elements: <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    spacetimedb_bindings::ElementDef {
                        tag: 0u8,
                        element_type: __get_struct_schema__CharacterIdentity(),
                    },
                    spacetimedb_bindings::ElementDef {
                        tag: 1u8,
                        element_type: spacetimedb_bindings::TypeDef::String,
                    },
                ]),
            ),
        },
    };
}
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn __create_table__Character(arg_ptr: usize, arg_size: usize) {
    unsafe {
        let ptr = arg_ptr as *mut u8;
        let def = __get_struct_schema__Character();
        let mut bytes = Vec::from_raw_parts(ptr, 0, arg_size);
        def.encode(&mut bytes);
        if let spacetimedb_bindings::TypeDef::Tuple(tuple_def) = def {
            spacetimedb_bindings::create_table(2u32, tuple_def);
        } else {
            ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                &["This type is not a tuple: {#original_struct_ident}"],
                &[],
            ));
        }
    }
}
#[allow(non_snake_case)]
fn __tuple_to_struct__Character(value: spacetimedb_bindings::TupleValue) -> Option<Character> {
    let elements_arr = value.elements;
    match (elements_arr[0usize].clone(), elements_arr[1usize].clone()) {
        (
            spacetimedb_bindings::TypeValue::Tuple(field_0),
            spacetimedb_bindings::TypeValue::String(field_1),
        ) => match (__tuple_to_struct__CharacterIdentity(field_0)) {
            ((Some(field_0))) => {
                return Some(Character {
                    ident: field_0,
                    character_name: field_1,
                });
            }
            _ => {}
        },
        _ => {}
    }
    return None;
}
#[allow(non_snake_case)]
fn __struct_to_tuple__Character(value: Character) -> spacetimedb_bindings::TypeValue {
    return spacetimedb_bindings::TypeValue::Tuple(spacetimedb_bindings::TupleValue {
        elements: <[_]>::into_vec(
            #[rustc_box]
            ::alloc::boxed::Box::new([
                __struct_to_tuple__CharacterIdentity(value.ident),
                spacetimedb_bindings::TypeValue::String(value.character_name),
            ]),
        ),
    });
}
pub struct CharacterIdentity {
    pub player_id: Hash,
    pub index: u32,
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for CharacterIdentity {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = match _serde::Serializer::serialize_struct(
                __serializer,
                "CharacterIdentity",
                false as usize + 1 + 1,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "player_id",
                &self.player_id,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "index",
                &self.index,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for CharacterIdentity {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                __field0,
                __field1,
                __ignore,
            }
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "field identifier")
                }
                fn visit_u64<__E>(self, __value: u64) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "player_id" => _serde::__private::Ok(__Field::__field0),
                        "index" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"player_id" => _serde::__private::Ok(__Field::__field0),
                        b"index" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                }
            }
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<CharacterIdentity>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = CharacterIdentity;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct CharacterIdentity")
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 =
                        match match _serde::de::SeqAccess::next_element::<Hash>(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct CharacterIdentity with 2 elements",
                                ));
                            }
                        };
                    let __field1 =
                        match match _serde::de::SeqAccess::next_element::<u32>(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct CharacterIdentity with 2 elements",
                                ));
                            }
                        };
                    _serde::__private::Ok(CharacterIdentity {
                        player_id: __field0,
                        index: __field1,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<Hash> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<u32> = _serde::__private::None;
                    while let _serde::__private::Some(__key) =
                        match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        }
                    {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "player_id",
                                        ),
                                    );
                                }
                                __field0 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Hash>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("index"),
                                    );
                                }
                                __field1 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<u32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            _ => {
                                let _ = match _serde::de::MapAccess::next_value::<
                                    _serde::de::IgnoredAny,
                                >(&mut __map)
                                {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                };
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("player_id") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("index") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    _serde::__private::Ok(CharacterIdentity {
                        player_id: __field0,
                        index: __field1,
                    })
                }
            }
            const FIELDS: &'static [&'static str] = &["player_id", "index"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "CharacterIdentity",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<CharacterIdentity>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[allow(non_snake_case)]
fn __get_struct_schema__CharacterIdentity() -> spacetimedb_bindings::TypeDef {
    return spacetimedb_bindings::TypeDef::Tuple {
        0: spacetimedb_bindings::TupleDef {
            elements: <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    spacetimedb_bindings::ElementDef {
                        tag: 0u8,
                        element_type: spacetimedb_bindings::TypeDef::Bytes,
                    },
                    spacetimedb_bindings::ElementDef {
                        tag: 1u8,
                        element_type: spacetimedb_bindings::TypeDef::U32,
                    },
                ]),
            ),
        },
    };
}
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __create_type__CharacterIdentity(arg_ptr: usize, arg_size: usize) {
    unsafe {
        let ptr = arg_ptr as *mut u8;
        let def = __get_struct_schema__CharacterIdentity();
        let mut bytes = Vec::from_raw_parts(ptr, 0, arg_size);
        def.encode(&mut bytes);
    }
}
#[allow(non_snake_case)]
fn __tuple_to_struct__CharacterIdentity(
    value: spacetimedb_bindings::TupleValue,
) -> Option<CharacterIdentity> {
    let elements_arr = value.elements;
    return match (elements_arr[0usize].clone(), elements_arr[1usize].clone()) {
        (
            spacetimedb_bindings::TypeValue::Bytes(field_0),
            spacetimedb_bindings::TypeValue::U32(field_1),
        ) => {
            let field_0: spacetimedb_bindings::hash::Hash =
                *spacetimedb_bindings::hash::Hash::from_slice(field_0.as_slice());
            Some(CharacterIdentity {
                player_id: field_0,
                index: field_1,
            })
        }
        _ => None,
    };
}
#[allow(non_snake_case)]
fn __struct_to_tuple__CharacterIdentity(
    value: CharacterIdentity,
) -> spacetimedb_bindings::TypeValue {
    return spacetimedb_bindings::TypeValue::Tuple(spacetimedb_bindings::TupleValue {
        elements: <[_]>::into_vec(
            #[rustc_box]
            ::alloc::boxed::Box::new([
                spacetimedb_bindings::TypeValue::Bytes(value.player_id.to_vec()),
                spacetimedb_bindings::TypeValue::U32(value.index),
            ]),
        ),
    });
}
pub struct Player {
    #[primary_key]
    pub player_id: Hash,
    pub creation_time: u64,
    pub position: Position,
    pub rotation: Rotation,
    pub moving: bool,
}
impl Player {
    #[allow(unused_variables)]
    pub fn insert(ins: Player) {
        unsafe {
            spacetimedb_bindings::insert(
                1u32,
                spacetimedb_bindings::TupleValue {
                    elements: <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([
                            spacetimedb_bindings::TypeValue::Bytes(ins.player_id.to_vec()),
                            spacetimedb_bindings::TypeValue::U64(ins.creation_time),
                            __struct_to_tuple__Position(ins.position),
                            __struct_to_tuple__Rotation(ins.rotation),
                            spacetimedb_bindings::TypeValue::Bool(ins.moving),
                        ]),
                    ),
                },
            );
        }
    }
    #[allow(unused_variables)]
    pub fn delete(f: fn(Player) -> bool) -> usize {
        0
    }
    #[allow(unused_variables)]
    pub fn update(value: Player) -> bool {
        false
    }
    #[allow(unused_variables)]
    #[allow(non_snake_case)]
    pub fn filter_player_id_eq(player_id: Hash) -> Option<Player> {
        let table_iter = Player::iter();
        if let Some(table_iter) = table_iter {
            for row in table_iter {
                let data_tuple = row.elements[0usize].clone();
                if let spacetimedb_bindings::TypeValue::Bytes(data) = data_tuple.clone() {
                    let data: spacetimedb_bindings::hash::Hash =
                        *spacetimedb_bindings::hash::Hash::from_slice(&data[0..32]);
                    let equatable = spacetimedb_bindings::EqTypeValue::try_from(data_tuple);
                    match equatable {
                        Ok(value) => {
                            let value = __tuple_to_struct__Player(row);
                            if let Some(value) = value {
                                return Some(value);
                            }
                        }
                        Err(E) => {
                            {
                                ::std::io::_print(::core::fmt::Arguments::new_v1(
                                    &["This type is not equatable: ", " Error:", "\n"],
                                    &[
                                        ::core::fmt::ArgumentV1::new_display(
                                            &"spacetimedb_bindings :: TypeValue :: Bytes",
                                        ),
                                        ::core::fmt::ArgumentV1::new_display(&E),
                                    ],
                                ));
                            };
                            return None;
                        }
                    }
                }
            }
        }
        return None;
    }
    #[allow(unused_variables)]
    #[allow(non_snake_case)]
    pub fn update_player_id_eq(player_id: Hash, new_value: Player) -> bool {
        Player::delete_player_id_eq(player_id);
        Player::insert(new_value);
        true
    }
    #[allow(unused_variables)]
    #[allow(non_snake_case)]
    pub fn delete_player_id_eq(player_id: Hash) -> bool {
        let data = player_id;
        let data = spacetimedb_bindings::TypeValue::Bytes(data.to_vec());
        let equatable = spacetimedb_bindings::EqTypeValue::try_from(data);
        match equatable {
            Ok(value) => {
                let result = spacetimedb_bindings::delete_eq(1, 0, value);
                match result {
                    None => {
                        {
                            ::std::io::_print(::core::fmt::Arguments::new_v1(
                                &["Internal server error on equatable type: ", "\n"],
                                &[::core::fmt::ArgumentV1::new_display(
                                    &"spacetimedb_bindings :: TypeValue :: Bytes",
                                )],
                            ));
                        };
                        false
                    }
                    Some(count) => count > 0,
                }
            }
            Err(E) => {
                {
                    ::std::io::_print(::core::fmt::Arguments::new_v1(
                        &["This type is not equatable: ", " Error:", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(
                                &"spacetimedb_bindings :: TypeValue :: Bytes",
                            ),
                            ::core::fmt::ArgumentV1::new_display(&E),
                        ],
                    ));
                };
                false
            }
        }
    }
    #[allow(unused_variables)]
    pub fn iter() -> Option<spacetimedb_bindings::TableIter> {
        spacetimedb_bindings::__iter__(1u32)
    }
    #[allow(non_snake_case)]
    #[allow(unused_variables)]
    pub fn filter_creation_time_eq(creation_time: u64) -> Vec<Player> {
        let mut result = Vec::<Player>::new();
        let table_iter = Player::iter();
        if let Some(table_iter) = table_iter {
            for row in table_iter {
                let tuple_data = row.elements[1usize].clone();
                if let spacetimedb_bindings::TypeValue::U64(data) = tuple_data.clone() {
                    let equatable = spacetimedb_bindings::EqTypeValue::try_from(tuple_data);
                    match equatable {
                        Ok(value) => {
                            let value = __tuple_to_struct__Player(row);
                            if let Some(value) = value {
                                result.push(value);
                            }
                        }
                        Err(E) => {
                            {
                                :: std :: io :: _print (:: core :: fmt :: Arguments :: new_v1 (& ["This type is not equatable: filter_creation_time_eq Error:" , "\n"] , & [:: core :: fmt :: ArgumentV1 :: new_display (& E)])) ;
                            };
                            return result;
                        }
                    }
                }
            }
        }
        return result;
    }
    #[allow(non_snake_case)]
    #[allow(unused_variables)]
    pub fn filter_position_eq(position: Position) -> Vec<Player> {
        let mut result = Vec::<Player>::new();
        let table_iter = Player::iter();
        if let Some(table_iter) = table_iter {
            for row in table_iter {
                let tuple_data = row.elements[2usize].clone();
                if let spacetimedb_bindings::TypeValue::Tuple(data) = tuple_data.clone() {
                    let data: Option<Position> = __tuple_to_struct__Position(data);
                    if let None = data {
                        {
                            ::std::io::_print(::core::fmt::Arguments::new_v1(
                                &["Internal server error, converting Position tuple to struct!\n"],
                                &[],
                            ));
                        };
                        continue;
                    }
                    let data = data.unwrap();
                    let equatable = spacetimedb_bindings::EqTypeValue::try_from(tuple_data);
                    match equatable {
                        Ok(value) => {
                            let value = __tuple_to_struct__Player(row);
                            if let Some(value) = value {
                                result.push(value);
                            }
                        }
                        Err(E) => {
                            {
                                ::std::io::_print(::core::fmt::Arguments::new_v1(
                                    &[
                                        "This type is not equatable: filter_position_eq Error:",
                                        "\n",
                                    ],
                                    &[::core::fmt::ArgumentV1::new_display(&E)],
                                ));
                            };
                            return result;
                        }
                    }
                }
            }
        }
        return result;
    }
    #[allow(non_snake_case)]
    #[allow(unused_variables)]
    pub fn filter_rotation_eq(rotation: Rotation) -> Vec<Player> {
        let mut result = Vec::<Player>::new();
        let table_iter = Player::iter();
        if let Some(table_iter) = table_iter {
            for row in table_iter {
                let tuple_data = row.elements[3usize].clone();
                if let spacetimedb_bindings::TypeValue::Tuple(data) = tuple_data.clone() {
                    let data: Option<Rotation> = __tuple_to_struct__Rotation(data);
                    if let None = data {
                        {
                            ::std::io::_print(::core::fmt::Arguments::new_v1(
                                &["Internal server error, converting Rotation tuple to struct!\n"],
                                &[],
                            ));
                        };
                        continue;
                    }
                    let data = data.unwrap();
                    let equatable = spacetimedb_bindings::EqTypeValue::try_from(tuple_data);
                    match equatable {
                        Ok(value) => {
                            let value = __tuple_to_struct__Player(row);
                            if let Some(value) = value {
                                result.push(value);
                            }
                        }
                        Err(E) => {
                            {
                                ::std::io::_print(::core::fmt::Arguments::new_v1(
                                    &[
                                        "This type is not equatable: filter_rotation_eq Error:",
                                        "\n",
                                    ],
                                    &[::core::fmt::ArgumentV1::new_display(&E)],
                                ));
                            };
                            return result;
                        }
                    }
                }
            }
        }
        return result;
    }
    #[allow(non_snake_case)]
    #[allow(unused_variables)]
    pub fn filter_moving_eq(moving: bool) -> Vec<Player> {
        let mut result = Vec::<Player>::new();
        let table_iter = Player::iter();
        if let Some(table_iter) = table_iter {
            for row in table_iter {
                let tuple_data = row.elements[4usize].clone();
                if let spacetimedb_bindings::TypeValue::Bool(data) = tuple_data.clone() {
                    let equatable = spacetimedb_bindings::EqTypeValue::try_from(tuple_data);
                    match equatable {
                        Ok(value) => {
                            let value = __tuple_to_struct__Player(row);
                            if let Some(value) = value {
                                result.push(value);
                            }
                        }
                        Err(E) => {
                            {
                                ::std::io::_print(::core::fmt::Arguments::new_v1(
                                    &["This type is not equatable: filter_moving_eq Error:", "\n"],
                                    &[::core::fmt::ArgumentV1::new_display(&E)],
                                ));
                            };
                            return result;
                        }
                    }
                }
            }
        }
        return result;
    }
}
#[allow(non_snake_case)]
fn __get_struct_schema__Player() -> spacetimedb_bindings::TypeDef {
    return spacetimedb_bindings::TypeDef::Tuple {
        0: spacetimedb_bindings::TupleDef {
            elements: <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    spacetimedb_bindings::ElementDef {
                        tag: 0u8,
                        element_type: spacetimedb_bindings::TypeDef::Bytes,
                    },
                    spacetimedb_bindings::ElementDef {
                        tag: 1u8,
                        element_type: spacetimedb_bindings::TypeDef::U64,
                    },
                    spacetimedb_bindings::ElementDef {
                        tag: 2u8,
                        element_type: __get_struct_schema__Position(),
                    },
                    spacetimedb_bindings::ElementDef {
                        tag: 3u8,
                        element_type: __get_struct_schema__Rotation(),
                    },
                    spacetimedb_bindings::ElementDef {
                        tag: 4u8,
                        element_type: spacetimedb_bindings::TypeDef::Bool,
                    },
                ]),
            ),
        },
    };
}
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn __create_table__Player(arg_ptr: usize, arg_size: usize) {
    unsafe {
        let ptr = arg_ptr as *mut u8;
        let def = __get_struct_schema__Player();
        let mut bytes = Vec::from_raw_parts(ptr, 0, arg_size);
        def.encode(&mut bytes);
        if let spacetimedb_bindings::TypeDef::Tuple(tuple_def) = def {
            spacetimedb_bindings::create_table(1u32, tuple_def);
        } else {
            ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                &["This type is not a tuple: {#original_struct_ident}"],
                &[],
            ));
        }
    }
}
#[allow(non_snake_case)]
fn __tuple_to_struct__Player(value: spacetimedb_bindings::TupleValue) -> Option<Player> {
    let elements_arr = value.elements;
    match (
        elements_arr[0usize].clone(),
        elements_arr[1usize].clone(),
        elements_arr[2usize].clone(),
        elements_arr[3usize].clone(),
        elements_arr[4usize].clone(),
    ) {
        (
            spacetimedb_bindings::TypeValue::Bytes(field_0),
            spacetimedb_bindings::TypeValue::U64(field_1),
            spacetimedb_bindings::TypeValue::Tuple(field_2),
            spacetimedb_bindings::TypeValue::Tuple(field_3),
            spacetimedb_bindings::TypeValue::Bool(field_4),
        ) => {
            match (
                __tuple_to_struct__Position(field_2),
                __tuple_to_struct__Rotation(field_3),
            ) {
                ((Some(field_2), Some(field_3))) => {
                    let field_0: spacetimedb_bindings::hash::Hash =
                        *spacetimedb_bindings::hash::Hash::from_slice(field_0.as_slice());
                    return Some(Player {
                        player_id: field_0,
                        creation_time: field_1,
                        position: field_2,
                        rotation: field_3,
                        moving: field_4,
                    });
                }
                _ => {}
            }
        }
        _ => {}
    }
    return None;
}
#[allow(non_snake_case)]
fn __struct_to_tuple__Player(value: Player) -> spacetimedb_bindings::TypeValue {
    return spacetimedb_bindings::TypeValue::Tuple(spacetimedb_bindings::TupleValue {
        elements: <[_]>::into_vec(
            #[rustc_box]
            ::alloc::boxed::Box::new([
                spacetimedb_bindings::TypeValue::Bytes(value.player_id.to_vec()),
                spacetimedb_bindings::TypeValue::U64(value.creation_time),
                __struct_to_tuple__Position(value.position),
                __struct_to_tuple__Rotation(value.rotation),
                spacetimedb_bindings::TypeValue::Bool(value.moving),
            ]),
        ),
    });
}
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __reducer__move_player(arg_ptr: usize, arg_size: usize) {
    const HEADER_SIZE: usize = 40;
    let arg_ptr = arg_ptr as *mut u8;
    let bytes: Vec<u8> =
        unsafe { Vec::from_raw_parts(arg_ptr, arg_size + HEADER_SIZE, arg_size + HEADER_SIZE) };
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(&["Parsing sender\n"], &[]));
    };
    let sender = *spacetimedb_bindings::hash::Hash::from_slice(&bytes[0..32]);
    let mut buf = [0; 8];
    buf.copy_from_slice(&bytes[32..HEADER_SIZE]);
    let timestamp = u64::from_le_bytes(buf);
    let arg_json: serde_json::Value = serde_json::from_slice(&bytes[HEADER_SIZE..]).unwrap();
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(&["unwrapping args\n"], &[]));
    };
    let args = arg_json.as_array().unwrap();
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["deserialize arguments\n"],
            &[],
        ));
    };
    let arg_2: Position = serde_json::from_value(args[0usize].clone()).unwrap();
    let arg_3: Rotation = serde_json::from_value(args[1usize].clone()).unwrap();
    let arg_4: bool = serde_json::from_value(args[2usize].clone()).unwrap();
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["invoke function call\n"],
            &[],
        ));
    };
    move_player(sender, timestamp, arg_2, arg_3, arg_4);
}
pub fn move_player(
    player_id: Hash,
    _timestamp: u64,
    position: Position,
    rotation: Rotation,
    moving: bool,
) {
    let player = Player::filter_player_id_eq(player_id.clone());
    match player {
        Some(mut player) => {
            player.position = position;
            player.rotation = rotation;
            player.moving = moving;
            Player::update_player_id_eq(player_id, player);
        }
        None => {}
    }
}
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __reducer__create_new_player(arg_ptr: usize, arg_size: usize) {
    const HEADER_SIZE: usize = 40;
    let arg_ptr = arg_ptr as *mut u8;
    let bytes: Vec<u8> =
        unsafe { Vec::from_raw_parts(arg_ptr, arg_size + HEADER_SIZE, arg_size + HEADER_SIZE) };
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(&["Parsing sender\n"], &[]));
    };
    let sender = *spacetimedb_bindings::hash::Hash::from_slice(&bytes[0..32]);
    let mut buf = [0; 8];
    buf.copy_from_slice(&bytes[32..HEADER_SIZE]);
    let timestamp = u64::from_le_bytes(buf);
    let arg_json: serde_json::Value = serde_json::from_slice(&bytes[HEADER_SIZE..]).unwrap();
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(&["unwrapping args\n"], &[]));
    };
    let args = arg_json.as_array().unwrap();
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["deserialize arguments\n"],
            &[],
        ));
    };
    let arg_2: Position = serde_json::from_value(args[0usize].clone()).unwrap();
    let arg_3: Rotation = serde_json::from_value(args[1usize].clone()).unwrap();
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["invoke function call\n"],
            &[],
        ));
    };
    create_new_player(sender, timestamp, arg_2, arg_3);
}
pub fn create_new_player(player_id: Hash, timestamp: u64, position: Position, rotation: Rotation) {
    let ident = CharacterIdentity {
        player_id,
        index: 0,
    };
    Character::insert(Character {
        ident: ident.clone(),
        character_name: "My Character".to_string(),
    });
    let got_character = Character::filter_ident(ident);
    match got_character {
        Some(a) => {
            if a.character_name.eq("My Character".to_string()) {
                ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(&["Success!"], &[]));
            }
            ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(&["Failure 2"], &[]));
        }
        None => {
            ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(&["Failure!"], &[]));
        }
    }
}
