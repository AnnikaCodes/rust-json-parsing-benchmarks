#![feature(test)]

#[cfg(test)]
mod tests {
    extern crate test;
    use lazy_static::lazy_static;
    use std::{fs, path::Path};
    use test::Bencher;

    lazy_static! {
        static ref SMALL_JSON: String = fs::read_to_string(Path::new("json/small.json")).expect(
            "Error loading 'json/small.json'; make sure you are operating from the root directory"
        );
        static ref LARGE_JSON: String = fs::read_to_string(Path::new("json/large.json")).expect(
            "Error loading 'json/large.json'; make sure you are operating from the root directory"
        );
    }

    // Serde JSON
    #[bench]
    fn serde_small_top_level(b: &mut Bencher) {
        b.iter(|| {
            let parsed: serde_json::Value = serde_json::from_str(&SMALL_JSON).unwrap();
            assert_eq!(parsed["topLevelProperty"], 1);
        });
    }
    #[bench]
    fn serde_large_top_level(b: &mut Bencher) {
        b.iter(|| {
            let parsed: serde_json::Value = serde_json::from_str(&LARGE_JSON).unwrap();
            assert_eq!(parsed["topLevelProperty"], 1);
        });
    }
    #[bench]
    fn serde_small_fourth_level(b: &mut Bencher) {
        b.iter(|| {
            let parsed: serde_json::Value = serde_json::from_str(&SMALL_JSON).unwrap();
            assert_eq!(
                parsed["property"]["subProperty"]["thirdLevel"]["pi"],
                3.14159
            );
        });
    }
    #[bench]
    fn serde_large_fourth_level(b: &mut Bencher) {
        b.iter(|| {
            let parsed: serde_json::Value = serde_json::from_str(&LARGE_JSON).unwrap();
            assert_eq!(
                parsed["property"]["subProperty"]["thirdLevel"]["pi"],
                3.14159
            );
        });
    }
    #[bench]
    fn serde_small_parse_all(b: &mut Bencher) {
        b.iter(|| {
            serde_json::from_str::<serde_json::Value>(&SMALL_JSON).unwrap();
        });
    }
    #[bench]
    fn serde_large_parse_all(b: &mut Bencher) {
        b.iter(|| {
            serde_json::from_str::<serde_json::Value>(&LARGE_JSON).unwrap();
        });
    }

    // SIMD JSON
    #[bench]
    fn simd_small_top_level(b: &mut Bencher) {
        b.iter(|| {
            let parsed: simd_json::OwnedValue =
                simd_json::serde::from_str(&mut SMALL_JSON.clone()).unwrap();
            assert_eq!(parsed["topLevelProperty"], 1);
        });
    }
    #[bench]
    fn simd_large_top_level(b: &mut Bencher) {
        b.iter(|| {
            let parsed: simd_json::OwnedValue =
                simd_json::serde::from_str(&mut LARGE_JSON.clone()).unwrap();
            assert_eq!(parsed["topLevelProperty"], 1);
        });
    }
    #[bench]
    fn simd_small_fourth_level(b: &mut Bencher) {
        b.iter(|| {
            let parsed: serde_json::Value =
                simd_json::serde::from_str(&mut SMALL_JSON.clone()).unwrap();
            // simd-json introduces floating-point precision problems, for some reason
            assert_eq!(
                parsed["property"]["subProperty"]["thirdLevel"]["pi"],
                3.1415900000000003
            );
        });
    }
    #[bench]
    fn simd_large_fourth_level(b: &mut Bencher) {
        b.iter(|| {
            let parsed: serde_json::Value =
                simd_json::serde::from_str(&mut LARGE_JSON.clone()).unwrap();
            assert_eq!(
                parsed["property"]["subProperty"]["thirdLevel"]["pi"],
                3.1415900000000003
            );
        });
    }
    #[bench]
    fn simd_small_parse_all(b: &mut Bencher) {
        b.iter(|| {
            simd_json::serde::from_str::<serde_json::Value>(&mut SMALL_JSON.clone()).unwrap();
        });
    }
    #[bench]
    fn simd_large_parse_all(b: &mut Bencher) {
        b.iter(|| {
            simd_json::serde::from_str::<serde_json::Value>(&mut LARGE_JSON.clone()).unwrap();
        });
    }

