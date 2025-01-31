// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![cfg(all(test, feature = "provider_serde"))]

use icu_datetime::skeleton::reference::Skeleton;
use std::{fs::File, io::BufReader};

/// Note that this file tests only valid skeleton cases for the stability of the serialization
/// pipeline. For tests failure cases see the file where skeletons are defined.

#[derive(serde::Serialize, serde::Deserialize)]
struct SkeletonFixtures {
    pub skeletons: Vec<String>,
}

fn get_skeleton_fixtures() -> Vec<String> {
    let file = File::open("./tests/fixtures/tests/skeletons.json".to_string())
        .expect("Unable to open ./tests/fixtures/tests/skeletons.json");
    let reader = BufReader::new(file);
    let fixtures: SkeletonFixtures =
        serde_json::from_reader(reader).expect("Unable to deserialize skeleton fixtures.");
    fixtures.skeletons
}

fn get_skeleton_bincode_write_handle() -> File {
    File::create("./tests/fixtures/tests/skeletons.bin")
        .expect("Unable to create ./tests/fixtures/tests/skeletons.bin")
}

fn get_skeleton_bincode_from_file() -> Vec<Vec<u8>> {
    bincode::deserialize_from(
        File::open("./tests/fixtures/tests/skeletons.bin")
            .expect("Unable to ./tests/fixtures/tests/skeletons.bin"),
    )
    .expect("Unable to deserialize bytes.")
}

#[test]
fn test_skeleton_json_serialization_roundtrip() {
    for skeleton_string in &get_skeleton_fixtures() {
        // Wrap the string in quotes so it's a JSON string.
        let json_in: String = serde_json::to_string(skeleton_string).unwrap();

        let skeleton: Skeleton = match serde_json::from_str(&json_in) {
            Ok(p) => p,
            Err(err) => {
                panic!(
                    "Unable to parse the skeleton {:?}. {:?}",
                    skeleton_string, err
                );
            }
        };

        let json_out = match serde_json::to_string(&skeleton) {
            Ok(s) => s,
            Err(err) => {
                panic!(
                    "Unable to re-serialize the skeleton {:?}. {:?}",
                    skeleton_string, err
                );
            }
        };

        assert_eq!(
            json_in, json_out,
            "The roundtrip serialization for the skeleton matched."
        );
    }
}

/// Bincode representation of skeletons need to be stable across time. This test checks the
/// current serialization against historic serialization to ensure that this remains stable.
#[test]
fn test_skeleton_bincode_serialization_roundtrip() {
    let skeletons = get_skeleton_fixtures();
    let update_bincode = std::env::var_os("ICU4X_REGEN_FIXTURE").is_some();
    let mut result_vec = Vec::new();
    let expect_vec = if update_bincode {
        None
    } else {
        Some(get_skeleton_bincode_from_file())
    };

    if let Some(ref expect_vec) = expect_vec {
        if expect_vec.len() != skeletons.len() {
            panic!(
                "Expected the bincode to have the same number of entries as the string skeletons. \
                 The bincode can be re-generated by re-running the test with the environment
                 variable ICU4X_REGEN_FIXTURE set."
            );
        }
    }

    for (i, skeleton_string) in skeletons.iter().enumerate() {
        // Wrap the string in quotes so it's a JSON string.
        let json_in: String = serde_json::to_string(skeleton_string).unwrap();

        let skeleton: Skeleton = match serde_json::from_str(&json_in) {
            Ok(p) => p,
            Err(err) => {
                panic!(
                    "Unable to parse the skeleton {:?}. {:?}",
                    skeleton_string, err
                );
            }
        };

        let bincode: Vec<u8> = bincode::serialize(&skeleton).unwrap();

        if let Some(ref expect_vec) = expect_vec {
            if bincode != *expect_vec.get(i).unwrap() {
                panic!(
                    "The bincode representations of the skeleton {:?} did not match the stored \
                     representation. Skeletons are supposed to have stable bincode representations. \
                     Something changed to make it different than what it was in the past. If this is \
                     expected, then the bincode can be updated by re-running the test with the \
                     environment variable ICU4X_REGEN_FIXTURE set.",
                    json_in
                )
            }
        }
        result_vec.push(bincode);
    }
    if update_bincode {
        eprintln!("Writing the bincode into a file");
        bincode::serialize_into(&mut get_skeleton_bincode_write_handle(), &result_vec).unwrap();
    }
}
