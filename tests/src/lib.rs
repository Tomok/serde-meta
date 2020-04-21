#[cfg(test)]
mod tests {
    use serde_meta::*;
    use serde_meta_derive;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    fn calculate_hash<T: Hash>(t: &T) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }

    mod test_derive_u32_field {
        use super::*;

        #[allow(unused)] //dummy struct just to check meta information generated for it
        #[derive(serde_meta_derive::SerdeMeta)]
        struct A {
            f: u32,
        }

        #[test]
        fn test() {
            let meta = A::meta();
            if let TypeInformation::StructValue(named_type_info) = meta {
                assert_eq!("A", named_type_info.name());
                let fields = named_type_info.type_info().fields();
                assert_eq!(1, fields.len());
                assert_eq!("f", fields[0].name());
                assert_eq!(TypeInformation::U32Value(), fields[0].inner_type());
            } else {
                panic!("Expected StructValue, but got {:#?}", meta);
            }
        }

        #[test]
        fn test_hash() {
            let meta1 = A::meta();
            let meta2 = A::meta();
            assert_eq!(calculate_hash(&meta1), calculate_hash(&meta2));
        }

        #[test]
        fn test_eq() {
            let meta1 = A::meta();
            let meta2 = A::meta();
            assert_eq!(meta1, meta2);
        }

        #[cfg(feature = "serde_ser")]
        #[test]
        fn test_serialization() {
            let meta = A::meta();
            let res = serde_json::to_string(&meta).unwrap();
            assert_eq!("{\"StructValue\":{\"id\":0,\"name\":\"A\",\"fields\":[{\"name\":\"f\",\"inner_type\":\"U32Value\"}]}}", res);
        }
    }

    mod test_derive_empty_struct {
        use super::*;

        #[derive(serde_meta_derive::SerdeMeta)]
        struct A {}

        #[test]
        fn test_hash() {
            let meta1 = A::meta();
            let meta2 = A::meta();
            assert_eq!(calculate_hash(&meta1), calculate_hash(&meta2));
        }

        #[test]
        fn test_eq() {
            let meta1 = A::meta();
            let meta2 = A::meta();
            assert_eq!(meta1, meta2);
        }

        #[test]
        fn test() {
            let meta = A::meta();
            if let TypeInformation::StructValue(named_type_info) = meta {
                let fields = named_type_info.type_info().fields();
                assert_eq!("A", named_type_info.name());
                let expected_fields: &[Field] = &[];
                assert_eq!(expected_fields, fields);
            } else {
                panic!("Expected StructValue, but got {:#?}", meta);
            }
        }

        #[cfg(feature = "serde_ser")]
        #[test]
        fn test_serialization() {
            let meta = A::meta();
            let res = serde_json::to_string(&meta).unwrap();
            assert_eq!(
                "{\"StructValue\":{\"id\":0,\"name\":\"A\",\"fields\":[]}}",
                res
            );
        }
    }

    mod test_derive_single_field_struct {
        use super::*;

        #[derive(serde_meta_derive::SerdeMeta)]
        struct B;

        #[allow(unused)] //dummy struct just to check meta information generated for it
        #[derive(serde_meta_derive::SerdeMeta)]
        struct A {
            f: B,
        }

        #[test]
        fn test_hash() {
            let meta1 = A::meta();
            let meta2 = A::meta();
            assert_eq!(calculate_hash(&meta1), calculate_hash(&meta2));

            let meta_b = B::meta();
            assert_ne!(calculate_hash(&meta1), calculate_hash(&meta_b));
        }

        #[test]
        fn test_eq() {
            let meta1 = A::meta();
            let meta2 = A::meta();
            assert_eq!(meta1, meta2);
            let meta_b = B::meta();
            assert_ne!(&meta1, &meta_b);
        }

        #[test]
        fn test() {
            let meta = A::meta();
            if let TypeInformation::StructValue(named_type_info) = meta {
                assert_eq!("A", named_type_info.name());
                let fields = named_type_info.type_info().fields();
                assert_eq!(1, fields.len());
                assert_eq!("f", fields[0].name());
                assert_eq!(B::meta(), fields[0].inner_type());
            } else {
                panic!("Expected StructValue, but got {:#?}", meta);
            }
        }

        #[cfg(feature = "serde_ser")]
        #[test]
        fn test_serialization() {
            let meta = A::meta();
            let res = serde_json::to_string(&meta).unwrap();
            assert_eq!("{\"StructValue\":{\"id\":0,\"name\":\"A\",\"fields\":[{\"name\":\"f\",\"inner_type\":{\"UnitStructValue\":{\"name\":\"B\"}}}]}}", res);
        }
    }

    mod test_derive_multiple_unnamed_fields_struct {
        use super::*;

        #[allow(unused)]
        #[derive(serde_meta_derive::SerdeMeta)]
        struct A(u8, u16, u32);

        #[test]
        fn test() {
            let meta = A::meta();
            if let TypeInformation::TupleStructValue(named_type_info) = meta {
                assert_eq!("A", named_type_info.name());
                let inner_types = named_type_info.type_info().inner_types();
                assert_eq!(3, inner_types.len());
                assert_eq!(TypeInformation::U8Value(), inner_types[0].inner_type());
                assert_eq!(TypeInformation::U16Value(), inner_types[1].inner_type());
                assert_eq!(TypeInformation::U32Value(), inner_types[2].inner_type());
            } else {
                panic!("Expected TupleStructValue, but got {:#?}", meta);
            }
        }

        #[test]
        fn test_hash() {
            let meta1 = A::meta();
            let meta2 = A::meta();
            assert_eq!(calculate_hash(&meta1), calculate_hash(&meta2));
        }

        #[test]
        fn test_eq() {
            let meta1 = A::meta();
            let meta2 = A::meta();
            assert_eq!(meta1, meta2);
        }

        #[cfg(feature = "serde_ser")]
        #[test]
        fn test_serialization() {
            let meta = A::meta();
            let res = serde_json::to_string(&meta).unwrap();
            assert_eq!("{\"TupleStructValue\":{\"id\":0,\"name\":\"A\",\"inner_types\":[\"U8Value\",\"U16Value\",\"U32Value\"]}}", res);
        }
    }

    mod test_derive_array_struct {
        use super::*;

        #[allow(unused)]
        #[derive(serde_meta_derive::SerdeMeta)]
        struct A {
            f: [u8; 3],
        }

        #[test]
        fn test() {
            let meta = A::meta();
            if let TypeInformation::StructValue(named_type_info) = meta {
                assert_eq!("A", named_type_info.name());
                let fields = named_type_info.type_info().fields();
                assert_eq!(1, fields.len());
                assert_eq!("f", fields[0].name());
                if let TypeInformation::TupleValue(tuple_types) = fields[0].inner_type() {
                    let inner_types = tuple_types.inner_types();
                    assert_eq!(3, inner_types.len());
                    for t in inner_types.iter() {
                        assert_eq!(TypeInformation::U8Value(), t.inner_type());
                    }
                } else {
                    panic!("Expected TupleValue, but got {:#?}", fields[0].inner_type());
                }
            } else {
                panic!("Expected StructValue, but got {:#?}", meta);
            }
        }

        #[test]
        fn test_hash() {
            let meta1 = A::meta();
            let meta2 = A::meta();
            assert_eq!(calculate_hash(&meta1), calculate_hash(&meta2));
        }

        #[test]
        fn test_eq() {
            let meta1 = A::meta();
            let meta2 = A::meta();
            assert_eq!(meta1, meta2);
        }

        #[cfg(feature = "serde_ser")]
        #[test]
        fn test_serialization() {
            let meta = A::meta();
            let res = serde_json::to_string(&meta).unwrap();
            assert_eq!("{\"StructValue\":{\"id\":0,\"name\":\"A\",\"fields\":[{\"name\":\"f\",\"inner_type\":{\"SeqValue\":{\"inner_types\":[\"U8Value\",\"U8Value\",\"U8Value\"]}}}]}}", res);
        }
    }

    mod test_derive_array_ref {
        use super::*;

        #[allow(unused)]
        #[derive(serde_meta_derive::SerdeMeta)]
        struct A<'a> {
            f: &'a [u16],
        }

        #[test]
        fn test() {
            let meta = A::meta();
            if let TypeInformation::StructValue(named_type_info) = meta {
                assert_eq!("A", named_type_info.name());
                let fields = named_type_info.type_info().fields();
                assert_eq!(1, fields.len());
                assert_eq!("f", fields[0].name());
                if let TypeInformation::SeqValue(seq_type) = fields[0].inner_type() {
                    assert_eq!(TypeInformation::U16Value(), seq_type.inner_type());
                } else {
                    panic!("Expected SeqValue, but got {:#?}", fields[0].inner_type());
                }
            } else {
                panic!("Expected StructValue, but got {:#?}", meta);
            }
        }

        #[test]
        fn test_hash() {
            let meta1 = A::meta();
            let meta2 = A::meta();
            assert_eq!(calculate_hash(&meta1), calculate_hash(&meta2));
        }

        #[test]
        fn test_eq() {
            let meta1 = A::meta();
            let meta2 = A::meta();
            assert_eq!(meta1, meta2);
        }

        #[cfg(feature = "serde_ser")]
        #[test]
        fn test_serialization() {
            let meta = A::meta();
            let res = serde_json::to_string(&meta).unwrap();
            assert_eq!("{\"StructValue\":{\"id\":0,\"name\":\"A\",\"fields\":[{\"name\":\"f\",\"inner_type\":{\"SeqValue\":{\"inner_type\":\"U16Value\"}}}]}}", res);
        }
    }

    mod test_derive_enum {
        use super::*;

        #[allow(unused)]
        #[derive(serde_meta_derive::SerdeMeta)]
        enum A {
            IntVal(i32),
            StructVal { field: bool },
            UnitVal,
        }

        #[test]
        fn test() {
            let meta = A::meta();
            if let TypeInformation::EnumValue(named_type_info) = meta {
                let possible_variants = named_type_info.type_info().possible_variants();
                assert_eq!("A", named_type_info.name());
                assert_eq!(3, possible_variants.len());

                assert_eq!("IntVal", possible_variants[0].name());
                if let EnumVariantType::TupleVariant(types) = possible_variants[0].inner_type() {
                    let x = types.inner_types();
                    assert_eq!(1, x.len());
                    assert_eq!(TypeInformation::I32Value(), x[0].inner_type());
                } else {
                    panic!(
                        "Expected TupleVariant, but found {:#?}",
                        possible_variants[0].inner_type()
                    );
                }

                assert_eq!("StructVal", possible_variants[1].name());
                if let EnumVariantType::StructVariant(fields) = possible_variants[1].inner_type() {
                    let x = fields.fields();
                    assert_eq!(1, x.len());
                    assert_eq!(TypeInformation::BoolValue(), x[0].inner_type());
                    assert_eq!("field", x[0].name());
                } else {
                    panic!(
                        "Expected StructVariant, but found {:#?}",
                        possible_variants[0].inner_type()
                    );
                }

                assert_eq!("UnitVal", possible_variants[2].name());
                assert_eq!(
                    &EnumVariantType::UnitVariant(),
                    possible_variants[2].inner_type()
                )
            } else {
                panic!("Expected EnumValue but got {:#?}", meta);
            }
        }

        #[test]
        fn test_hash() {
            let meta1 = A::meta();
            let meta2 = A::meta();
            assert_eq!(calculate_hash(&meta1), calculate_hash(&meta2));
        }

        #[test]
        fn test_eq() {
            let meta1 = A::meta();
            let meta2 = A::meta();
            assert_eq!(meta1, meta2);
        }

        #[cfg(feature = "serde_ser")]
        #[test]
        fn test_serialization() {
            let meta = A::meta();
            let res = serde_json::to_string(&meta).unwrap();
            assert_eq!("{\"EnumValue\":{\"id\":0,\"name\":\"A\",\"possible_variants\":[{\"name\":\"IntVal\",\"inner_type\":{\"TupleVariant\":{\"fields\":[\"I32Value\"]}}},{\"name\":\"StructVal\",\"inner_type\":{\"TupleVariant\":{\"fields\":[{\"name\":\"field\",\"inner_type\":\"BoolValue\"}]}}},{\"name\":\"UnitVal\",\"inner_type\":\"UnitVariant\"}]}}", res);
        }
    }
}