    // GJSON
    #[bench]
    fn gjson_small_top_level(b: &mut Bencher) {
        b.iter(|| {
            let parsed: gjson::Value = gjson::get(&SMALL_JSON, "topLevelProperty");
            assert_eq!(parsed.i32(), 1);
        });
    }
    #[bench]
    fn gjson_large_top_level(b: &mut Bencher) {
        b.iter(|| {
            let parsed: gjson::Value = gjson::get(&LARGE_JSON, "topLevelProperty");
            assert_eq!(parsed.i32(), 1);
        });
    }
    #[bench]
    fn gjson_small_fourth_level(b: &mut Bencher) {
        b.iter(|| {
            let parsed: gjson::Value =
                gjson::get(&SMALL_JSON, "property.subProperty.thirdLevel.pi");
            assert_eq!(parsed.f32(), 3.14159);
        });
    }
    #[bench]
    fn gjson_large_fourth_level(b: &mut Bencher) {
        b.iter(|| {
            let parsed: gjson::Value =
                gjson::get(&LARGE_JSON, "property.subProperty.thirdLevel.pi");
            assert_eq!(parsed.f32(), 3.14159);
        });
    }
    #[bench]
    #[ignore = "GJSON's parse() method delays all parsing until a property is accessed, so benchmarking is not feasible"]
    fn gjson_small_parse_all(b: &mut Bencher) {
        b.iter(|| {
            gjson::parse(&SMALL_JSON);
        });
    }
    #[bench]
    #[ignore = "GJSON's parse() method delays all parsing until a property is accessed, so benchmarking is not feasible"]
    fn gjson_large_parse_all(b: &mut Bencher) {
        b.iter(|| {
            gjson::parse(&LARGE_JSON);
        });
    }

    // A-JSON
    #[bench]
    fn ajson_small_top_level(b: &mut Bencher) {
        b.iter(|| {
            let parsed: ajson::Value = ajson::get(&SMALL_JSON, "topLevelProperty").unwrap();
            assert_eq!(parsed.to_i64(), 1);
        });
    }
    #[bench]
    fn ajson_large_top_level(b: &mut Bencher) {
        b.iter(|| {
            let parsed: ajson::Value = ajson::get(&LARGE_JSON, "topLevelProperty").unwrap();
            assert_eq!(parsed.to_i64(), 1);
        });
    }
    #[bench]
    fn ajson_small_fourth_level(b: &mut Bencher) {
        b.iter(|| {
            let parsed: ajson::Value =
                ajson::get(&SMALL_JSON, "property.subProperty.thirdLevel.pi").unwrap();
            assert_eq!(parsed.to_f64(), 3.14159);
        });
    }
    #[bench]
    fn ajson_large_fourth_level(b: &mut Bencher) {
        b.iter(|| {
            let parsed: ajson::Value =
                ajson::get(&LARGE_JSON, "property.subProperty.thirdLevel.pi").unwrap();
            assert_eq!(parsed.to_f64(), 3.14159);
        });
    }
    #[bench]
    #[ignore = "AJSON's parse() method delays about half of the time spent parsing until a property is accessed, so benchmarking is not feasible"]
    fn ajson_small_parse_all(b: &mut Bencher) {
        b.iter(|| {
            ajson::parse(&SMALL_JSON).unwrap();
        });
    }
    #[bench]
    #[ignore = "AJSON's parse() method delays about half of the time spent parsing until a property is accessed, so benchmarking is not feasible"]
    fn ajson_large_parse_all(b: &mut Bencher) {
        b.iter(|| {
            ajson::parse(&LARGE_JSON).unwrap();
        });
    }

