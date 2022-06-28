#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use spacetimedb_bindgen::spacetimedb;
pub struct Pocket {
    pub item_id: u32,
    pub item_count: u32,
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for Pocket {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = match _serde::Serializer::serialize_struct(
                __serializer,
                "Pocket",
                false as usize + 1 + 1,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "item_id",
                &self.item_id,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "item_count",
                &self.item_count,
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
    impl<'de> _serde::Deserialize<'de> for Pocket {
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
                        "item_id" => _serde::__private::Ok(__Field::__field0),
                        "item_count" => _serde::__private::Ok(__Field::__field1),
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
                        b"item_id" => _serde::__private::Ok(__Field::__field0),
                        b"item_count" => _serde::__private::Ok(__Field::__field1),
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
                marker: _serde::__private::PhantomData<Pocket>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = Pocket;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct Pocket")
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
                        match match _serde::de::SeqAccess::next_element::<u32>(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct Pocket with 2 elements",
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
                                    &"struct Pocket with 2 elements",
                                ));
                            }
                        };
                    _serde::__private::Ok(Pocket {
                        item_id: __field0,
                        item_count: __field1,
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
                    let mut __field0: _serde::__private::Option<u32> = _serde::__private::None;
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
                                            "item_id",
                                        ),
                                    );
                                }
                                __field0 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<u32>(&mut __map) {
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
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "item_count",
                                        ),
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
                            match _serde::__private::de::missing_field("item_id") {
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
                            match _serde::__private::de::missing_field("item_count") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    _serde::__private::Ok(Pocket {
                        item_id: __field0,
                        item_count: __field1,
                    })
                }
            }
            const FIELDS: &'static [&'static str] = &["item_id", "item_count"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "Pocket",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<Pocket>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[allow(non_snake_case)]
fn __get_struct_schema__Pocket() -> spacetimedb_bindings::TypeDef {
    return spacetimedb_bindings::TypeDef::Tuple {
        0: spacetimedb_bindings::TupleDef {
            elements: <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    spacetimedb_bindings::ElementDef {
                        tag: 0u8,
                        element_type: Box::new(spacetimedb_bindings::TypeDef::U32),
                    },
                    spacetimedb_bindings::ElementDef {
                        tag: 1u8,
                        element_type: Box::new(spacetimedb_bindings::TypeDef::U32),
                    },
                ]),
            ),
        },
    };
}
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __create_type__Pocket(arg_ptr: usize, arg_size: usize) {
    unsafe {
        let ptr = arg_ptr as *mut u8;
        let def = __get_struct_schema__Pocket();
        let mut bytes = Vec::from_raw_parts(ptr, 0, arg_size);
        def.encode(&mut bytes);
    }
}
#[allow(non_snake_case)]
fn __tuple_to_struct__Pocket(value: spacetimedb_bindings::TupleValue) -> Option<Pocket> {
    let elements_arr = value.elements;
    return match (elements_arr[0usize].clone(), elements_arr[1usize].clone()) {
        (
            spacetimedb_bindings::TypeValue::U32(field_0),
            spacetimedb_bindings::TypeValue::U32(field_1),
        ) => Some(Pocket {
            item_id: field_0,
            item_count: field_1,
        }),
        _ => None,
    };
}
#[allow(non_snake_case)]
fn __struct_to_tuple__Pocket(value: Pocket) -> spacetimedb_bindings::TypeValue {
    return spacetimedb_bindings::TypeValue::Tuple(spacetimedb_bindings::TupleValue {
        elements: <[_]>::into_vec(
            #[rustc_box]
            ::alloc::boxed::Box::new([
                spacetimedb_bindings::TypeValue::U32(value.item_id),
                spacetimedb_bindings::TypeValue::U32(value.item_count),
            ]),
        ),
    });
}
pub struct Inventory {
    pub pocket_count: u32,
    pub pocket: Pocket,
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for Inventory {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = match _serde::Serializer::serialize_struct(
                __serializer,
                "Inventory",
                false as usize + 1 + 1,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "pocket_count",
                &self.pocket_count,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "pocket",
                &self.pocket,
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
    impl<'de> _serde::Deserialize<'de> for Inventory {
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
                        "pocket_count" => _serde::__private::Ok(__Field::__field0),
                        "pocket" => _serde::__private::Ok(__Field::__field1),
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
                        b"pocket_count" => _serde::__private::Ok(__Field::__field0),
                        b"pocket" => _serde::__private::Ok(__Field::__field1),
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
                marker: _serde::__private::PhantomData<Inventory>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = Inventory;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct Inventory")
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
                        match match _serde::de::SeqAccess::next_element::<u32>(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct Inventory with 2 elements",
                                ));
                            }
                        };
                    let __field1 =
                        match match _serde::de::SeqAccess::next_element::<Pocket>(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct Inventory with 2 elements",
                                ));
                            }
                        };
                    _serde::__private::Ok(Inventory {
                        pocket_count: __field0,
                        pocket: __field1,
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
                    let mut __field0: _serde::__private::Option<u32> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<Pocket> = _serde::__private::None;
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
                                            "pocket_count",
                                        ),
                                    );
                                }
                                __field0 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<u32>(&mut __map) {
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
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "pocket",
                                        ),
                                    );
                                }
                                __field1 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Pocket>(&mut __map) {
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
                            match _serde::__private::de::missing_field("pocket_count") {
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
                            match _serde::__private::de::missing_field("pocket") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    _serde::__private::Ok(Inventory {
                        pocket_count: __field0,
                        pocket: __field1,
                    })
                }
            }
            const FIELDS: &'static [&'static str] = &["pocket_count", "pocket"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "Inventory",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<Inventory>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[allow(non_snake_case)]
fn __get_struct_schema__Inventory() -> spacetimedb_bindings::TypeDef {
    return spacetimedb_bindings::TypeDef::Tuple {
        0: spacetimedb_bindings::TupleDef {
            elements: <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    spacetimedb_bindings::ElementDef {
                        tag: 0u8,
                        element_type: Box::new(spacetimedb_bindings::TypeDef::U32),
                    },
                    spacetimedb_bindings::ElementDef {
                        tag: 1u8,
                        element_type: Box::new(__get_struct_schema__Pocket()),
                    },
                ]),
            ),
        },
    };
}
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __create_type__Inventory(arg_ptr: usize, arg_size: usize) {
    unsafe {
        let ptr = arg_ptr as *mut u8;
        let def = __get_struct_schema__Inventory();
        let mut bytes = Vec::from_raw_parts(ptr, 0, arg_size);
        def.encode(&mut bytes);
    }
}
#[allow(non_snake_case)]
fn __tuple_to_struct__Inventory(value: spacetimedb_bindings::TupleValue) -> Option<Inventory> {
    let elements_arr = value.elements;
    match (elements_arr[0usize].clone(), elements_arr[1usize].clone()) {
        (
            spacetimedb_bindings::TypeValue::U32(field_0),
            spacetimedb_bindings::TypeValue::Tuple(field_1),
        ) => match (__tuple_to_struct__Pocket(field_1)) {
            ((Some(field_1))) => {
                return Some(Inventory {
                    pocket_count: field_0,
                    pocket: field_1,
                });
            }
            _ => {}
        },
        _ => {}
    }
    return None;
}
#[allow(non_snake_case)]
fn __struct_to_tuple__Inventory(value: Inventory) -> spacetimedb_bindings::TypeValue {
    return spacetimedb_bindings::TypeValue::Tuple(spacetimedb_bindings::TupleValue {
        elements: <[_]>::into_vec(
            #[rustc_box]
            ::alloc::boxed::Box::new([
                spacetimedb_bindings::TypeValue::U32(value.pocket_count),
                __struct_to_tuple__Pocket(value.pocket),
            ]),
        ),
    });
}
pub struct Player {
    #[primary_key]
    pub player_id: u32,
    pub inventory: Inventory,
    pub inventory2: Inventory,
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
    impl _serde::Serialize for Player {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = match _serde::Serializer::serialize_struct(
                __serializer,
                "Player",
                false as usize + 1 + 1 + 1 + 1 + 1 + 1,
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
                "inventory",
                &self.inventory,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "inventory2",
                &self.inventory2,
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
    impl<'de> _serde::Deserialize<'de> for Player {
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
                __field4,
                __field5,
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
                        4u64 => _serde::__private::Ok(__Field::__field4),
                        5u64 => _serde::__private::Ok(__Field::__field5),
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
                        "inventory" => _serde::__private::Ok(__Field::__field1),
                        "inventory2" => _serde::__private::Ok(__Field::__field2),
                        "pos_x" => _serde::__private::Ok(__Field::__field3),
                        "pos_y" => _serde::__private::Ok(__Field::__field4),
                        "pos_z" => _serde::__private::Ok(__Field::__field5),
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
                        b"inventory" => _serde::__private::Ok(__Field::__field1),
                        b"inventory2" => _serde::__private::Ok(__Field::__field2),
                        b"pos_x" => _serde::__private::Ok(__Field::__field3),
                        b"pos_y" => _serde::__private::Ok(__Field::__field4),
                        b"pos_z" => _serde::__private::Ok(__Field::__field5),
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
                marker: _serde::__private::PhantomData<Player>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = Player;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct Player")
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
                        match match _serde::de::SeqAccess::next_element::<u32>(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct Player with 6 elements",
                                ));
                            }
                        };
                    let __field1 =
                        match match _serde::de::SeqAccess::next_element::<Inventory>(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct Player with 6 elements",
                                ));
                            }
                        };
                    let __field2 =
                        match match _serde::de::SeqAccess::next_element::<Inventory>(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    2usize,
                                    &"struct Player with 6 elements",
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
                                    &"struct Player with 6 elements",
                                ));
                            }
                        };
                    let __field4 =
                        match match _serde::de::SeqAccess::next_element::<f32>(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    4usize,
                                    &"struct Player with 6 elements",
                                ));
                            }
                        };
                    let __field5 =
                        match match _serde::de::SeqAccess::next_element::<f32>(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    5usize,
                                    &"struct Player with 6 elements",
                                ));
                            }
                        };
                    _serde::__private::Ok(Player {
                        player_id: __field0,
                        inventory: __field1,
                        inventory2: __field2,
                        pos_x: __field3,
                        pos_y: __field4,
                        pos_z: __field5,
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
                    let mut __field0: _serde::__private::Option<u32> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<Inventory> =
                        _serde::__private::None;
                    let mut __field2: _serde::__private::Option<Inventory> =
                        _serde::__private::None;
                    let mut __field3: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut __field4: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut __field5: _serde::__private::Option<f32> = _serde::__private::None;
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
                                    match _serde::de::MapAccess::next_value::<u32>(&mut __map) {
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
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "inventory",
                                        ),
                                    );
                                }
                                __field1 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Inventory>(&mut __map)
                                    {
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
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "inventory2",
                                        ),
                                    );
                                }
                                __field2 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Inventory>(&mut __map)
                                    {
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
                                        <__A::Error as _serde::de::Error>::duplicate_field("pos_x"),
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
                            __Field::__field4 => {
                                if _serde::__private::Option::is_some(&__field4) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("pos_y"),
                                    );
                                }
                                __field4 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field5 => {
                                if _serde::__private::Option::is_some(&__field5) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("pos_z"),
                                    );
                                }
                                __field5 = _serde::__private::Some(
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
                            match _serde::__private::de::missing_field("inventory") {
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
                            match _serde::__private::de::missing_field("inventory2") {
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
                            match _serde::__private::de::missing_field("pos_x") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field4 = match __field4 {
                        _serde::__private::Some(__field4) => __field4,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("pos_y") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field5 = match __field5 {
                        _serde::__private::Some(__field5) => __field5,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("pos_z") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    _serde::__private::Ok(Player {
                        player_id: __field0,
                        inventory: __field1,
                        inventory2: __field2,
                        pos_x: __field3,
                        pos_y: __field4,
                        pos_z: __field5,
                    })
                }
            }
            const FIELDS: &'static [&'static str] = &[
                "player_id",
                "inventory",
                "inventory2",
                "pos_x",
                "pos_y",
                "pos_z",
            ];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "Player",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<Player>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
static mut __table_id__Player: u32 = 0;
impl Player {
    #[allow(unused_variables)]
    pub fn insert(ins: Player) {
        unsafe {
            spacetimedb_bindings::insert(
                __table_id__Player,
                spacetimedb_bindings::TupleValue {
                    elements: <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([
                            spacetimedb_bindings::TypeValue::U32(ins.player_id),
                            __struct_to_tuple__Inventory(ins.inventory),
                            __struct_to_tuple__Inventory(ins.inventory2),
                            spacetimedb_bindings::TypeValue::F32(ins.pos_x),
                            spacetimedb_bindings::TypeValue::F32(ins.pos_y),
                            spacetimedb_bindings::TypeValue::F32(ins.pos_z),
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
    pub fn filter_player_id_eq(player_id: u32) -> Option<Player> {
        unsafe {
            let table_iter = spacetimedb_bindings::iter(__table_id__Player);
            if let Some(table_iter) = table_iter {
                for row in table_iter {
                    let data = row.elements[0usize].clone();
                    if let spacetimedb_bindings::TypeValue::U32(data) = data {
                        if player_id == data {
                            let value = __tuple_to_struct__Player(row);
                            if let Some(value) = value {
                                return Some(value);
                            }
                        }
                    }
                }
            }
        }
        return None;
    }
    #[allow(unused_variables)]
    #[allow(non_snake_case)]
    pub fn delete_player_id_eq(player_id: u32) -> bool {
        false
    }
    #[allow(non_snake_case)]
    #[allow(unused_variables)]
    pub fn filter_pos_x_eq(pos_x: f32) -> Vec<Player> {
        unsafe {
            let mut result = Vec::<Player>::new();
            let table_iter = spacetimedb_bindings::iter(__table_id__Player);
            if let Some(table_iter) = table_iter {
                for row in table_iter {
                    let data = row.elements[0usize].clone();
                    if let spacetimedb_bindings::TypeValue::F32(data) = data {
                        if pos_x == data {
                            let value = __tuple_to_struct__Player(row);
                            if let Some(value) = value {
                                result.push(value);
                            }
                        }
                    }
                }
            }
            return result;
        }
    }
    #[allow(non_snake_case)]
    #[allow(unused_variables)]
    pub fn delete_pos_x_eq(pos_x: f32) -> usize {
        0
    }
    #[allow(non_snake_case)]
    #[allow(unused_variables)]
    pub fn filter_pos_y_eq(pos_y: f32) -> Vec<Player> {
        unsafe {
            let mut result = Vec::<Player>::new();
            let table_iter = spacetimedb_bindings::iter(__table_id__Player);
            if let Some(table_iter) = table_iter {
                for row in table_iter {
                    let data = row.elements[1usize].clone();
                    if let spacetimedb_bindings::TypeValue::F32(data) = data {
                        if pos_y == data {
                            let value = __tuple_to_struct__Player(row);
                            if let Some(value) = value {
                                result.push(value);
                            }
                        }
                    }
                }
            }
            return result;
        }
    }
    #[allow(non_snake_case)]
    #[allow(unused_variables)]
    pub fn delete_pos_y_eq(pos_y: f32) -> usize {
        0
    }
    #[allow(non_snake_case)]
    #[allow(unused_variables)]
    pub fn filter_pos_z_eq(pos_z: f32) -> Vec<Player> {
        unsafe {
            let mut result = Vec::<Player>::new();
            let table_iter = spacetimedb_bindings::iter(__table_id__Player);
            if let Some(table_iter) = table_iter {
                for row in table_iter {
                    let data = row.elements[2usize].clone();
                    if let spacetimedb_bindings::TypeValue::F32(data) = data {
                        if pos_z == data {
                            let value = __tuple_to_struct__Player(row);
                            if let Some(value) = value {
                                result.push(value);
                            }
                        }
                    }
                }
            }
            return result;
        }
    }
    #[allow(non_snake_case)]
    #[allow(unused_variables)]
    pub fn delete_pos_z_eq(pos_z: f32) -> usize {
        0
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
                        element_type: Box::new(spacetimedb_bindings::TypeDef::U32),
                    },
                    spacetimedb_bindings::ElementDef {
                        tag: 1u8,
                        element_type: Box::new(__get_struct_schema__Inventory()),
                    },
                    spacetimedb_bindings::ElementDef {
                        tag: 2u8,
                        element_type: Box::new(__get_struct_schema__Inventory()),
                    },
                    spacetimedb_bindings::ElementDef {
                        tag: 3u8,
                        element_type: Box::new(spacetimedb_bindings::TypeDef::F32),
                    },
                    spacetimedb_bindings::ElementDef {
                        tag: 4u8,
                        element_type: Box::new(spacetimedb_bindings::TypeDef::F32),
                    },
                    spacetimedb_bindings::ElementDef {
                        tag: 5u8,
                        element_type: Box::new(spacetimedb_bindings::TypeDef::F32),
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
        elements_arr[5usize].clone(),
    ) {
        (
            spacetimedb_bindings::TypeValue::U32(field_0),
            spacetimedb_bindings::TypeValue::Tuple(field_1),
            spacetimedb_bindings::TypeValue::Tuple(field_2),
            spacetimedb_bindings::TypeValue::F32(field_3),
            spacetimedb_bindings::TypeValue::F32(field_4),
            spacetimedb_bindings::TypeValue::F32(field_5),
        ) => {
            match (
                __tuple_to_struct__Inventory(field_1),
                __tuple_to_struct__Inventory(field_2),
            ) {
                ((Some(field_1), Some(field_2))) => {
                    return Some(Player {
                        player_id: field_0,
                        inventory: field_1,
                        inventory2: field_2,
                        pos_x: field_3,
                        pos_y: field_4,
                        pos_z: field_5,
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
                spacetimedb_bindings::TypeValue::U32(value.player_id),
                __struct_to_tuple__Inventory(value.inventory),
                __struct_to_tuple__Inventory(value.inventory2),
                spacetimedb_bindings::TypeValue::F32(value.pos_x),
                spacetimedb_bindings::TypeValue::F32(value.pos_y),
                spacetimedb_bindings::TypeValue::F32(value.pos_z),
            ]),
        ),
    });
}
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __reducer__move_player(arg_ptr: usize, arg_size: usize) {
    let arg_ptr = arg_ptr as *mut u8;
    let bytes: Vec<u8> = unsafe { Vec::from_raw_parts(arg_ptr, arg_size, arg_size) };
    let arg_json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    let args = arg_json.as_array().unwrap();
    let arg_0: u32 = serde_json::from_value(args[0usize].clone()).unwrap();
    let arg_1: f32 = serde_json::from_value(args[1usize].clone()).unwrap();
    let arg_2: f32 = serde_json::from_value(args[2usize].clone()).unwrap();
    let arg_3: f32 = serde_json::from_value(args[3usize].clone()).unwrap();
    move_player(arg_0, arg_1, arg_2, arg_3);
}
#[allow(unused)]
pub fn move_player(player_id: u32, x: f32, y: f32, z: f32) {
    let player = Player::filter_player_id_eq(player_id);
    match player {
        Some(mut player) => {
            player.pos_x = x;
            player.pos_y = y;
            player.pos_z = z;
        }
        None => {}
    }
}
