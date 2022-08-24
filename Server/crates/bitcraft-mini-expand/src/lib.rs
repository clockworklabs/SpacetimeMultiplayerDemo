#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use spacetimedb_bindgen::spacetimedb;
use spacetimedb_bindings::hash::Hash;
pub struct Player {
    #[unique]
    pub entity_id: u32,
    #[unique]
    pub owner_id: Hash,
    pub creation_time: u64,
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
                false as usize + 1 + 1 + 1,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "entity_id",
                &self.entity_id,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "owner_id",
                &self.owner_id,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "creation_time",
                &self.creation_time,
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
                        "entity_id" => _serde::__private::Ok(__Field::__field0),
                        "owner_id" => _serde::__private::Ok(__Field::__field1),
                        "creation_time" => _serde::__private::Ok(__Field::__field2),
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
                        b"entity_id" => _serde::__private::Ok(__Field::__field0),
                        b"owner_id" => _serde::__private::Ok(__Field::__field1),
                        b"creation_time" => _serde::__private::Ok(__Field::__field2),
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
                                    &"struct Player with 3 elements",
                                ));
                            }
                        };
                    let __field1 =
                        match match _serde::de::SeqAccess::next_element::<Hash>(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct Player with 3 elements",
                                ));
                            }
                        };
                    let __field2 =
                        match match _serde::de::SeqAccess::next_element::<u64>(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    2usize,
                                    &"struct Player with 3 elements",
                                ));
                            }
                        };
                    _serde::__private::Ok(Player {
                        entity_id: __field0,
                        owner_id: __field1,
                        creation_time: __field2,
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
                    let mut __field1: _serde::__private::Option<Hash> = _serde::__private::None;
                    let mut __field2: _serde::__private::Option<u64> = _serde::__private::None;
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
                                            "entity_id",
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
                                            "owner_id",
                                        ),
                                    );
                                }
                                __field1 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Hash>(&mut __map) {
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
                                            "creation_time",
                                        ),
                                    );
                                }
                                __field2 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<u64>(&mut __map) {
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
                            match _serde::__private::de::missing_field("entity_id") {
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
                            match _serde::__private::de::missing_field("owner_id") {
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
                            match _serde::__private::de::missing_field("creation_time") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    _serde::__private::Ok(Player {
                        entity_id: __field0,
                        owner_id: __field1,
                        creation_time: __field2,
                    })
                }
            }
            const FIELDS: &'static [&'static str] = &["entity_id", "owner_id", "creation_time"];
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
impl Player {
    #[allow(unused_variables)]
    pub fn insert(ins: Player) {
        spacetimedb_bindings::insert(
            1u32,
            spacetimedb_bindings::TupleValue {
                elements: <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        spacetimedb_bindings::TypeValue::U32(ins.entity_id),
                        spacetimedb_bindings::TypeValue::U64(ins.creation_time),
                    ]),
                ),
            },
        );
    }
    #[allow(unused_variables)]
    pub fn delete(f: fn(Player) -> bool) -> usize {
        ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
            &["Delete using a function is not supported yet!"],
            &[],
        ));
    }
    #[allow(unused_variables)]
    pub fn update(value: Player) -> bool {
        ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
            &["Update using a value is not supported yet!"],
            &[],
        ));
    }
    #[allow(unused_variables)]
    #[allow(non_snake_case)]
    pub fn filter_entity_id_eq(entity_id: u32) -> Option<Player> {
        let table_iter = Player::iter();
        if let Some(table_iter) = table_iter {
            for row in table_iter {
                let column_data = row.elements[0usize].clone();
                if let spacetimedb_bindings::TypeValue::U32(entry_data) = column_data.clone() {
                    if entry_data == entity_id {
                        let tuple = __tuple_to_struct__Player(row);
                        if let None = tuple {
                            ::spacetimedb_bindings::_console_log_info(&{
                                let res = :: alloc :: fmt :: format (:: core :: fmt :: Arguments :: new_v1 (& ["Internal stdb error: Can\'t convert from tuple to struct (wrong version?) Player"] , & [])) ;
                                res
                            });
                            return None;
                        }
                        return Some(tuple.unwrap());
                    }
                }
            }
        } else {
            ::spacetimedb_bindings::_console_log_info(&{
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &["Failed to get iterator for table: Player"],
                    &[],
                ));
                res
            });
        }
        return None;
    }
    #[allow(unused_variables)]
    #[allow(non_snake_case)]
    pub fn filter_owner_id_eq(owner_id: Hash) -> Option<Player> {
        let table_iter = Player::iter();
        if let Some(table_iter) = table_iter {
            for row in table_iter {
                let column_data = row.elements[1usize].clone();
                if let spacetimedb_bindings::TypeValue::Bytes(entry_data) = column_data.clone() {
                    let entry_data =
                        spacetimedb_bindings::hash::Hash::from_slice(&entry_data[0..32]);
                    if owner_id.eq(&entry_data) {
                        let tuple = __tuple_to_struct__Player(row);
                        if let None = tuple {
                            ::spacetimedb_bindings::_console_log_info(&{
                                let res = :: alloc :: fmt :: format (:: core :: fmt :: Arguments :: new_v1 (& ["Internal stdb error: Can\'t convert from tuple to struct (wrong version?) Player"] , & [])) ;
                                res
                            });
                            return None;
                        }
                        return Some(tuple.unwrap());
                    }
                }
            }
        } else {
            ::spacetimedb_bindings::_console_log_info(&{
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &["Failed to get iterator for table: Player"],
                    &[],
                ));
                res
            });
        }
        return None;
    }
    #[allow(unused_variables)]
    #[allow(non_snake_case)]
    pub fn update_entity_id_eq(entity_id: u32, new_value: Player) -> bool {
        Player::delete_entity_id_eq(entity_id);
        Player::insert(new_value);
        true
    }
    #[allow(unused_variables)]
    #[allow(non_snake_case)]
    pub fn update_owner_id_eq(owner_id: Hash, new_value: Player) -> bool {
        Player::delete_owner_id_eq(owner_id);
        Player::insert(new_value);
        true
    }
    #[allow(unused_variables)]
    #[allow(non_snake_case)]
    pub fn delete_entity_id_eq(entity_id: u32) -> bool {
        let data = entity_id;
        let data = spacetimedb_bindings::TypeValue::U32(data);
        let equatable = spacetimedb_bindings::EqTypeValue::try_from(data);
        match equatable {
            Ok(value) => {
                let result = spacetimedb_bindings::delete_eq(1u32, 0u32, value);
                match result {
                    None => false,
                    Some(count) => count > 0,
                }
            }
            Err(e) => {
                ::spacetimedb_bindings::_console_log_info(&{
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &["This type is not equatable: ", " Error:"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(
                                &"spacetimedb_bindings :: TypeValue :: U32",
                            ),
                            ::core::fmt::ArgumentV1::new_display(&e),
                        ],
                    ));
                    res
                });
                false
            }
        }
    }
    #[allow(unused_variables)]
    #[allow(non_snake_case)]
    pub fn delete_owner_id_eq(owner_id: Hash) -> bool {
        let data = owner_id;
        let data = spacetimedb_bindings::TypeValue::Bytes(data.to_vec());
        let equatable = spacetimedb_bindings::EqTypeValue::try_from(data);
        match equatable {
            Ok(value) => {
                let result = spacetimedb_bindings::delete_eq(1u32, 1u32, value);
                match result {
                    None => false,
                    Some(count) => count > 0,
                }
            }
            Err(e) => {
                ::spacetimedb_bindings::_console_log_info(&{
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &["This type is not equatable: ", " Error:"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(
                                &"spacetimedb_bindings :: TypeValue :: Bytes",
                            ),
                            ::core::fmt::ArgumentV1::new_display(&e),
                        ],
                    ));
                    res
                });
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
                let column_data = row.elements[2usize].clone();
                if let spacetimedb_bindings::TypeValue::U64(entry_data) = column_data.clone() {
                    if entry_data == creation_time {
                        let tuple = __tuple_to_struct__Player(row);
                        if let None = tuple {
                            ::spacetimedb_bindings::_console_log_info(&{
                                let res = :: alloc :: fmt :: format (:: core :: fmt :: Arguments :: new_v1 (& ["Internal stdb error: Can\'t convert from tuple to struct (wrong version?) Player"] , & [])) ;
                                res
                            });
                            continue;
                        }
                        result.push(tuple.unwrap());
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
                        element_type: spacetimedb_bindings::TypeDef::U32,
                    },
                    spacetimedb_bindings::ElementDef {
                        tag: 1u8,
                        element_type: spacetimedb_bindings::TypeDef::Bytes,
                    },
                    spacetimedb_bindings::ElementDef {
                        tag: 2u8,
                        element_type: spacetimedb_bindings::TypeDef::U64,
                    },
                ]),
            ),
        },
    };
}
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn __create_table__Player(arg_ptr: usize, arg_size: usize) {
    let def = __get_struct_schema__Player();
    if let spacetimedb_bindings::TypeDef::Tuple(tuple_def) = def {
        spacetimedb_bindings::create_table(1u32, tuple_def);
    } else {
        ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
            &["This type is not a tuple: {#original_struct_ident}"],
            &[],
        ));
    }
}
#[allow(non_snake_case)]
fn __tuple_to_struct__Player(value: spacetimedb_bindings::TupleValue) -> Option<Player> {
    let elements_arr = value.elements;
    return match (
        elements_arr[0usize].clone(),
        elements_arr[1usize].clone(),
        elements_arr[2usize].clone(),
    ) {
        (
            spacetimedb_bindings::TypeValue::U32(field_0),
            spacetimedb_bindings::TypeValue::Bytes(field_1),
            spacetimedb_bindings::TypeValue::U64(field_2),
        ) => {
            let field_1: spacetimedb_bindings::hash::Hash =
                spacetimedb_bindings::hash::Hash::from_slice(field_1.as_slice());
            Some(Player {
                entity_id: field_0,
                owner_id: field_1,
                creation_time: field_2,
            })
        }
        _ => None,
    };
}
#[allow(non_snake_case)]
fn __struct_to_tuple__Player(value: Player) -> spacetimedb_bindings::TypeValue {
    return spacetimedb_bindings::TypeValue::Tuple(spacetimedb_bindings::TupleValue {
        elements: <[_]>::into_vec(
            #[rustc_box]
            ::alloc::boxed::Box::new([
                spacetimedb_bindings::TypeValue::U32(value.entity_id),
                spacetimedb_bindings::TypeValue::Bytes(value.owner_id.to_vec()),
                spacetimedb_bindings::TypeValue::U64(value.creation_time),
            ]),
        ),
    });
}
pub struct EntityTransform {
    #[unique]
    pub entity_id: u32,
    pub pos_x: f32,
    pub pos_y: f32,
    pub pos_z: f32,
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
    impl _serde::Serialize for EntityTransform {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = match _serde::Serializer::serialize_struct(
                __serializer,
                "EntityTransform",
                false as usize + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "entity_id",
                &self.entity_id,
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
    impl<'de> _serde::Deserialize<'de> for EntityTransform {
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
                __field6,
                __field7,
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
                        6u64 => _serde::__private::Ok(__Field::__field6),
                        7u64 => _serde::__private::Ok(__Field::__field7),
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
                        "entity_id" => _serde::__private::Ok(__Field::__field0),
                        "pos_x" => _serde::__private::Ok(__Field::__field1),
                        "pos_y" => _serde::__private::Ok(__Field::__field2),
                        "pos_z" => _serde::__private::Ok(__Field::__field3),
                        "rot_x" => _serde::__private::Ok(__Field::__field4),
                        "rot_y" => _serde::__private::Ok(__Field::__field5),
                        "rot_z" => _serde::__private::Ok(__Field::__field6),
                        "rot_w" => _serde::__private::Ok(__Field::__field7),
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
                        b"entity_id" => _serde::__private::Ok(__Field::__field0),
                        b"pos_x" => _serde::__private::Ok(__Field::__field1),
                        b"pos_y" => _serde::__private::Ok(__Field::__field2),
                        b"pos_z" => _serde::__private::Ok(__Field::__field3),
                        b"rot_x" => _serde::__private::Ok(__Field::__field4),
                        b"rot_y" => _serde::__private::Ok(__Field::__field5),
                        b"rot_z" => _serde::__private::Ok(__Field::__field6),
                        b"rot_w" => _serde::__private::Ok(__Field::__field7),
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
                marker: _serde::__private::PhantomData<EntityTransform>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = EntityTransform;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct EntityTransform")
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
                                    &"struct EntityTransform with 8 elements",
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
                                    &"struct EntityTransform with 8 elements",
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
                                    &"struct EntityTransform with 8 elements",
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
                                    &"struct EntityTransform with 8 elements",
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
                                    &"struct EntityTransform with 8 elements",
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
                                    &"struct EntityTransform with 8 elements",
                                ));
                            }
                        };
                    let __field6 =
                        match match _serde::de::SeqAccess::next_element::<f32>(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    6usize,
                                    &"struct EntityTransform with 8 elements",
                                ));
                            }
                        };
                    let __field7 =
                        match match _serde::de::SeqAccess::next_element::<f32>(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    7usize,
                                    &"struct EntityTransform with 8 elements",
                                ));
                            }
                        };
                    _serde::__private::Ok(EntityTransform {
                        entity_id: __field0,
                        pos_x: __field1,
                        pos_y: __field2,
                        pos_z: __field3,
                        rot_x: __field4,
                        rot_y: __field5,
                        rot_z: __field6,
                        rot_w: __field7,
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
                    let mut __field1: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut __field2: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut __field3: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut __field4: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut __field5: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut __field6: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut __field7: _serde::__private::Option<f32> = _serde::__private::None;
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
                                            "entity_id",
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
                                        <__A::Error as _serde::de::Error>::duplicate_field("pos_x"),
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
                                        <__A::Error as _serde::de::Error>::duplicate_field("pos_y"),
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
                                        <__A::Error as _serde::de::Error>::duplicate_field("pos_z"),
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
                                        <__A::Error as _serde::de::Error>::duplicate_field("rot_x"),
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
                                        <__A::Error as _serde::de::Error>::duplicate_field("rot_y"),
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
                            __Field::__field6 => {
                                if _serde::__private::Option::is_some(&__field6) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("rot_z"),
                                    );
                                }
                                __field6 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<f32>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field7 => {
                                if _serde::__private::Option::is_some(&__field7) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("rot_w"),
                                    );
                                }
                                __field7 = _serde::__private::Some(
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
                            match _serde::__private::de::missing_field("entity_id") {
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
                            match _serde::__private::de::missing_field("pos_x") {
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
                            match _serde::__private::de::missing_field("pos_y") {
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
                            match _serde::__private::de::missing_field("pos_z") {
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
                            match _serde::__private::de::missing_field("rot_x") {
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
                            match _serde::__private::de::missing_field("rot_y") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field6 = match __field6 {
                        _serde::__private::Some(__field6) => __field6,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("rot_z") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field7 = match __field7 {
                        _serde::__private::Some(__field7) => __field7,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("rot_w") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    _serde::__private::Ok(EntityTransform {
                        entity_id: __field0,
                        pos_x: __field1,
                        pos_y: __field2,
                        pos_z: __field3,
                        rot_x: __field4,
                        rot_y: __field5,
                        rot_z: __field6,
                        rot_w: __field7,
                    })
                }
            }
            const FIELDS: &'static [&'static str] = &[
                "entity_id",
                "pos_x",
                "pos_y",
                "pos_z",
                "rot_x",
                "rot_y",
                "rot_z",
                "rot_w",
            ];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "EntityTransform",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<EntityTransform>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl EntityTransform {
    #[allow(unused_variables)]
    pub fn insert(ins: EntityTransform) {
        spacetimedb_bindings::insert(
            2u32,
            spacetimedb_bindings::TupleValue {
                elements: <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        spacetimedb_bindings::TypeValue::U32(ins.entity_id),
                        spacetimedb_bindings::TypeValue::F32(ins.pos_x),
                        spacetimedb_bindings::TypeValue::F32(ins.pos_y),
                        spacetimedb_bindings::TypeValue::F32(ins.pos_z),
                        spacetimedb_bindings::TypeValue::F32(ins.rot_x),
                        spacetimedb_bindings::TypeValue::F32(ins.rot_y),
                        spacetimedb_bindings::TypeValue::F32(ins.rot_z),
                        spacetimedb_bindings::TypeValue::F32(ins.rot_w),
                    ]),
                ),
            },
        );
    }
    #[allow(unused_variables)]
    pub fn delete(f: fn(EntityTransform) -> bool) -> usize {
        ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
            &["Delete using a function is not supported yet!"],
            &[],
        ));
    }
    #[allow(unused_variables)]
    pub fn update(value: EntityTransform) -> bool {
        ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
            &["Update using a value is not supported yet!"],
            &[],
        ));
    }
    #[allow(unused_variables)]
    #[allow(non_snake_case)]
    pub fn filter_entity_id_eq(entity_id: u32) -> Option<EntityTransform> {
        let table_iter = EntityTransform::iter();
        if let Some(table_iter) = table_iter {
            for row in table_iter {
                let column_data = row.elements[0usize].clone();
                if let spacetimedb_bindings::TypeValue::U32(entry_data) = column_data.clone() {
                    if entry_data == entity_id {
                        let tuple = __tuple_to_struct__EntityTransform(row);
                        if let None = tuple {
                            ::spacetimedb_bindings::_console_log_info(&{
                                let res = :: alloc :: fmt :: format (:: core :: fmt :: Arguments :: new_v1 (& ["Internal stdb error: Can\'t convert from tuple to struct (wrong version?) EntityTransform"] , & [])) ;
                                res
                            });
                            return None;
                        }
                        return Some(tuple.unwrap());
                    }
                }
            }
        } else {
            ::spacetimedb_bindings::_console_log_info(&{
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &["Failed to get iterator for table: EntityTransform"],
                    &[],
                ));
                res
            });
        }
        return None;
    }
    #[allow(unused_variables)]
    #[allow(non_snake_case)]
    pub fn update_entity_id_eq(entity_id: u32, new_value: EntityTransform) -> bool {
        EntityTransform::delete_entity_id_eq(entity_id);
        EntityTransform::insert(new_value);
        true
    }
    #[allow(unused_variables)]
    #[allow(non_snake_case)]
    pub fn delete_entity_id_eq(entity_id: u32) -> bool {
        let data = entity_id;
        let data = spacetimedb_bindings::TypeValue::U32(data);
        let equatable = spacetimedb_bindings::EqTypeValue::try_from(data);
        match equatable {
            Ok(value) => {
                let result = spacetimedb_bindings::delete_eq(2u32, 0u32, value);
                match result {
                    None => false,
                    Some(count) => count > 0,
                }
            }
            Err(e) => {
                ::spacetimedb_bindings::_console_log_info(&{
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &["This type is not equatable: ", " Error:"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(
                                &"spacetimedb_bindings :: TypeValue :: U32",
                            ),
                            ::core::fmt::ArgumentV1::new_display(&e),
                        ],
                    ));
                    res
                });
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
    pub fn filter_pos_x_eq(pos_x: f32) -> Vec<EntityTransform> {
        let mut result = Vec::<EntityTransform>::new();
        let table_iter = EntityTransform::iter();
        if let Some(table_iter) = table_iter {
            for row in table_iter {
                let column_data = row.elements[1usize].clone();
                if let spacetimedb_bindings::TypeValue::F32(entry_data) = column_data.clone() {
                    if entry_data == pos_x {
                        let tuple = __tuple_to_struct__EntityTransform(row);
                        if let None = tuple {
                            ::spacetimedb_bindings::_console_log_info(&{
                                let res = :: alloc :: fmt :: format (:: core :: fmt :: Arguments :: new_v1 (& ["Internal stdb error: Can\'t convert from tuple to struct (wrong version?) EntityTransform"] , & [])) ;
                                res
                            });
                            continue;
                        }
                        result.push(tuple.unwrap());
                    }
                }
            }
        }
        return result;
    }
    #[allow(non_snake_case)]
    #[allow(unused_variables)]
    pub fn filter_pos_y_eq(pos_y: f32) -> Vec<EntityTransform> {
        let mut result = Vec::<EntityTransform>::new();
        let table_iter = EntityTransform::iter();
        if let Some(table_iter) = table_iter {
            for row in table_iter {
                let column_data = row.elements[2usize].clone();
                if let spacetimedb_bindings::TypeValue::F32(entry_data) = column_data.clone() {
                    if entry_data == pos_y {
                        let tuple = __tuple_to_struct__EntityTransform(row);
                        if let None = tuple {
                            ::spacetimedb_bindings::_console_log_info(&{
                                let res = :: alloc :: fmt :: format (:: core :: fmt :: Arguments :: new_v1 (& ["Internal stdb error: Can\'t convert from tuple to struct (wrong version?) EntityTransform"] , & [])) ;
                                res
                            });
                            continue;
                        }
                        result.push(tuple.unwrap());
                    }
                }
            }
        }
        return result;
    }
    #[allow(non_snake_case)]
    #[allow(unused_variables)]
    pub fn filter_pos_z_eq(pos_z: f32) -> Vec<EntityTransform> {
        let mut result = Vec::<EntityTransform>::new();
        let table_iter = EntityTransform::iter();
        if let Some(table_iter) = table_iter {
            for row in table_iter {
                let column_data = row.elements[3usize].clone();
                if let spacetimedb_bindings::TypeValue::F32(entry_data) = column_data.clone() {
                    if entry_data == pos_z {
                        let tuple = __tuple_to_struct__EntityTransform(row);
                        if let None = tuple {
                            ::spacetimedb_bindings::_console_log_info(&{
                                let res = :: alloc :: fmt :: format (:: core :: fmt :: Arguments :: new_v1 (& ["Internal stdb error: Can\'t convert from tuple to struct (wrong version?) EntityTransform"] , & [])) ;
                                res
                            });
                            continue;
                        }
                        result.push(tuple.unwrap());
                    }
                }
            }
        }
        return result;
    }
    #[allow(non_snake_case)]
    #[allow(unused_variables)]
    pub fn filter_rot_x_eq(rot_x: f32) -> Vec<EntityTransform> {
        let mut result = Vec::<EntityTransform>::new();
        let table_iter = EntityTransform::iter();
        if let Some(table_iter) = table_iter {
            for row in table_iter {
                let column_data = row.elements[4usize].clone();
                if let spacetimedb_bindings::TypeValue::F32(entry_data) = column_data.clone() {
                    if entry_data == rot_x {
                        let tuple = __tuple_to_struct__EntityTransform(row);
                        if let None = tuple {
                            ::spacetimedb_bindings::_console_log_info(&{
                                let res = :: alloc :: fmt :: format (:: core :: fmt :: Arguments :: new_v1 (& ["Internal stdb error: Can\'t convert from tuple to struct (wrong version?) EntityTransform"] , & [])) ;
                                res
                            });
                            continue;
                        }
                        result.push(tuple.unwrap());
                    }
                }
            }
        }
        return result;
    }
    #[allow(non_snake_case)]
    #[allow(unused_variables)]
    pub fn filter_rot_y_eq(rot_y: f32) -> Vec<EntityTransform> {
        let mut result = Vec::<EntityTransform>::new();
        let table_iter = EntityTransform::iter();
        if let Some(table_iter) = table_iter {
            for row in table_iter {
                let column_data = row.elements[5usize].clone();
                if let spacetimedb_bindings::TypeValue::F32(entry_data) = column_data.clone() {
                    if entry_data == rot_y {
                        let tuple = __tuple_to_struct__EntityTransform(row);
                        if let None = tuple {
                            ::spacetimedb_bindings::_console_log_info(&{
                                let res = :: alloc :: fmt :: format (:: core :: fmt :: Arguments :: new_v1 (& ["Internal stdb error: Can\'t convert from tuple to struct (wrong version?) EntityTransform"] , & [])) ;
                                res
                            });
                            continue;
                        }
                        result.push(tuple.unwrap());
                    }
                }
            }
        }
        return result;
    }
    #[allow(non_snake_case)]
    #[allow(unused_variables)]
    pub fn filter_rot_z_eq(rot_z: f32) -> Vec<EntityTransform> {
        let mut result = Vec::<EntityTransform>::new();
        let table_iter = EntityTransform::iter();
        if let Some(table_iter) = table_iter {
            for row in table_iter {
                let column_data = row.elements[6usize].clone();
                if let spacetimedb_bindings::TypeValue::F32(entry_data) = column_data.clone() {
                    if entry_data == rot_z {
                        let tuple = __tuple_to_struct__EntityTransform(row);
                        if let None = tuple {
                            ::spacetimedb_bindings::_console_log_info(&{
                                let res = :: alloc :: fmt :: format (:: core :: fmt :: Arguments :: new_v1 (& ["Internal stdb error: Can\'t convert from tuple to struct (wrong version?) EntityTransform"] , & [])) ;
                                res
                            });
                            continue;
                        }
                        result.push(tuple.unwrap());
                    }
                }
            }
        }
        return result;
    }
    #[allow(non_snake_case)]
    #[allow(unused_variables)]
    pub fn filter_rot_w_eq(rot_w: f32) -> Vec<EntityTransform> {
        let mut result = Vec::<EntityTransform>::new();
        let table_iter = EntityTransform::iter();
        if let Some(table_iter) = table_iter {
            for row in table_iter {
                let column_data = row.elements[7usize].clone();
                if let spacetimedb_bindings::TypeValue::F32(entry_data) = column_data.clone() {
                    if entry_data == rot_w {
                        let tuple = __tuple_to_struct__EntityTransform(row);
                        if let None = tuple {
                            ::spacetimedb_bindings::_console_log_info(&{
                                let res = :: alloc :: fmt :: format (:: core :: fmt :: Arguments :: new_v1 (& ["Internal stdb error: Can\'t convert from tuple to struct (wrong version?) EntityTransform"] , & [])) ;
                                res
                            });
                            continue;
                        }
                        result.push(tuple.unwrap());
                    }
                }
            }
        }
        return result;
    }
}
#[allow(non_snake_case)]
fn __get_struct_schema__EntityTransform() -> spacetimedb_bindings::TypeDef {
    return spacetimedb_bindings::TypeDef::Tuple {
        0: spacetimedb_bindings::TupleDef {
            elements: <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    spacetimedb_bindings::ElementDef {
                        tag: 0u8,
                        element_type: spacetimedb_bindings::TypeDef::U32,
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
                    spacetimedb_bindings::ElementDef {
                        tag: 4u8,
                        element_type: spacetimedb_bindings::TypeDef::F32,
                    },
                    spacetimedb_bindings::ElementDef {
                        tag: 5u8,
                        element_type: spacetimedb_bindings::TypeDef::F32,
                    },
                    spacetimedb_bindings::ElementDef {
                        tag: 6u8,
                        element_type: spacetimedb_bindings::TypeDef::F32,
                    },
                    spacetimedb_bindings::ElementDef {
                        tag: 7u8,
                        element_type: spacetimedb_bindings::TypeDef::F32,
                    },
                ]),
            ),
        },
    };
}
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn __create_table__EntityTransform(arg_ptr: usize, arg_size: usize) {
    let def = __get_struct_schema__EntityTransform();
    if let spacetimedb_bindings::TypeDef::Tuple(tuple_def) = def {
        spacetimedb_bindings::create_table(2u32, tuple_def);
    } else {
        ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
            &["This type is not a tuple: {#original_struct_ident}"],
            &[],
        ));
    }
}
#[allow(non_snake_case)]
fn __tuple_to_struct__EntityTransform(
    value: spacetimedb_bindings::TupleValue,
) -> Option<EntityTransform> {
    let elements_arr = value.elements;
    return match (
        elements_arr[0usize].clone(),
        elements_arr[1usize].clone(),
        elements_arr[2usize].clone(),
        elements_arr[3usize].clone(),
        elements_arr[4usize].clone(),
        elements_arr[5usize].clone(),
        elements_arr[6usize].clone(),
        elements_arr[7usize].clone(),
    ) {
        (
            spacetimedb_bindings::TypeValue::U32(field_0),
            spacetimedb_bindings::TypeValue::F32(field_1),
            spacetimedb_bindings::TypeValue::F32(field_2),
            spacetimedb_bindings::TypeValue::F32(field_3),
            spacetimedb_bindings::TypeValue::F32(field_4),
            spacetimedb_bindings::TypeValue::F32(field_5),
            spacetimedb_bindings::TypeValue::F32(field_6),
            spacetimedb_bindings::TypeValue::F32(field_7),
        ) => Some(EntityTransform {
            entity_id: field_0,
            pos_x: field_1,
            pos_y: field_2,
            pos_z: field_3,
            rot_x: field_4,
            rot_y: field_5,
            rot_z: field_6,
            rot_w: field_7,
        }),
        _ => None,
    };
}
#[allow(non_snake_case)]
fn __struct_to_tuple__EntityTransform(value: EntityTransform) -> spacetimedb_bindings::TypeValue {
    return spacetimedb_bindings::TypeValue::Tuple(spacetimedb_bindings::TupleValue {
        elements: <[_]>::into_vec(
            #[rustc_box]
            ::alloc::boxed::Box::new([
                spacetimedb_bindings::TypeValue::U32(value.entity_id),
                spacetimedb_bindings::TypeValue::F32(value.pos_x),
                spacetimedb_bindings::TypeValue::F32(value.pos_y),
                spacetimedb_bindings::TypeValue::F32(value.pos_z),
                spacetimedb_bindings::TypeValue::F32(value.rot_x),
                spacetimedb_bindings::TypeValue::F32(value.rot_y),
                spacetimedb_bindings::TypeValue::F32(value.rot_z),
                spacetimedb_bindings::TypeValue::F32(value.rot_w),
            ]),
        ),
    });
}
pub struct PlayerAnimation {
    #[unique]
    pub entity_id: u32,
    pub moving: bool,
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for PlayerAnimation {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = match _serde::Serializer::serialize_struct(
                __serializer,
                "PlayerAnimation",
                false as usize + 1 + 1,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "entity_id",
                &self.entity_id,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "moving",
                &self.moving,
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
    impl<'de> _serde::Deserialize<'de> for PlayerAnimation {
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
                        "entity_id" => _serde::__private::Ok(__Field::__field0),
                        "moving" => _serde::__private::Ok(__Field::__field1),
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
                        b"entity_id" => _serde::__private::Ok(__Field::__field0),
                        b"moving" => _serde::__private::Ok(__Field::__field1),
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
                marker: _serde::__private::PhantomData<PlayerAnimation>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = PlayerAnimation;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct PlayerAnimation")
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
                                    &"struct PlayerAnimation with 2 elements",
                                ));
                            }
                        };
                    let __field1 =
                        match match _serde::de::SeqAccess::next_element::<bool>(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct PlayerAnimation with 2 elements",
                                ));
                            }
                        };
                    _serde::__private::Ok(PlayerAnimation {
                        entity_id: __field0,
                        moving: __field1,
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
                    let mut __field1: _serde::__private::Option<bool> = _serde::__private::None;
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
                                            "entity_id",
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
                                            "moving",
                                        ),
                                    );
                                }
                                __field1 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<bool>(&mut __map) {
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
                            match _serde::__private::de::missing_field("entity_id") {
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
                            match _serde::__private::de::missing_field("moving") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    _serde::__private::Ok(PlayerAnimation {
                        entity_id: __field0,
                        moving: __field1,
                    })
                }
            }
            const FIELDS: &'static [&'static str] = &["entity_id", "moving"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "PlayerAnimation",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<PlayerAnimation>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl PlayerAnimation {
    #[allow(unused_variables)]
    pub fn insert(ins: PlayerAnimation) {
        spacetimedb_bindings::insert(
            3u32,
            spacetimedb_bindings::TupleValue {
                elements: <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        spacetimedb_bindings::TypeValue::U32(ins.entity_id),
                        spacetimedb_bindings::TypeValue::Bool(ins.moving),
                    ]),
                ),
            },
        );
    }
    #[allow(unused_variables)]
    pub fn delete(f: fn(PlayerAnimation) -> bool) -> usize {
        ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
            &["Delete using a function is not supported yet!"],
            &[],
        ));
    }
    #[allow(unused_variables)]
    pub fn update(value: PlayerAnimation) -> bool {
        ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
            &["Update using a value is not supported yet!"],
            &[],
        ));
    }
    #[allow(unused_variables)]
    #[allow(non_snake_case)]
    pub fn filter_entity_id_eq(entity_id: u32) -> Option<PlayerAnimation> {
        let table_iter = PlayerAnimation::iter();
        if let Some(table_iter) = table_iter {
            for row in table_iter {
                let column_data = row.elements[0usize].clone();
                if let spacetimedb_bindings::TypeValue::U32(entry_data) = column_data.clone() {
                    if entry_data == entity_id {
                        let tuple = __tuple_to_struct__PlayerAnimation(row);
                        if let None = tuple {
                            ::spacetimedb_bindings::_console_log_info(&{
                                let res = :: alloc :: fmt :: format (:: core :: fmt :: Arguments :: new_v1 (& ["Internal stdb error: Can\'t convert from tuple to struct (wrong version?) PlayerAnimation"] , & [])) ;
                                res
                            });
                            return None;
                        }
                        return Some(tuple.unwrap());
                    }
                }
            }
        } else {
            ::spacetimedb_bindings::_console_log_info(&{
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &["Failed to get iterator for table: PlayerAnimation"],
                    &[],
                ));
                res
            });
        }
        return None;
    }
    #[allow(unused_variables)]
    #[allow(non_snake_case)]
    pub fn update_entity_id_eq(entity_id: u32, new_value: PlayerAnimation) -> bool {
        PlayerAnimation::delete_entity_id_eq(entity_id);
        PlayerAnimation::insert(new_value);
        true
    }
    #[allow(unused_variables)]
    #[allow(non_snake_case)]
    pub fn delete_entity_id_eq(entity_id: u32) -> bool {
        let data = entity_id;
        let data = spacetimedb_bindings::TypeValue::U32(data);
        let equatable = spacetimedb_bindings::EqTypeValue::try_from(data);
        match equatable {
            Ok(value) => {
                let result = spacetimedb_bindings::delete_eq(3u32, 0u32, value);
                match result {
                    None => false,
                    Some(count) => count > 0,
                }
            }
            Err(e) => {
                ::spacetimedb_bindings::_console_log_info(&{
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &["This type is not equatable: ", " Error:"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(
                                &"spacetimedb_bindings :: TypeValue :: U32",
                            ),
                            ::core::fmt::ArgumentV1::new_display(&e),
                        ],
                    ));
                    res
                });
                false
            }
        }
    }
    #[allow(unused_variables)]
    pub fn iter() -> Option<spacetimedb_bindings::TableIter> {
        spacetimedb_bindings::__iter__(3u32)
    }
    #[allow(non_snake_case)]
    #[allow(unused_variables)]
    pub fn filter_moving_eq(moving: bool) -> Vec<PlayerAnimation> {
        let mut result = Vec::<PlayerAnimation>::new();
        let table_iter = PlayerAnimation::iter();
        if let Some(table_iter) = table_iter {
            for row in table_iter {
                let column_data = row.elements[1usize].clone();
                if let spacetimedb_bindings::TypeValue::Bool(entry_data) = column_data.clone() {
                    if entry_data == moving {
                        let tuple = __tuple_to_struct__PlayerAnimation(row);
                        if let None = tuple {
                            ::spacetimedb_bindings::_console_log_info(&{
                                let res = :: alloc :: fmt :: format (:: core :: fmt :: Arguments :: new_v1 (& ["Internal stdb error: Can\'t convert from tuple to struct (wrong version?) PlayerAnimation"] , & [])) ;
                                res
                            });
                            continue;
                        }
                        result.push(tuple.unwrap());
                    }
                }
            }
        }
        return result;
    }
}
#[allow(non_snake_case)]
fn __get_struct_schema__PlayerAnimation() -> spacetimedb_bindings::TypeDef {
    return spacetimedb_bindings::TypeDef::Tuple {
        0: spacetimedb_bindings::TupleDef {
            elements: <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    spacetimedb_bindings::ElementDef {
                        tag: 0u8,
                        element_type: spacetimedb_bindings::TypeDef::U32,
                    },
                    spacetimedb_bindings::ElementDef {
                        tag: 1u8,
                        element_type: spacetimedb_bindings::TypeDef::Bool,
                    },
                ]),
            ),
        },
    };
}
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn __create_table__PlayerAnimation(arg_ptr: usize, arg_size: usize) {
    let def = __get_struct_schema__PlayerAnimation();
    if let spacetimedb_bindings::TypeDef::Tuple(tuple_def) = def {
        spacetimedb_bindings::create_table(3u32, tuple_def);
    } else {
        ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
            &["This type is not a tuple: {#original_struct_ident}"],
            &[],
        ));
    }
}
#[allow(non_snake_case)]
fn __tuple_to_struct__PlayerAnimation(
    value: spacetimedb_bindings::TupleValue,
) -> Option<PlayerAnimation> {
    let elements_arr = value.elements;
    return match (elements_arr[0usize].clone(), elements_arr[1usize].clone()) {
        (
            spacetimedb_bindings::TypeValue::U32(field_0),
            spacetimedb_bindings::TypeValue::Bool(field_1),
        ) => Some(PlayerAnimation {
            entity_id: field_0,
            moving: field_1,
        }),
        _ => None,
    };
}
#[allow(non_snake_case)]
fn __struct_to_tuple__PlayerAnimation(value: PlayerAnimation) -> spacetimedb_bindings::TypeValue {
    return spacetimedb_bindings::TypeValue::Tuple(spacetimedb_bindings::TupleValue {
        elements: <[_]>::into_vec(
            #[rustc_box]
            ::alloc::boxed::Box::new([
                spacetimedb_bindings::TypeValue::U32(value.entity_id),
                spacetimedb_bindings::TypeValue::Bool(value.moving),
            ]),
        ),
    });
}
pub struct EntityInventory {
    #[unique]
    pub entity_id: u32,
    pub pockets: Vec<Pocket>,
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for EntityInventory {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = match _serde::Serializer::serialize_struct(
                __serializer,
                "EntityInventory",
                false as usize + 1 + 1,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "entity_id",
                &self.entity_id,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "pockets",
                &self.pockets,
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
    impl<'de> _serde::Deserialize<'de> for EntityInventory {
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
                        "entity_id" => _serde::__private::Ok(__Field::__field0),
                        "pockets" => _serde::__private::Ok(__Field::__field1),
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
                        b"entity_id" => _serde::__private::Ok(__Field::__field0),
                        b"pockets" => _serde::__private::Ok(__Field::__field1),
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
                marker: _serde::__private::PhantomData<EntityInventory>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = EntityInventory;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct EntityInventory")
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
                                    &"struct EntityInventory with 2 elements",
                                ));
                            }
                        };
                    let __field1 = match match _serde::de::SeqAccess::next_element::<Vec<Pocket>>(
                        &mut __seq,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                1usize,
                                &"struct EntityInventory with 2 elements",
                            ));
                        }
                    };
                    _serde::__private::Ok(EntityInventory {
                        entity_id: __field0,
                        pockets: __field1,
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
                    let mut __field1: _serde::__private::Option<Vec<Pocket>> =
                        _serde::__private::None;
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
                                            "entity_id",
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
                                            "pockets",
                                        ),
                                    );
                                }
                                __field1 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<Vec<Pocket>>(
                                        &mut __map,
                                    ) {
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
                            match _serde::__private::de::missing_field("entity_id") {
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
                            match _serde::__private::de::missing_field("pockets") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    _serde::__private::Ok(EntityInventory {
                        entity_id: __field0,
                        pockets: __field1,
                    })
                }
            }
            const FIELDS: &'static [&'static str] = &["entity_id", "pockets"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "EntityInventory",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<EntityInventory>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl EntityInventory {
    #[allow(unused_variables)]
    pub fn insert(ins: EntityInventory) {
        let mut type_value_vec_pockets: Vec<spacetimedb_bindings::TypeValue> =
            Vec::<spacetimedb_bindings::TypeValue>::new();
        for value in ins.pockets {
            type_value_vec_pockets.push(__struct_to_tuple__Pocket(value));
        }
        spacetimedb_bindings::insert(
            4u32,
            spacetimedb_bindings::TupleValue {
                elements: <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        spacetimedb_bindings::TypeValue::U32(ins.entity_id),
                        spacetimedb_bindings::TypeValue::Vec(type_value_vec_pockets),
                    ]),
                ),
            },
        );
    }
    #[allow(unused_variables)]
    pub fn delete(f: fn(EntityInventory) -> bool) -> usize {
        ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
            &["Delete using a function is not supported yet!"],
            &[],
        ));
    }
    #[allow(unused_variables)]
    pub fn update(value: EntityInventory) -> bool {
        ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
            &["Update using a value is not supported yet!"],
            &[],
        ));
    }
    #[allow(unused_variables)]
    #[allow(non_snake_case)]
    pub fn filter_entity_id_eq(entity_id: u32) -> Option<EntityInventory> {
        let table_iter = EntityInventory::iter();
        if let Some(table_iter) = table_iter {
            for row in table_iter {
                let column_data = row.elements[0usize].clone();
                if let spacetimedb_bindings::TypeValue::U32(entry_data) = column_data.clone() {
                    if entry_data == entity_id {
                        let tuple = __tuple_to_struct__EntityInventory(row);
                        if let None = tuple {
                            ::spacetimedb_bindings::_console_log_info(&{
                                let res = :: alloc :: fmt :: format (:: core :: fmt :: Arguments :: new_v1 (& ["Internal stdb error: Can\'t convert from tuple to struct (wrong version?) EntityInventory"] , & [])) ;
                                res
                            });
                            return None;
                        }
                        return Some(tuple.unwrap());
                    }
                }
            }
        } else {
            ::spacetimedb_bindings::_console_log_info(&{
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &["Failed to get iterator for table: EntityInventory"],
                    &[],
                ));
                res
            });
        }
        return None;
    }
    #[allow(unused_variables)]
    #[allow(non_snake_case)]
    pub fn update_entity_id_eq(entity_id: u32, new_value: EntityInventory) -> bool {
        EntityInventory::delete_entity_id_eq(entity_id);
        EntityInventory::insert(new_value);
        true
    }
    #[allow(unused_variables)]
    #[allow(non_snake_case)]
    pub fn delete_entity_id_eq(entity_id: u32) -> bool {
        let data = entity_id;
        let data = spacetimedb_bindings::TypeValue::U32(data);
        let equatable = spacetimedb_bindings::EqTypeValue::try_from(data);
        match equatable {
            Ok(value) => {
                let result = spacetimedb_bindings::delete_eq(4u32, 0u32, value);
                match result {
                    None => false,
                    Some(count) => count > 0,
                }
            }
            Err(e) => {
                ::spacetimedb_bindings::_console_log_info(&{
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &["This type is not equatable: ", " Error:"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(
                                &"spacetimedb_bindings :: TypeValue :: U32",
                            ),
                            ::core::fmt::ArgumentV1::new_display(&e),
                        ],
                    ));
                    res
                });
                false
            }
        }
    }
    #[allow(unused_variables)]
    pub fn iter() -> Option<spacetimedb_bindings::TableIter> {
        spacetimedb_bindings::__iter__(4u32)
    }
}
#[allow(non_snake_case)]
fn __get_struct_schema__EntityInventory() -> spacetimedb_bindings::TypeDef {
    return spacetimedb_bindings::TypeDef::Tuple {
        0: spacetimedb_bindings::TupleDef {
            elements: <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    spacetimedb_bindings::ElementDef {
                        tag: 0u8,
                        element_type: spacetimedb_bindings::TypeDef::U32,
                    },
                    spacetimedb_bindings::ElementDef {
                        tag: 1u8,
                        element_type: spacetimedb_bindings::TypeDef::Vec {
                            element_type: __get_struct_schema__Pocket().into(),
                        },
                    },
                ]),
            ),
        },
    };
}
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn __create_table__EntityInventory(arg_ptr: usize, arg_size: usize) {
    let def = __get_struct_schema__EntityInventory();
    if let spacetimedb_bindings::TypeDef::Tuple(tuple_def) = def {
        spacetimedb_bindings::create_table(4u32, tuple_def);
    } else {
        ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
            &["This type is not a tuple: {#original_struct_ident}"],
            &[],
        ));
    }
}
#[allow(non_snake_case)]
fn __tuple_to_struct__EntityInventory(
    value: spacetimedb_bindings::TupleValue,
) -> Option<EntityInventory> {
    let elements_arr = value.elements;
    return match (elements_arr[0usize].clone(), elements_arr[1usize].clone()) {
        (
            spacetimedb_bindings::TypeValue::U32(field_0),
            spacetimedb_bindings::TypeValue::Vec(field_1),
        ) => {
            let mut native_vec_field_1: Vec<Pocket> = Vec::<Pocket>::new();
            for tuple_val in field_1 {
                match tuple_val {
                    spacetimedb_bindings::TypeValue::Tuple(entry) => {
                        match __tuple_to_struct__Pocket(entry) {
                            Some(native_value) => {
                                native_vec_field_1.push(native_value);
                            }
                            None => {
                                ::spacetimedb_bindings::_console_log_info(&{
                                    let res = :: alloc :: fmt :: format (:: core :: fmt :: Arguments :: new_v1 (& ["Failed to convert TypeValue::Tuple to native struct type: "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& "Pocket")])) ;
                                    res
                                });
                            }
                        }
                    }
                    _ => {
                        ::spacetimedb_bindings::_console_log_info(&{
                            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                &["Vec contains wrong type, expected TypeValue::Tuple"],
                                &[],
                            ));
                            res
                        });
                    }
                }
            }
            let field_1 = native_vec_field_1;
            Some(EntityInventory {
                entity_id: field_0,
                pockets: field_1,
            })
        }
        _ => None,
    };
}
#[allow(non_snake_case)]
fn __struct_to_tuple__EntityInventory(value: EntityInventory) -> spacetimedb_bindings::TypeValue {
    let mut tuple_vec_pockets: Vec<spacetimedb_bindings::TypeValue> =
        Vec::<spacetimedb_bindings::TypeValue>::new();
    for entry in value.pockets {
        tuple_vec_pockets.push(__struct_to_tuple__Pocket(entry));
    }
    return spacetimedb_bindings::TypeValue::Tuple(spacetimedb_bindings::TupleValue {
        elements: <[_]>::into_vec(
            #[rustc_box]
            ::alloc::boxed::Box::new([
                spacetimedb_bindings::TypeValue::U32(value.entity_id),
                spacetimedb_bindings::TypeValue::Vec(tuple_vec_pockets),
            ]),
        ),
    });
}
pub struct Pocket {
    pub item_id: u32,
    pub pocket_idx: u32,
    pub item_count: i32,
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
                false as usize + 1 + 1 + 1,
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
                "pocket_idx",
                &self.pocket_idx,
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
                        "item_id" => _serde::__private::Ok(__Field::__field0),
                        "pocket_idx" => _serde::__private::Ok(__Field::__field1),
                        "item_count" => _serde::__private::Ok(__Field::__field2),
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
                        b"pocket_idx" => _serde::__private::Ok(__Field::__field1),
                        b"item_count" => _serde::__private::Ok(__Field::__field2),
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
                                    &"struct Pocket with 3 elements",
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
                                    &"struct Pocket with 3 elements",
                                ));
                            }
                        };
                    let __field2 =
                        match match _serde::de::SeqAccess::next_element::<i32>(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    2usize,
                                    &"struct Pocket with 3 elements",
                                ));
                            }
                        };
                    _serde::__private::Ok(Pocket {
                        item_id: __field0,
                        pocket_idx: __field1,
                        item_count: __field2,
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
                    let mut __field2: _serde::__private::Option<i32> = _serde::__private::None;
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
                                            "pocket_idx",
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
                            __Field::__field2 => {
                                if _serde::__private::Option::is_some(&__field2) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "item_count",
                                        ),
                                    );
                                }
                                __field2 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<i32>(&mut __map) {
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
                            match _serde::__private::de::missing_field("pocket_idx") {
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
                        pocket_idx: __field1,
                        item_count: __field2,
                    })
                }
            }
            const FIELDS: &'static [&'static str] = &["item_id", "pocket_idx", "item_count"];
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
                        element_type: spacetimedb_bindings::TypeDef::U32,
                    },
                    spacetimedb_bindings::ElementDef {
                        tag: 1u8,
                        element_type: spacetimedb_bindings::TypeDef::U32,
                    },
                    spacetimedb_bindings::ElementDef {
                        tag: 2u8,
                        element_type: spacetimedb_bindings::TypeDef::I32,
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
    return match (
        elements_arr[0usize].clone(),
        elements_arr[1usize].clone(),
        elements_arr[2usize].clone(),
    ) {
        (
            spacetimedb_bindings::TypeValue::U32(field_0),
            spacetimedb_bindings::TypeValue::U32(field_1),
            spacetimedb_bindings::TypeValue::I32(field_2),
        ) => Some(Pocket {
            item_id: field_0,
            pocket_idx: field_1,
            item_count: field_2,
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
                spacetimedb_bindings::TypeValue::U32(value.pocket_idx),
                spacetimedb_bindings::TypeValue::I32(value.item_count),
            ]),
        ),
    });
}
#[automatically_derived]
impl ::core::marker::Copy for Pocket {}
#[automatically_derived]
impl ::core::clone::Clone for Pocket {
    #[inline]
    fn clone(&self) -> Pocket {
        let _: ::core::clone::AssertParamIsClone<u32>;
        let _: ::core::clone::AssertParamIsClone<i32>;
        *self
    }
}
pub struct PlayerLogin {
    #[unique]
    pub entity_id: u32,
    pub logged_in: bool,
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for PlayerLogin {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = match _serde::Serializer::serialize_struct(
                __serializer,
                "PlayerLogin",
                false as usize + 1 + 1,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "entity_id",
                &self.entity_id,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "logged_in",
                &self.logged_in,
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
    impl<'de> _serde::Deserialize<'de> for PlayerLogin {
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
                        "entity_id" => _serde::__private::Ok(__Field::__field0),
                        "logged_in" => _serde::__private::Ok(__Field::__field1),
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
                        b"entity_id" => _serde::__private::Ok(__Field::__field0),
                        b"logged_in" => _serde::__private::Ok(__Field::__field1),
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
                marker: _serde::__private::PhantomData<PlayerLogin>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = PlayerLogin;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct PlayerLogin")
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
                                    &"struct PlayerLogin with 2 elements",
                                ));
                            }
                        };
                    let __field1 =
                        match match _serde::de::SeqAccess::next_element::<bool>(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct PlayerLogin with 2 elements",
                                ));
                            }
                        };
                    _serde::__private::Ok(PlayerLogin {
                        entity_id: __field0,
                        logged_in: __field1,
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
                    let mut __field1: _serde::__private::Option<bool> = _serde::__private::None;
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
                                            "entity_id",
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
                                            "logged_in",
                                        ),
                                    );
                                }
                                __field1 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<bool>(&mut __map) {
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
                            match _serde::__private::de::missing_field("entity_id") {
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
                            match _serde::__private::de::missing_field("logged_in") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    _serde::__private::Ok(PlayerLogin {
                        entity_id: __field0,
                        logged_in: __field1,
                    })
                }
            }
            const FIELDS: &'static [&'static str] = &["entity_id", "logged_in"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "PlayerLogin",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<PlayerLogin>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl PlayerLogin {
    #[allow(unused_variables)]
    pub fn insert(ins: PlayerLogin) {
        spacetimedb_bindings::insert(
            5u32,
            spacetimedb_bindings::TupleValue {
                elements: <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        spacetimedb_bindings::TypeValue::U32(ins.entity_id),
                        spacetimedb_bindings::TypeValue::Bool(ins.logged_in),
                    ]),
                ),
            },
        );
    }
    #[allow(unused_variables)]
    pub fn delete(f: fn(PlayerLogin) -> bool) -> usize {
        ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
            &["Delete using a function is not supported yet!"],
            &[],
        ));
    }
    #[allow(unused_variables)]
    pub fn update(value: PlayerLogin) -> bool {
        ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
            &["Update using a value is not supported yet!"],
            &[],
        ));
    }
    #[allow(unused_variables)]
    #[allow(non_snake_case)]
    pub fn filter_entity_id_eq(entity_id: u32) -> Option<PlayerLogin> {
        let table_iter = PlayerLogin::iter();
        if let Some(table_iter) = table_iter {
            for row in table_iter {
                let column_data = row.elements[0usize].clone();
                if let spacetimedb_bindings::TypeValue::U32(entry_data) = column_data.clone() {
                    if entry_data == entity_id {
                        let tuple = __tuple_to_struct__PlayerLogin(row);
                        if let None = tuple {
                            ::spacetimedb_bindings::_console_log_info(&{
                                let res = :: alloc :: fmt :: format (:: core :: fmt :: Arguments :: new_v1 (& ["Internal stdb error: Can\'t convert from tuple to struct (wrong version?) PlayerLogin"] , & [])) ;
                                res
                            });
                            return None;
                        }
                        return Some(tuple.unwrap());
                    }
                }
            }
        } else {
            ::spacetimedb_bindings::_console_log_info(&{
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &["Failed to get iterator for table: PlayerLogin"],
                    &[],
                ));
                res
            });
        }
        return None;
    }
    #[allow(unused_variables)]
    #[allow(non_snake_case)]
    pub fn update_entity_id_eq(entity_id: u32, new_value: PlayerLogin) -> bool {
        PlayerLogin::delete_entity_id_eq(entity_id);
        PlayerLogin::insert(new_value);
        true
    }
    #[allow(unused_variables)]
    #[allow(non_snake_case)]
    pub fn delete_entity_id_eq(entity_id: u32) -> bool {
        let data = entity_id;
        let data = spacetimedb_bindings::TypeValue::U32(data);
        let equatable = spacetimedb_bindings::EqTypeValue::try_from(data);
        match equatable {
            Ok(value) => {
                let result = spacetimedb_bindings::delete_eq(5u32, 0u32, value);
                match result {
                    None => false,
                    Some(count) => count > 0,
                }
            }
            Err(e) => {
                ::spacetimedb_bindings::_console_log_info(&{
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &["This type is not equatable: ", " Error:"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(
                                &"spacetimedb_bindings :: TypeValue :: U32",
                            ),
                            ::core::fmt::ArgumentV1::new_display(&e),
                        ],
                    ));
                    res
                });
                false
            }
        }
    }
    #[allow(unused_variables)]
    pub fn iter() -> Option<spacetimedb_bindings::TableIter> {
        spacetimedb_bindings::__iter__(5u32)
    }
    #[allow(non_snake_case)]
    #[allow(unused_variables)]
    pub fn filter_logged_in_eq(logged_in: bool) -> Vec<PlayerLogin> {
        let mut result = Vec::<PlayerLogin>::new();
        let table_iter = PlayerLogin::iter();
        if let Some(table_iter) = table_iter {
            for row in table_iter {
                let column_data = row.elements[1usize].clone();
                if let spacetimedb_bindings::TypeValue::Bool(entry_data) = column_data.clone() {
                    if entry_data == logged_in {
                        let tuple = __tuple_to_struct__PlayerLogin(row);
                        if let None = tuple {
                            ::spacetimedb_bindings::_console_log_info(&{
                                let res = :: alloc :: fmt :: format (:: core :: fmt :: Arguments :: new_v1 (& ["Internal stdb error: Can\'t convert from tuple to struct (wrong version?) PlayerLogin"] , & [])) ;
                                res
                            });
                            continue;
                        }
                        result.push(tuple.unwrap());
                    }
                }
            }
        }
        return result;
    }
}
#[allow(non_snake_case)]
fn __get_struct_schema__PlayerLogin() -> spacetimedb_bindings::TypeDef {
    return spacetimedb_bindings::TypeDef::Tuple {
        0: spacetimedb_bindings::TupleDef {
            elements: <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    spacetimedb_bindings::ElementDef {
                        tag: 0u8,
                        element_type: spacetimedb_bindings::TypeDef::U32,
                    },
                    spacetimedb_bindings::ElementDef {
                        tag: 1u8,
                        element_type: spacetimedb_bindings::TypeDef::Bool,
                    },
                ]),
            ),
        },
    };
}
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn __create_table__PlayerLogin(arg_ptr: usize, arg_size: usize) {
    let def = __get_struct_schema__PlayerLogin();
    if let spacetimedb_bindings::TypeDef::Tuple(tuple_def) = def {
        spacetimedb_bindings::create_table(5u32, tuple_def);
    } else {
        ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
            &["This type is not a tuple: {#original_struct_ident}"],
            &[],
        ));
    }
}
#[allow(non_snake_case)]
fn __tuple_to_struct__PlayerLogin(value: spacetimedb_bindings::TupleValue) -> Option<PlayerLogin> {
    let elements_arr = value.elements;
    return match (elements_arr[0usize].clone(), elements_arr[1usize].clone()) {
        (
            spacetimedb_bindings::TypeValue::U32(field_0),
            spacetimedb_bindings::TypeValue::Bool(field_1),
        ) => Some(PlayerLogin {
            entity_id: field_0,
            logged_in: field_1,
        }),
        _ => None,
    };
}
#[allow(non_snake_case)]
fn __struct_to_tuple__PlayerLogin(value: PlayerLogin) -> spacetimedb_bindings::TypeValue {
    return spacetimedb_bindings::TypeValue::Tuple(spacetimedb_bindings::TupleValue {
        elements: <[_]>::into_vec(
            #[rustc_box]
            ::alloc::boxed::Box::new([
                spacetimedb_bindings::TypeValue::U32(value.entity_id),
                spacetimedb_bindings::TypeValue::Bool(value.logged_in),
            ]),
        ),
    });
}
pub struct Config {
    #[unique]
    pub version: u32,
    pub max_player_inventory_slots: u32,
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for Config {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = match _serde::Serializer::serialize_struct(
                __serializer,
                "Config",
                false as usize + 1 + 1,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "version",
                &self.version,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "max_player_inventory_slots",
                &self.max_player_inventory_slots,
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
    impl<'de> _serde::Deserialize<'de> for Config {
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
                        "version" => _serde::__private::Ok(__Field::__field0),
                        "max_player_inventory_slots" => _serde::__private::Ok(__Field::__field1),
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
                        b"version" => _serde::__private::Ok(__Field::__field0),
                        b"max_player_inventory_slots" => _serde::__private::Ok(__Field::__field1),
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
                marker: _serde::__private::PhantomData<Config>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = Config;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct Config")
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
                                    &"struct Config with 2 elements",
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
                                    &"struct Config with 2 elements",
                                ));
                            }
                        };
                    _serde::__private::Ok(Config {
                        version: __field0,
                        max_player_inventory_slots: __field1,
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
                                            "version",
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
                                            "max_player_inventory_slots",
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
                            match _serde::__private::de::missing_field("version") {
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
                            match _serde::__private::de::missing_field("max_player_inventory_slots")
                            {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    _serde::__private::Ok(Config {
                        version: __field0,
                        max_player_inventory_slots: __field1,
                    })
                }
            }
            const FIELDS: &'static [&'static str] = &["version", "max_player_inventory_slots"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "Config",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<Config>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl Config {
    #[allow(unused_variables)]
    pub fn insert(ins: Config) {
        spacetimedb_bindings::insert(
            6u32,
            spacetimedb_bindings::TupleValue {
                elements: <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        spacetimedb_bindings::TypeValue::U32(ins.version),
                        spacetimedb_bindings::TypeValue::U32(ins.max_player_inventory_slots),
                    ]),
                ),
            },
        );
    }
    #[allow(unused_variables)]
    pub fn delete(f: fn(Config) -> bool) -> usize {
        ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
            &["Delete using a function is not supported yet!"],
            &[],
        ));
    }
    #[allow(unused_variables)]
    pub fn update(value: Config) -> bool {
        ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
            &["Update using a value is not supported yet!"],
            &[],
        ));
    }
    #[allow(unused_variables)]
    #[allow(non_snake_case)]
    pub fn filter_version_eq(version: u32) -> Option<Config> {
        let table_iter = Config::iter();
        if let Some(table_iter) = table_iter {
            for row in table_iter {
                let column_data = row.elements[0usize].clone();
                if let spacetimedb_bindings::TypeValue::U32(entry_data) = column_data.clone() {
                    if entry_data == version {
                        let tuple = __tuple_to_struct__Config(row);
                        if let None = tuple {
                            ::spacetimedb_bindings::_console_log_info(&{
                                let res = :: alloc :: fmt :: format (:: core :: fmt :: Arguments :: new_v1 (& ["Internal stdb error: Can\'t convert from tuple to struct (wrong version?) Config"] , & [])) ;
                                res
                            });
                            return None;
                        }
                        return Some(tuple.unwrap());
                    }
                }
            }
        } else {
            ::spacetimedb_bindings::_console_log_info(&{
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &["Failed to get iterator for table: Config"],
                    &[],
                ));
                res
            });
        }
        return None;
    }
    #[allow(unused_variables)]
    #[allow(non_snake_case)]
    pub fn update_version_eq(version: u32, new_value: Config) -> bool {
        Config::delete_version_eq(version);
        Config::insert(new_value);
        true
    }
    #[allow(unused_variables)]
    #[allow(non_snake_case)]
    pub fn delete_version_eq(version: u32) -> bool {
        let data = version;
        let data = spacetimedb_bindings::TypeValue::U32(data);
        let equatable = spacetimedb_bindings::EqTypeValue::try_from(data);
        match equatable {
            Ok(value) => {
                let result = spacetimedb_bindings::delete_eq(6u32, 0u32, value);
                match result {
                    None => false,
                    Some(count) => count > 0,
                }
            }
            Err(e) => {
                ::spacetimedb_bindings::_console_log_info(&{
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &["This type is not equatable: ", " Error:"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(
                                &"spacetimedb_bindings :: TypeValue :: U32",
                            ),
                            ::core::fmt::ArgumentV1::new_display(&e),
                        ],
                    ));
                    res
                });
                false
            }
        }
    }
    #[allow(unused_variables)]
    pub fn iter() -> Option<spacetimedb_bindings::TableIter> {
        spacetimedb_bindings::__iter__(6u32)
    }
    #[allow(non_snake_case)]
    #[allow(unused_variables)]
    pub fn filter_max_player_inventory_slots_eq(max_player_inventory_slots: u32) -> Vec<Config> {
        let mut result = Vec::<Config>::new();
        let table_iter = Config::iter();
        if let Some(table_iter) = table_iter {
            for row in table_iter {
                let column_data = row.elements[1usize].clone();
                if let spacetimedb_bindings::TypeValue::U32(entry_data) = column_data.clone() {
                    if entry_data == max_player_inventory_slots {
                        let tuple = __tuple_to_struct__Config(row);
                        if let None = tuple {
                            ::spacetimedb_bindings::_console_log_info(&{
                                let res = :: alloc :: fmt :: format (:: core :: fmt :: Arguments :: new_v1 (& ["Internal stdb error: Can\'t convert from tuple to struct (wrong version?) Config"] , & [])) ;
                                res
                            });
                            continue;
                        }
                        result.push(tuple.unwrap());
                    }
                }
            }
        }
        return result;
    }
}
#[allow(non_snake_case)]
fn __get_struct_schema__Config() -> spacetimedb_bindings::TypeDef {
    return spacetimedb_bindings::TypeDef::Tuple {
        0: spacetimedb_bindings::TupleDef {
            elements: <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    spacetimedb_bindings::ElementDef {
                        tag: 0u8,
                        element_type: spacetimedb_bindings::TypeDef::U32,
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
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn __create_table__Config(arg_ptr: usize, arg_size: usize) {
    let def = __get_struct_schema__Config();
    if let spacetimedb_bindings::TypeDef::Tuple(tuple_def) = def {
        spacetimedb_bindings::create_table(6u32, tuple_def);
    } else {
        ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
            &["This type is not a tuple: {#original_struct_ident}"],
            &[],
        ));
    }
}
#[allow(non_snake_case)]
fn __tuple_to_struct__Config(value: spacetimedb_bindings::TupleValue) -> Option<Config> {
    let elements_arr = value.elements;
    return match (elements_arr[0usize].clone(), elements_arr[1usize].clone()) {
        (
            spacetimedb_bindings::TypeValue::U32(field_0),
            spacetimedb_bindings::TypeValue::U32(field_1),
        ) => Some(Config {
            version: field_0,
            max_player_inventory_slots: field_1,
        }),
        _ => None,
    };
}
#[allow(non_snake_case)]
fn __struct_to_tuple__Config(value: Config) -> spacetimedb_bindings::TypeValue {
    return spacetimedb_bindings::TypeValue::Tuple(spacetimedb_bindings::TupleValue {
        elements: <[_]>::into_vec(
            #[rustc_box]
            ::alloc::boxed::Box::new([
                spacetimedb_bindings::TypeValue::U32(value.version),
                spacetimedb_bindings::TypeValue::U32(value.max_player_inventory_slots),
            ]),
        ),
    });
}
#[automatically_derived]
impl ::core::marker::Copy for Config {}
#[automatically_derived]
impl ::core::clone::Clone for Config {
    #[inline]
    fn clone(&self) -> Config {
        let _: ::core::clone::AssertParamIsClone<u32>;
        *self
    }
}
pub struct PlayerChatMessage {
    pub player_id: u32,
    pub msg_time: u64,
    pub message: String,
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for PlayerChatMessage {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = match _serde::Serializer::serialize_struct(
                __serializer,
                "PlayerChatMessage",
                false as usize + 1 + 1 + 1,
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
                "msg_time",
                &self.msg_time,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "message",
                &self.message,
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
    impl<'de> _serde::Deserialize<'de> for PlayerChatMessage {
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
                        "player_id" => _serde::__private::Ok(__Field::__field0),
                        "msg_time" => _serde::__private::Ok(__Field::__field1),
                        "message" => _serde::__private::Ok(__Field::__field2),
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
                        b"msg_time" => _serde::__private::Ok(__Field::__field1),
                        b"message" => _serde::__private::Ok(__Field::__field2),
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
                marker: _serde::__private::PhantomData<PlayerChatMessage>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = PlayerChatMessage;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct PlayerChatMessage")
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
                                    &"struct PlayerChatMessage with 3 elements",
                                ));
                            }
                        };
                    let __field1 =
                        match match _serde::de::SeqAccess::next_element::<u64>(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct PlayerChatMessage with 3 elements",
                                ));
                            }
                        };
                    let __field2 =
                        match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    2usize,
                                    &"struct PlayerChatMessage with 3 elements",
                                ));
                            }
                        };
                    _serde::__private::Ok(PlayerChatMessage {
                        player_id: __field0,
                        msg_time: __field1,
                        message: __field2,
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
                    let mut __field1: _serde::__private::Option<u64> = _serde::__private::None;
                    let mut __field2: _serde::__private::Option<String> = _serde::__private::None;
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
                                            "msg_time",
                                        ),
                                    );
                                }
                                __field1 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<u64>(&mut __map) {
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
                                            "message",
                                        ),
                                    );
                                }
                                __field2 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<String>(&mut __map) {
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
                            match _serde::__private::de::missing_field("msg_time") {
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
                            match _serde::__private::de::missing_field("message") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    _serde::__private::Ok(PlayerChatMessage {
                        player_id: __field0,
                        msg_time: __field1,
                        message: __field2,
                    })
                }
            }
            const FIELDS: &'static [&'static str] = &["player_id", "msg_time", "message"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "PlayerChatMessage",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<PlayerChatMessage>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl PlayerChatMessage {
    #[allow(unused_variables)]
    pub fn insert(ins: PlayerChatMessage) {
        spacetimedb_bindings::insert(
            7u32,
            spacetimedb_bindings::TupleValue {
                elements: <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        spacetimedb_bindings::TypeValue::U32(ins.player_id),
                        spacetimedb_bindings::TypeValue::U64(ins.msg_time),
                        spacetimedb_bindings::TypeValue::String(ins.message),
                    ]),
                ),
            },
        );
    }
    #[allow(unused_variables)]
    pub fn delete(f: fn(PlayerChatMessage) -> bool) -> usize {
        ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
            &["Delete using a function is not supported yet!"],
            &[],
        ));
    }
    #[allow(unused_variables)]
    pub fn update(value: PlayerChatMessage) -> bool {
        ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
            &["Update using a value is not supported yet!"],
            &[],
        ));
    }
    #[allow(unused_variables)]
    pub fn iter() -> Option<spacetimedb_bindings::TableIter> {
        spacetimedb_bindings::__iter__(7u32)
    }
    #[allow(non_snake_case)]
    #[allow(unused_variables)]
    pub fn filter_player_id_eq(player_id: u32) -> Vec<PlayerChatMessage> {
        let mut result = Vec::<PlayerChatMessage>::new();
        let table_iter = PlayerChatMessage::iter();
        if let Some(table_iter) = table_iter {
            for row in table_iter {
                let column_data = row.elements[0usize].clone();
                if let spacetimedb_bindings::TypeValue::U32(entry_data) = column_data.clone() {
                    if entry_data == player_id {
                        let tuple = __tuple_to_struct__PlayerChatMessage(row);
                        if let None = tuple {
                            ::spacetimedb_bindings::_console_log_info(&{
                                let res = :: alloc :: fmt :: format (:: core :: fmt :: Arguments :: new_v1 (& ["Internal stdb error: Can\'t convert from tuple to struct (wrong version?) PlayerChatMessage"] , & [])) ;
                                res
                            });
                            continue;
                        }
                        result.push(tuple.unwrap());
                    }
                }
            }
        }
        return result;
    }
    #[allow(non_snake_case)]
    #[allow(unused_variables)]
    pub fn filter_msg_time_eq(msg_time: u64) -> Vec<PlayerChatMessage> {
        let mut result = Vec::<PlayerChatMessage>::new();
        let table_iter = PlayerChatMessage::iter();
        if let Some(table_iter) = table_iter {
            for row in table_iter {
                let column_data = row.elements[1usize].clone();
                if let spacetimedb_bindings::TypeValue::U64(entry_data) = column_data.clone() {
                    if entry_data == msg_time {
                        let tuple = __tuple_to_struct__PlayerChatMessage(row);
                        if let None = tuple {
                            ::spacetimedb_bindings::_console_log_info(&{
                                let res = :: alloc :: fmt :: format (:: core :: fmt :: Arguments :: new_v1 (& ["Internal stdb error: Can\'t convert from tuple to struct (wrong version?) PlayerChatMessage"] , & [])) ;
                                res
                            });
                            continue;
                        }
                        result.push(tuple.unwrap());
                    }
                }
            }
        }
        return result;
    }
    #[allow(non_snake_case)]
    #[allow(unused_variables)]
    pub fn filter_message_eq(message: String) -> Vec<PlayerChatMessage> {
        let mut result = Vec::<PlayerChatMessage>::new();
        let table_iter = PlayerChatMessage::iter();
        if let Some(table_iter) = table_iter {
            for row in table_iter {
                let column_data = row.elements[2usize].clone();
                if let spacetimedb_bindings::TypeValue::String(entry_data) = column_data.clone() {
                    if entry_data == message {
                        let tuple = __tuple_to_struct__PlayerChatMessage(row);
                        if let None = tuple {
                            ::spacetimedb_bindings::_console_log_info(&{
                                let res = :: alloc :: fmt :: format (:: core :: fmt :: Arguments :: new_v1 (& ["Internal stdb error: Can\'t convert from tuple to struct (wrong version?) PlayerChatMessage"] , & [])) ;
                                res
                            });
                            continue;
                        }
                        result.push(tuple.unwrap());
                    }
                }
            }
        }
        return result;
    }
}
#[allow(non_snake_case)]
fn __get_struct_schema__PlayerChatMessage() -> spacetimedb_bindings::TypeDef {
    return spacetimedb_bindings::TypeDef::Tuple {
        0: spacetimedb_bindings::TupleDef {
            elements: <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    spacetimedb_bindings::ElementDef {
                        tag: 0u8,
                        element_type: spacetimedb_bindings::TypeDef::U32,
                    },
                    spacetimedb_bindings::ElementDef {
                        tag: 1u8,
                        element_type: spacetimedb_bindings::TypeDef::U64,
                    },
                    spacetimedb_bindings::ElementDef {
                        tag: 2u8,
                        element_type: spacetimedb_bindings::TypeDef::String,
                    },
                ]),
            ),
        },
    };
}
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn __create_table__PlayerChatMessage(arg_ptr: usize, arg_size: usize) {
    let def = __get_struct_schema__PlayerChatMessage();
    if let spacetimedb_bindings::TypeDef::Tuple(tuple_def) = def {
        spacetimedb_bindings::create_table(7u32, tuple_def);
    } else {
        ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
            &["This type is not a tuple: {#original_struct_ident}"],
            &[],
        ));
    }
}
#[allow(non_snake_case)]
fn __tuple_to_struct__PlayerChatMessage(
    value: spacetimedb_bindings::TupleValue,
) -> Option<PlayerChatMessage> {
    let elements_arr = value.elements;
    return match (
        elements_arr[0usize].clone(),
        elements_arr[1usize].clone(),
        elements_arr[2usize].clone(),
    ) {
        (
            spacetimedb_bindings::TypeValue::U32(field_0),
            spacetimedb_bindings::TypeValue::U64(field_1),
            spacetimedb_bindings::TypeValue::String(field_2),
        ) => Some(PlayerChatMessage {
            player_id: field_0,
            msg_time: field_1,
            message: field_2,
        }),
        _ => None,
    };
}
#[allow(non_snake_case)]
fn __struct_to_tuple__PlayerChatMessage(
    value: PlayerChatMessage,
) -> spacetimedb_bindings::TypeValue {
    return spacetimedb_bindings::TypeValue::Tuple(spacetimedb_bindings::TupleValue {
        elements: <[_]>::into_vec(
            #[rustc_box]
            ::alloc::boxed::Box::new([
                spacetimedb_bindings::TypeValue::U32(value.player_id),
                spacetimedb_bindings::TypeValue::U64(value.msg_time),
                spacetimedb_bindings::TypeValue::String(value.message),
            ]),
        ),
    });
}
impl EntityInventory {
    fn get_pocket(&self, pocket_idx: u32) -> Option<Pocket> {
        for x in 0..self.pockets.len() {
            if self.pockets[x].pocket_idx == pocket_idx && self.pockets[x].item_count > 0 {
                return Some(self.pockets[x]);
            }
        }
        return None;
    }
    fn set_pocket(&mut self, pocket: Pocket) {
        for x in 0..self.pockets.len() {
            if self.pockets[x].pocket_idx == pocket.pocket_idx {
                self.pockets[x] = pocket;
                return;
            }
        }
        self.pockets.push(pocket);
    }
    fn delete_pocket(&mut self, pocket_idx: u32) {
        for x in 0..self.pockets.len() {
            if self.pockets[x].pocket_idx == pocket_idx {
                self.pockets.remove(x);
                return;
            }
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __reducer__initialize(arg_ptr: usize, arg_size: usize) {
    let arguments = spacetimedb_bindings::args::ReducerArguments::decode_mem(
        unsafe { arg_ptr as *mut u8 },
        arg_size,
    )
    .expect("Unable to decode module arguments");
    initialize(arguments.identity, arguments.timestamp);
}
pub fn initialize(_identity: Hash, _timestamp: u64) {
    match Config::filter_version_eq(0) {
        Some(_) => {
            ::spacetimedb_bindings::_console_log_info(&{
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &["Config already exists, skipping config."],
                    &[],
                ));
                res
            });
        }
        None => {
            ::spacetimedb_bindings::_console_log_info(&{
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &["Creating new config!"],
                    &[],
                ));
                res
            });
            Config::insert(Config {
                version: 0,
                max_player_inventory_slots: 30,
            });
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __reducer__move_or_swap_inventory_slot(arg_ptr: usize, arg_size: usize) {
    let arguments = spacetimedb_bindings::args::ReducerArguments::decode_mem(
        unsafe { arg_ptr as *mut u8 },
        arg_size,
    )
    .expect("Unable to decode module arguments");
    let arg_json: serde_json::Value = serde_json::from_slice(arguments.argument_bytes.as_slice())
        .expect(
            {
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &[
                        "Unable to parse arguments as JSON: ",
                        " bytes/arg_size: ",
                        ": ",
                    ],
                    &[
                        ::core::fmt::ArgumentV1::new_display(&arguments.argument_bytes.len()),
                        ::core::fmt::ArgumentV1::new_display(&arg_size),
                        ::core::fmt::ArgumentV1::new_debug(&arguments.argument_bytes),
                    ],
                ));
                res
            }
            .as_str(),
        );
    let args = arg_json
        .as_array()
        .expect("Unable to extract reducer arguments list");
    let arg_2: u32 = serde_json::from_value(args[0usize].clone()).unwrap();
    let arg_3: u32 = serde_json::from_value(args[1usize].clone()).unwrap();
    let arg_4: u32 = serde_json::from_value(args[2usize].clone()).unwrap();
    move_or_swap_inventory_slot(arguments.identity, arguments.timestamp, arg_2, arg_3, arg_4);
}
pub fn move_or_swap_inventory_slot(
    identity: Hash,
    _timestamp: u64,
    entity_id: u32,
    source_pocket_idx: u32,
    dest_pocket_idx: u32,
) {
    let config = Config::filter_version_eq(0).unwrap();
    if source_pocket_idx >= config.max_player_inventory_slots {
        ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
            &["move_or_swap_inventory_slot: The source pocket index is invalid: "],
            &[::core::fmt::ArgumentV1::new_display(&source_pocket_idx)],
        ));
    }
    if dest_pocket_idx >= config.max_player_inventory_slots {
        ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
            &["move_or_swap_inventory_slot: The dest pocket index is invalid: "],
            &[::core::fmt::ArgumentV1::new_display(&dest_pocket_idx)],
        ));
    }
    let player = Player::filter_entity_id_eq(entity_id)
        .expect("move_or_swap_inventory_slot: This player doesn't exist!");
    if player.owner_id != identity {
        ::spacetimedb_bindings::_console_log_info(&{
            let res = :: alloc :: fmt :: format (:: core :: fmt :: Arguments :: new_v1 (& ["move_or_swap_inventory_slot: This identity doesn\'t own this player! (allowed for now)"] , & [])) ;
            res
        });
    }
    let mut inventory = EntityInventory::filter_entity_id_eq(entity_id)
        .expect("move_or_swap_inventory_slot: This player doesn't have an inventory!");
    let mut source_pocket = inventory
        .get_pocket(source_pocket_idx)
        .expect("move_or_swap_inventory_slot: Nothing in source pocket, nothing to do.");
    let dest_pocket = inventory.get_pocket(dest_pocket_idx);
    if let None = dest_pocket {
        inventory.delete_pocket(source_pocket_idx);
        source_pocket.pocket_idx = dest_pocket_idx;
        inventory.set_pocket(source_pocket);
        EntityInventory::update_entity_id_eq(entity_id, inventory);
        ::spacetimedb_bindings::_console_log_info(&{
            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                &["move_or_swap_inventory_slot: Source pocket moved to dest pocket."],
                &[],
            ));
            res
        });
        return;
    }
    let mut dest_pocket = dest_pocket.unwrap();
    if source_pocket.item_id == dest_pocket.item_id {
        dest_pocket.item_count += source_pocket.item_count;
        inventory.delete_pocket(source_pocket_idx);
        inventory.set_pocket(dest_pocket);
        EntityInventory::update_entity_id_eq(entity_id, inventory);
        ::spacetimedb_bindings::_console_log_info(&{
            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                &["move_or_swap_inventory_slot: Source pocket moved into dest pocket (same item)"],
                &[],
            ));
            res
        });
        return;
    }
    inventory.delete_pocket(source_pocket_idx);
    inventory.delete_pocket(dest_pocket_idx);
    dest_pocket.pocket_idx = source_pocket_idx;
    source_pocket.pocket_idx = dest_pocket_idx;
    inventory.set_pocket(source_pocket);
    inventory.set_pocket(dest_pocket);
    EntityInventory::update_entity_id_eq(entity_id, inventory);
    ::spacetimedb_bindings::_console_log_info(&{
        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
            &["move_or_swap_inventory_slot: Pockets swapped (different items)"],
            &[],
        ));
        res
    });
}
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __reducer__add_item_to_inventory(arg_ptr: usize, arg_size: usize) {
    let arguments = spacetimedb_bindings::args::ReducerArguments::decode_mem(
        unsafe { arg_ptr as *mut u8 },
        arg_size,
    )
    .expect("Unable to decode module arguments");
    let arg_json: serde_json::Value = serde_json::from_slice(arguments.argument_bytes.as_slice())
        .expect(
            {
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &[
                        "Unable to parse arguments as JSON: ",
                        " bytes/arg_size: ",
                        ": ",
                    ],
                    &[
                        ::core::fmt::ArgumentV1::new_display(&arguments.argument_bytes.len()),
                        ::core::fmt::ArgumentV1::new_display(&arg_size),
                        ::core::fmt::ArgumentV1::new_debug(&arguments.argument_bytes),
                    ],
                ));
                res
            }
            .as_str(),
        );
    let args = arg_json
        .as_array()
        .expect("Unable to extract reducer arguments list");
    let arg_2: u32 = serde_json::from_value(args[0usize].clone()).unwrap();
    let arg_3: u32 = serde_json::from_value(args[1usize].clone()).unwrap();
    let arg_4: u32 = serde_json::from_value(args[2usize].clone()).unwrap();
    let arg_5: i32 = serde_json::from_value(args[3usize].clone()).unwrap();
    add_item_to_inventory(
        arguments.identity,
        arguments.timestamp,
        arg_2,
        arg_3,
        arg_4,
        arg_5,
    );
}
/// This adds or removes items from an inventory slot. you can pass a negative item count in order
/// to remove items.
pub fn add_item_to_inventory(
    identity: Hash,
    _timestamp: u64,
    entity_id: u32,
    item_id: u32,
    pocket_idx: u32,
    item_count: i32,
) {
    let config = Config::filter_version_eq(0).unwrap();
    if !(pocket_idx < config.max_player_inventory_slots) {
        ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
            &["add_item_to_inventory: This pocket index is invalid: "],
            &[::core::fmt::ArgumentV1::new_display(&pocket_idx)],
        ))
    };
    let player = Player::filter_entity_id_eq(entity_id)
        .expect("add_item_to_inventory: This player doesn't exist!");
    if player.owner_id != identity {
        ::spacetimedb_bindings::_console_log_info(&{
            let res = :: alloc :: fmt :: format (:: core :: fmt :: Arguments :: new_v1 (& ["add_item_to_inventory: This identity doesn\'t own this player! (allowed for now)"] , & [])) ;
            res
        });
    }
    let mut inventory = EntityInventory::filter_entity_id_eq(entity_id)
        .expect("add_item_to_inventory: This player doesn't have an inventory!");
    match inventory.get_pocket(pocket_idx) {
        Some(mut pocket) => {
            match (&pocket.item_id, &item_id) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::Some(::core::fmt::Arguments::new_v1(
                                &["add_item_to_inventory: Item ID mismatch"],
                                &[],
                            )),
                        );
                    }
                }
            };
            pocket.item_count += item_count;
        }
        None => {
            inventory.set_pocket(Pocket {
                pocket_idx,
                item_id,
                item_count,
            });
        }
    }
    EntityInventory::update_entity_id_eq(entity_id, inventory);
    ::spacetimedb_bindings::_console_log_info(&{
        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
            &["add_item_to_inventory: Item ", " inserted into inventory "],
            &[
                ::core::fmt::ArgumentV1::new_display(&item_id),
                ::core::fmt::ArgumentV1::new_display(&entity_id),
            ],
        ));
        res
    });
}
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __reducer__dump_inventory(arg_ptr: usize, arg_size: usize) {
    let arguments = spacetimedb_bindings::args::ReducerArguments::decode_mem(
        unsafe { arg_ptr as *mut u8 },
        arg_size,
    )
    .expect("Unable to decode module arguments");
    let arg_json: serde_json::Value = serde_json::from_slice(arguments.argument_bytes.as_slice())
        .expect(
            {
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &[
                        "Unable to parse arguments as JSON: ",
                        " bytes/arg_size: ",
                        ": ",
                    ],
                    &[
                        ::core::fmt::ArgumentV1::new_display(&arguments.argument_bytes.len()),
                        ::core::fmt::ArgumentV1::new_display(&arg_size),
                        ::core::fmt::ArgumentV1::new_debug(&arguments.argument_bytes),
                    ],
                ));
                res
            }
            .as_str(),
        );
    let args = arg_json
        .as_array()
        .expect("Unable to extract reducer arguments list");
    let arg_2: u32 = serde_json::from_value(args[0usize].clone()).unwrap();
    dump_inventory(arguments.identity, arguments.timestamp, arg_2);
}
pub fn dump_inventory(_identity: Hash, _timestamp: u64, entity_id: u32) {
    let inventory = EntityInventory::filter_entity_id_eq(entity_id);
    if !inventory.is_some() {
        ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
            &["Inventory NOT found for entity:: "],
            &[::core::fmt::ArgumentV1::new_display(&entity_id)],
        ))
    };
    let inventory = inventory.unwrap();
    ::spacetimedb_bindings::_console_log_info(&{
        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
            &["Inventory found for entity: "],
            &[::core::fmt::ArgumentV1::new_display(&entity_id)],
        ));
        res
    });
    for pocket in inventory.pockets {
        ::spacetimedb_bindings::_console_log_info(&{
            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                &["PocketIdx: ", " Item: ", " Count: "],
                &[
                    ::core::fmt::ArgumentV1::new_display(&pocket.pocket_idx),
                    ::core::fmt::ArgumentV1::new_display(&pocket.item_id),
                    ::core::fmt::ArgumentV1::new_display(&pocket.item_count),
                ],
            ));
            res
        });
    }
}
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __reducer__move_player(arg_ptr: usize, arg_size: usize) {
    let arguments = spacetimedb_bindings::args::ReducerArguments::decode_mem(
        unsafe { arg_ptr as *mut u8 },
        arg_size,
    )
    .expect("Unable to decode module arguments");
    let arg_json: serde_json::Value = serde_json::from_slice(arguments.argument_bytes.as_slice())
        .expect(
            {
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &[
                        "Unable to parse arguments as JSON: ",
                        " bytes/arg_size: ",
                        ": ",
                    ],
                    &[
                        ::core::fmt::ArgumentV1::new_display(&arguments.argument_bytes.len()),
                        ::core::fmt::ArgumentV1::new_display(&arg_size),
                        ::core::fmt::ArgumentV1::new_debug(&arguments.argument_bytes),
                    ],
                ));
                res
            }
            .as_str(),
        );
    let args = arg_json
        .as_array()
        .expect("Unable to extract reducer arguments list");
    let arg_2: u32 = serde_json::from_value(args[0usize].clone()).unwrap();
    let arg_3: f32 = serde_json::from_value(args[1usize].clone()).unwrap();
    let arg_4: f32 = serde_json::from_value(args[2usize].clone()).unwrap();
    let arg_5: f32 = serde_json::from_value(args[3usize].clone()).unwrap();
    let arg_6: f32 = serde_json::from_value(args[4usize].clone()).unwrap();
    let arg_7: f32 = serde_json::from_value(args[5usize].clone()).unwrap();
    let arg_8: f32 = serde_json::from_value(args[6usize].clone()).unwrap();
    let arg_9: f32 = serde_json::from_value(args[7usize].clone()).unwrap();
    move_player(
        arguments.identity,
        arguments.timestamp,
        arg_2,
        arg_3,
        arg_4,
        arg_5,
        arg_6,
        arg_7,
        arg_8,
        arg_9,
    );
}
pub fn move_player(
    identity: Hash,
    _timestamp: u64,
    entity_id: u32,
    pos_x: f32,
    pos_y: f32,
    pos_z: f32,
    rot_x: f32,
    rot_y: f32,
    rot_z: f32,
    rot_w: f32,
) {
    match Player::filter_entity_id_eq(entity_id) {
        Some(player) => {
            if player.owner_id != identity {
                ::spacetimedb_bindings::_console_log_info(&{
                    let res = :: alloc :: fmt :: format (:: core :: fmt :: Arguments :: new_v1 (& ["move_player: This identity doesn\'t own this player! (allowed for now)"] , & [])) ;
                    res
                });
            }
        }
        None => {
            ::spacetimedb_bindings::_console_log_info(&{
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &["move_player: This player doesn\'t exist: "],
                    &[::core::fmt::ArgumentV1::new_display(&entity_id)],
                ));
                res
            });
            return;
        }
    }
    EntityTransform::update_entity_id_eq(
        entity_id,
        EntityTransform {
            entity_id,
            pos_x,
            pos_y,
            pos_z,
            rot_x,
            rot_y,
            rot_z,
            rot_w,
        },
    );
}
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __reducer__update_player_animation(arg_ptr: usize, arg_size: usize) {
    let arguments = spacetimedb_bindings::args::ReducerArguments::decode_mem(
        unsafe { arg_ptr as *mut u8 },
        arg_size,
    )
    .expect("Unable to decode module arguments");
    let arg_json: serde_json::Value = serde_json::from_slice(arguments.argument_bytes.as_slice())
        .expect(
            {
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &[
                        "Unable to parse arguments as JSON: ",
                        " bytes/arg_size: ",
                        ": ",
                    ],
                    &[
                        ::core::fmt::ArgumentV1::new_display(&arguments.argument_bytes.len()),
                        ::core::fmt::ArgumentV1::new_display(&arg_size),
                        ::core::fmt::ArgumentV1::new_debug(&arguments.argument_bytes),
                    ],
                ));
                res
            }
            .as_str(),
        );
    let args = arg_json
        .as_array()
        .expect("Unable to extract reducer arguments list");
    let arg_2: u32 = serde_json::from_value(args[0usize].clone()).unwrap();
    let arg_3: bool = serde_json::from_value(args[1usize].clone()).unwrap();
    update_player_animation(arguments.identity, arguments.timestamp, arg_2, arg_3);
}
pub fn update_player_animation(identity: Hash, _timestamp: u64, entity_id: u32, moving: bool) {
    match Player::filter_entity_id_eq(entity_id) {
        Some(player) => {
            if player.owner_id != identity {
                ::spacetimedb_bindings::_console_log_info(&{
                    let res = :: alloc :: fmt :: format (:: core :: fmt :: Arguments :: new_v1 (& ["update_player_animation: This identity doesn\'t own this player! (allowed for now)"] , & [])) ;
                    res
                });
            }
        }
        None => {
            ::spacetimedb_bindings::_console_log_info(&{
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &["update_player_animation: This player doesn\'t exist!"],
                    &[],
                ));
                res
            });
            return;
        }
    }
    PlayerAnimation::update_entity_id_eq(entity_id, PlayerAnimation { entity_id, moving });
}
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __reducer__create_new_player(arg_ptr: usize, arg_size: usize) {
    let arguments = spacetimedb_bindings::args::ReducerArguments::decode_mem(
        unsafe { arg_ptr as *mut u8 },
        arg_size,
    )
    .expect("Unable to decode module arguments");
    let arg_json: serde_json::Value = serde_json::from_slice(arguments.argument_bytes.as_slice())
        .expect(
            {
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &[
                        "Unable to parse arguments as JSON: ",
                        " bytes/arg_size: ",
                        ": ",
                    ],
                    &[
                        ::core::fmt::ArgumentV1::new_display(&arguments.argument_bytes.len()),
                        ::core::fmt::ArgumentV1::new_display(&arg_size),
                        ::core::fmt::ArgumentV1::new_debug(&arguments.argument_bytes),
                    ],
                ));
                res
            }
            .as_str(),
        );
    let args = arg_json
        .as_array()
        .expect("Unable to extract reducer arguments list");
    let arg_2: u32 = serde_json::from_value(args[0usize].clone()).unwrap();
    create_new_player(arguments.identity, arguments.timestamp, arg_2);
}
pub fn create_new_player(identity: Hash, timestamp: u64, entity_id: u32) {
    if let Some(_) = Player::filter_entity_id_eq(entity_id) {
        ::spacetimedb_bindings::_console_log_info(&{
            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                &["create_new_player: A player with this entity_id already exists: "],
                &[::core::fmt::ArgumentV1::new_display(&entity_id)],
            ));
            res
        });
        return;
    }
    ::spacetimedb_bindings::_console_log_info(&{
        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
            &["create_new_player: player created: "],
            &[::core::fmt::ArgumentV1::new_display(&entity_id)],
        ));
        res
    });
    Player::insert(Player {
        entity_id,
        owner_id: identity,
        creation_time: timestamp,
    });
    EntityInventory::insert(EntityInventory {
        entity_id,
        pockets: Vec::<Pocket>::new(),
    });
}
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __reducer__player_chat(arg_ptr: usize, arg_size: usize) {
    let arguments = spacetimedb_bindings::args::ReducerArguments::decode_mem(
        unsafe { arg_ptr as *mut u8 },
        arg_size,
    )
    .expect("Unable to decode module arguments");
    let arg_json: serde_json::Value = serde_json::from_slice(arguments.argument_bytes.as_slice())
        .expect(
            {
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &[
                        "Unable to parse arguments as JSON: ",
                        " bytes/arg_size: ",
                        ": ",
                    ],
                    &[
                        ::core::fmt::ArgumentV1::new_display(&arguments.argument_bytes.len()),
                        ::core::fmt::ArgumentV1::new_display(&arg_size),
                        ::core::fmt::ArgumentV1::new_debug(&arguments.argument_bytes),
                    ],
                ));
                res
            }
            .as_str(),
        );
    let args = arg_json
        .as_array()
        .expect("Unable to extract reducer arguments list");
    let arg_2: u32 = serde_json::from_value(args[0usize].clone()).unwrap();
    let arg_3: String = serde_json::from_value(args[1usize].clone()).unwrap();
    player_chat(arguments.identity, arguments.timestamp, arg_2, arg_3);
}
pub fn player_chat(_identity: Hash, timestamp: u64, player_id: u32, message: String) {
    let chat = PlayerChatMessage {
        player_id,
        msg_time: timestamp,
        message,
    };
    PlayerChatMessage::insert(chat);
}
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __reducer__player_update_login_state(arg_ptr: usize, arg_size: usize) {
    let arguments = spacetimedb_bindings::args::ReducerArguments::decode_mem(
        unsafe { arg_ptr as *mut u8 },
        arg_size,
    )
    .expect("Unable to decode module arguments");
    let arg_json: serde_json::Value = serde_json::from_slice(arguments.argument_bytes.as_slice())
        .expect(
            {
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &[
                        "Unable to parse arguments as JSON: ",
                        " bytes/arg_size: ",
                        ": ",
                    ],
                    &[
                        ::core::fmt::ArgumentV1::new_display(&arguments.argument_bytes.len()),
                        ::core::fmt::ArgumentV1::new_display(&arg_size),
                        ::core::fmt::ArgumentV1::new_debug(&arguments.argument_bytes),
                    ],
                ));
                res
            }
            .as_str(),
        );
    let args = arg_json
        .as_array()
        .expect("Unable to extract reducer arguments list");
    let arg_2: bool = serde_json::from_value(args[0usize].clone()).unwrap();
    player_update_login_state(arguments.identity, arguments.timestamp, arg_2);
}
pub fn player_update_login_state(identity: Hash, _timestamp: u64, logged_in: bool) {
    match Player::filter_owner_id_eq(identity) {
        Some(player) => match PlayerLogin::filter_entity_id_eq(player.entity_id) {
            Some(login_state) => {
                if !(login_state.logged_in != logged_in) {
                    ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                        &["Player is already set to this login state: "],
                        &[::core::fmt::ArgumentV1::new_display(&logged_in)],
                    ))
                };
            }
            None => {
                ::spacetimedb_bindings::_console_log_info(&{
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &["Player set login state to: "],
                        &[::core::fmt::ArgumentV1::new_display(&logged_in)],
                    ));
                    res
                });
                PlayerLogin::update(PlayerLogin {
                    entity_id: player.entity_id,
                    logged_in,
                });
            }
        },
        None => {
            ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                &["You cannot sign in without a player!"],
                &[],
            ));
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __identity_connected__(arg_ptr: usize, arg_size: usize) {
    let arguments = spacetimedb_bindings::args::ConnectDisconnectArguments::decode_mem(
        unsafe { arg_ptr as *mut u8 },
        arg_size,
    )
    .expect("Unable to decode module arguments");
    identity_connected(arguments.identity, arguments.timestamp);
}
pub fn identity_connected(identity: Hash, _timestamp: u64) {
    match Player::filter_owner_id_eq(identity) {
        Some(_) => {
            ::spacetimedb_bindings::_console_log_info(&{
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &["Player has returned."],
                    &[],
                ));
                res
            });
        }
        None => {
            ::spacetimedb_bindings::_console_log_info(&{
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &["A new identity has connected."],
                    &[],
                ));
                res
            });
        }
    }
}
#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn __identity_disconnected__(arg_ptr: usize, arg_size: usize) {
    let arguments = spacetimedb_bindings::args::ConnectDisconnectArguments::decode_mem(
        unsafe { arg_ptr as *mut u8 },
        arg_size,
    )
    .expect("Unable to decode module arguments");
    identity_disconnected(arguments.identity, arguments.timestamp);
}
pub fn identity_disconnected(identity: Hash, _timestamp: u64) {
    if let Some(player) = Player::filter_owner_id_eq(identity) {
        if let Some(login_state) = PlayerLogin::filter_entity_id_eq(player.entity_id) {
            if login_state.logged_in {
                ::spacetimedb_bindings::_console_log_info(&{
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &["User has disconnected without signing out."],
                        &[],
                    ));
                    res
                });
                PlayerLogin::update_entity_id_eq(
                    player.entity_id,
                    PlayerLogin {
                        entity_id: player.entity_id,
                        logged_in: false,
                    },
                );
            }
        }
    }
}