    // json-rust
    #[bench]
    fn json_rust_small_top_level(b: &mut Bencher) {
        b.iter(|| {
            let parsed = json::parse(&SMALL_JSON).unwrap();
            assert_eq!(parsed["topLevelProperty"], 1);
        });
    }
    #[bench]
    fn json_rust_large_top_level(b: &mut Bencher) {
        b.iter(|| {
            let parsed = json::parse(&LARGE_JSON).unwrap();
            assert_eq!(parsed["topLevelProperty"], 1);
        });
    }
    #[bench]
    fn json_rust_small_fourth_level(b: &mut Bencher) {
        b.iter(|| {
            let parsed = json::parse(&SMALL_JSON).unwrap();
            assert_eq!(
                parsed["property"]["subProperty"]["thirdLevel"]["pi"],
                3.14159
            );
        });
    }
    #[bench]
    fn json_rust_large_fourth_level(b: &mut Bencher) {
        b.iter(|| {
            let parsed = json::parse(&LARGE_JSON).unwrap();
            assert_eq!(
                parsed["property"]["subProperty"]["thirdLevel"]["pi"],
                3.14159
            );
        });
    }
    #[bench]
    fn json_rust_small_parse_all(b: &mut Bencher) {
        b.iter(|| {
            json::parse(&SMALL_JSON).unwrap();
        });
    }
    #[bench]
    fn json_rust_large_parse_all(b: &mut Bencher) {
        b.iter(|| {
            json::parse(&LARGE_JSON).unwrap();
        });
    }

    // Pikkr
    #[bench]
    fn pikkr_stateful_small_top_level(b: &mut Bencher) {
        let mut parser =
            pikkr_annika::Pikkr::new(&vec!["$.topLevelProperty".as_bytes()], 2).unwrap();
        b.iter(|| {
            // Pikkr has a rather low-level API
            // Maybe someday I'll write a wrapper for it...
            let parsed: u32 = String::from_utf8(
                parser
                    .parse(SMALL_JSON.as_bytes())
                    .unwrap()
                    .get(0)
                    .unwrap()
                    .unwrap()
                    .to_vec(),
            )
            .unwrap()
            .parse()
            .unwrap();
            assert_eq!(parsed, 1);
        });
    }

    #[bench]
    fn pikkr_stateful_large_top_level(b: &mut Bencher) {
        let mut parser =
            pikkr_annika::Pikkr::new(&vec!["$.topLevelProperty".as_bytes()], 2).unwrap();
        b.iter(|| {
            let parsed: u32 = String::from_utf8(
                parser
                    .parse(LARGE_JSON.as_bytes())
                    .unwrap()
                    .get(0)
                    .unwrap()
                    .unwrap()
                    .to_vec(),
            )
            .unwrap()
            .parse()
            .unwrap();
            assert_eq!(parsed, 1);
        });
    }

    #[bench]
    fn pikkr_stateless_small_top_level(b: &mut Bencher) {
        b.iter(|| {
            let mut parser =
                pikkr_annika::Pikkr::new(&vec!["$.topLevelProperty".as_bytes()], 2).unwrap();
            let parsed: u32 = String::from_utf8(
                parser
                    .parse(SMALL_JSON.as_bytes())
                    .unwrap()
                    .get(0)
                    .unwrap()
                    .unwrap()
                    .to_vec(),
            )
            .unwrap()
            .parse()
            .unwrap();
            assert_eq!(parsed, 1);
        });
    }

    #[bench]
    fn pikkr_stateless_large_top_level(b: &mut Bencher) {
        b.iter(|| {
            let mut parser =
                pikkr_annika::Pikkr::new(&vec!["$.topLevelProperty".as_bytes()], 2).unwrap();
            let parsed: u32 = String::from_utf8(
                parser
                    .parse(LARGE_JSON.as_bytes())
                    .unwrap()
                    .get(0)
                    .unwrap()
                    .unwrap()
                    .to_vec(),
            )
            .unwrap()
            .parse()
            .unwrap();
            assert_eq!(parsed, 1);
        });
    }
    #[bench]
    fn pikkr_stateful_small_fourth_level(b: &mut Bencher) {
        let mut parser =
            pikkr_annika::Pikkr::new(&vec!["$.property.subProperty.thirdLevel.pi".as_bytes()], 2)
                .unwrap();
        b.iter(|| {
            // Pikkr has a rather low-level API
            // Maybe someday I'll write a wrapper for it...
            let parsed: f32 = String::from_utf8(
                parser
                    .parse(SMALL_JSON.as_bytes())
                    .unwrap()
                    .get(0)
                    .unwrap()
                    .unwrap()
                    .to_vec(),
            )
            .unwrap()
            .parse()
            .unwrap();
            assert_eq!(parsed, 3.14159);
        });
    }

    #[bench]
    fn pikkr_stateful_large_fourth_level(b: &mut Bencher) {
        let mut parser =
            pikkr_annika::Pikkr::new(&vec!["$.property.subProperty.thirdLevel.pi".as_bytes()], 2)
                .unwrap();
        b.iter(|| {
            let parsed: f32 = String::from_utf8(
                parser
                    .parse(LARGE_JSON.as_bytes())
                    .unwrap()
                    .get(0)
                    .unwrap()
                    .unwrap()
                    .to_vec(),
            )
            .unwrap()
            .parse()
            .unwrap();
            assert_eq!(parsed, 3.14159);
        });
    }

    #[bench]
    fn pikkr_stateless_small_fourth_level(b: &mut Bencher) {
        b.iter(|| {
            let mut parser = pikkr_annika::Pikkr::new(
                &vec!["$.property.subProperty.thirdLevel.pi".as_bytes()],
                2,
            )
            .unwrap();
            let parsed: f32 = String::from_utf8(
                parser
                    .parse(SMALL_JSON.as_bytes())
                    .unwrap()
                    .get(0)
                    .unwrap()
                    .unwrap()
                    .to_vec(),
            )
            .unwrap()
            .parse()
            .unwrap();
            assert_eq!(parsed, 3.14159);
        });
    }

    #[bench]
    fn pikkr_stateless_large_fourth_level(b: &mut Bencher) {
        b.iter(|| {
            let mut parser = pikkr_annika::Pikkr::new(
                &vec!["$.property.subProperty.thirdLevel.pi".as_bytes()],
                2,
            )
            .unwrap();
            let parsed: f32 = String::from_utf8_lossy(
                parser
                    .parse(LARGE_JSON.as_bytes())
                    .unwrap()
                    .get(0)
                    .unwrap()
                    .unwrap(),
            )
            .parse()
            .unwrap();
            assert_eq!(parsed, 3.14159);
        });
    }

    #[bench]
    #[ignore = "Pikkr does not support parsing an entire JSON file at once"]
    fn pikkr_stateful_small_parse_all(_: &mut Bencher) {}
    #[bench]
    #[ignore = "Pikkr does not support parsing an entire JSON file at once"]
    fn pikkr_stateful_large_parse_all(_: &mut Bencher) {}
    #[bench]
    #[ignore = "Pikkr does not support parsing an entire JSON file at once"]
    fn pikkr_stateless_small_parse_all(_: &mut Bencher) {}
    #[bench]
    #[ignore = "Pikkr does not support parsing an entire JSON file at once"]
    fn pikkr_stateless_large_parse_all(_: &mut Bencher) {}

    // tinyjson
    #[bench]
    fn tinyjson_small_top_level(b: &mut Bencher) {
        b.iter(|| {
            let parsed: tinyjson::JsonValue = SMALL_JSON.parse().unwrap();
            assert_eq!(*parsed["topLevelProperty"].get::<f64>().unwrap(), 1.0);
        });
    }
    #[bench]
    fn tinyjson_large_top_level(b: &mut Bencher) {
        b.iter(|| {
            let parsed: tinyjson::JsonValue = LARGE_JSON.parse().unwrap();
            assert_eq!(*parsed["topLevelProperty"].get::<f64>().unwrap(), 1.0);
        });
    }
    #[bench]
    fn tinyjson_small_fourth_level(b: &mut Bencher) {
        b.iter(|| {
            let parsed: tinyjson::JsonValue = SMALL_JSON.parse().unwrap();
            assert_eq!(
                *parsed["property"]["subProperty"]["thirdLevel"]["pi"]
                    .get::<f64>()
                    .unwrap(),
                3.14159
            );
        });
    }
    #[bench]
    fn tinyjson_large_fourth_level(b: &mut Bencher) {
        b.iter(|| {
            let parsed: tinyjson::JsonValue = LARGE_JSON.parse().unwrap();
            assert_eq!(
                *parsed["property"]["subProperty"]["thirdLevel"]["pi"]
                    .get::<f64>()
                    .unwrap(),
                3.14159
            );
        });
    }
    #[bench]
    fn tinyjson_small_parse_all(b: &mut Bencher) {
        b.iter(|| {
            SMALL_JSON.parse::<tinyjson::JsonValue>().unwrap();
        });
    }
    #[bench]
    fn tinyjson_large_parse_all(b: &mut Bencher) {
        b.iter(|| {
            LARGE_JSON.parse::<tinyjson::JsonValue>().unwrap();
        });
    }
}
