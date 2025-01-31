// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::common::CommonDateProvider;

use crate::error::Error;

use crate::cldr_serde;
use crate::CldrPaths;
use icu_datetime::pattern::CoarseHourCycle;
use icu_datetime::{pattern, provider::*};

use icu_provider::iter::{IterableDataProviderCore, KeyedDataProvider};
use icu_provider::prelude::*;
use std::convert::TryFrom;

/// All keys that this module is able to produce.
pub const ALL_KEYS: [ResourceKey; 1] = [
    key::DATE_PATTERNS_V1, //
];

/// A data provider reading from CLDR JSON dates files.
#[derive(PartialEq, Debug)]
pub struct DatePatternsProvider(CommonDateProvider);

impl TryFrom<&dyn CldrPaths> for DatePatternsProvider {
    type Error = Error;
    fn try_from(cldr_paths: &dyn CldrPaths) -> Result<Self, Self::Error> {
        CommonDateProvider::try_from(cldr_paths).map(DatePatternsProvider)
    }
}

impl KeyedDataProvider for DatePatternsProvider {
    fn supports_key(resc_key: &ResourceKey) -> Result<(), DataError> {
        key::DATE_PATTERNS_V1.match_key(*resc_key)
    }
}

impl DataProvider<calendar::DatePatternsV1Marker> for DatePatternsProvider {
    fn load_payload(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<calendar::DatePatternsV1Marker>, DataError> {
        DatePatternsProvider::supports_key(&req.resource_path.key)?;
        let dates = self.0.dates_for(req)?;
        let metadata = DataResponseMetadata::default();
        // TODO(#1109): Set metadata.data_langid correctly.
        Ok(DataResponse {
            metadata,
            payload: Some(DataPayload::from_owned(calendar::DatePatternsV1::from(
                dates,
            ))),
        })
    }
}

icu_provider::impl_dyn_provider!(DatePatternsProvider, {
    _ => calendar::DatePatternsV1Marker,
}, SERDE_SE);

impl IterableDataProviderCore for DatePatternsProvider {
    #[allow(clippy::needless_collect)] // https://github.com/rust-lang/rust-clippy/issues/7526
    fn supported_options_for_key(
        &self,
        resc_key: &ResourceKey,
    ) -> Result<Box<dyn Iterator<Item = ResourceOptions>>, DataError> {
        self.0.supported_options_for_key(resc_key)
    }
}

impl From<&cldr_serde::ca::LengthPatterns> for calendar::patterns::LengthPatternsV1<'_> {
    fn from(other: &cldr_serde::ca::LengthPatterns) -> Self {
        // TODO(#308): Support numbering system variations. We currently throw them away.
        Self {
            full: other
                .full
                .get_pattern()
                .parse()
                .expect("Failed to parse pattern"),
            long: other
                .long
                .get_pattern()
                .parse()
                .expect("Failed to parse pattern"),
            medium: other
                .medium
                .get_pattern()
                .parse()
                .expect("Failed to parse pattern"),
            short: other
                .short
                .get_pattern()
                .parse()
                .expect("Failed to parse pattern"),
        }
    }
}

impl From<&cldr_serde::ca::DateTimeFormats> for calendar::patterns::LengthPatternsV1<'_> {
    fn from(other: &cldr_serde::ca::DateTimeFormats) -> Self {
        // TODO(#308): Support numbering system variations. We currently throw them away.
        Self {
            full: other
                .full
                .get_pattern()
                .parse()
                .expect("Failed to parse pattern"),
            long: other
                .long
                .get_pattern()
                .parse()
                .expect("Failed to parse pattern"),
            medium: other
                .medium
                .get_pattern()
                .parse()
                .expect("Failed to parse pattern"),
            short: other
                .short
                .get_pattern()
                .parse()
                .expect("Failed to parse pattern"),
        }
    }
}

impl From<&cldr_serde::ca::DateTimeFormats> for calendar::patterns::GenericLengthPatternsV1<'_> {
    fn from(other: &cldr_serde::ca::DateTimeFormats) -> Self {
        // TODO(#308): Support numbering system variations. We currently throw them away.
        Self {
            full: other
                .full
                .get_pattern()
                .parse()
                .expect("Failed to parse pattern"),
            long: other
                .long
                .get_pattern()
                .parse()
                .expect("Failed to parse pattern"),
            medium: other
                .medium
                .get_pattern()
                .parse()
                .expect("Failed to parse pattern"),
            short: other
                .short
                .get_pattern()
                .parse()
                .expect("Failed to parse pattern"),
        }
    }
}

impl From<&cldr_serde::ca::Dates> for calendar::DatePatternsV1<'_> {
    fn from(other: &cldr_serde::ca::Dates) -> Self {
        let length_combinations_v1 =
            calendar::patterns::GenericLengthPatternsV1::from(&other.datetime_formats);
        let skeletons_v1 = calendar::DateSkeletonPatternsV1::from(&other.datetime_formats);

        let pattern_str_full = other.time_formats.full.get_pattern();
        let pattern_str_long = other.time_formats.long.get_pattern();
        let pattern_str_medium = other.time_formats.medium.get_pattern();
        let pattern_str_short = other.time_formats.short.get_pattern();

        let pattern_full = pattern_str_full
            .parse()
            .expect("Failed to create a full Pattern from bytes.");
        let pattern_long = pattern_str_long
            .parse()
            .expect("Failed to create a long Pattern from bytes.");
        let pattern_medium = pattern_str_medium
            .parse()
            .expect("Failed to create a medium Pattern from bytes.");
        let pattern_short = pattern_str_short
            .parse()
            .expect("Failed to create a short Pattern from bytes.");

        let mut preferred_hour_cycle: Option<CoarseHourCycle> = None;
        let arr = [
            pattern::CoarseHourCycle::determine(&pattern_full),
            pattern::CoarseHourCycle::determine(&pattern_long),
            pattern::CoarseHourCycle::determine(&pattern_medium),
            pattern::CoarseHourCycle::determine(&pattern_short),
        ];
        let iter = arr.iter().flatten();
        for hour_cycle in iter {
            if let Some(preferred_hour_cycle) = preferred_hour_cycle {
                assert_eq!(
                    *hour_cycle, preferred_hour_cycle,
                    "A locale contained a mix of coarse hour cycle types"
                );
            } else {
                preferred_hour_cycle = Some(*hour_cycle);
            }
        }

        let preferred_hour_cycle =
            preferred_hour_cycle.expect("Could not find a preferred hour cycle.");
        let alt_hour_cycle = if preferred_hour_cycle == CoarseHourCycle::H11H12 {
            CoarseHourCycle::H23H24
        } else {
            CoarseHourCycle::H11H12
        };

        let (time_h11_h12, time_h23_h24) = {
            let time = (&other.time_formats).into();
            let alt_time = calendar::patterns::LengthPatternsV1 {
                full: alt_hour_cycle
                    .apply_on_pattern(
                        &length_combinations_v1,
                        &skeletons_v1,
                        pattern_str_full,
                        pattern_full,
                    )
                    .as_ref()
                    .expect("Failed to apply a coarse hour cycle to a full pattern.")
                    .into(),
                long: alt_hour_cycle
                    .apply_on_pattern(
                        &length_combinations_v1,
                        &skeletons_v1,
                        pattern_str_long,
                        pattern_long,
                    )
                    .as_ref()
                    .expect("Failed to apply a coarse hour cycle to a long pattern.")
                    .into(),
                medium: alt_hour_cycle
                    .apply_on_pattern(
                        &length_combinations_v1,
                        &skeletons_v1,
                        pattern_str_medium,
                        pattern_medium,
                    )
                    .as_ref()
                    .expect("Failed to apply a coarse hour cycle to a medium pattern.")
                    .into(),
                short: alt_hour_cycle
                    .apply_on_pattern(
                        &length_combinations_v1,
                        &skeletons_v1,
                        pattern_str_short,
                        pattern_short,
                    )
                    .as_ref()
                    .expect("Failed to apply a coarse hour cycle to a short pattern.")
                    .into(),
            };

            match preferred_hour_cycle {
                CoarseHourCycle::H11H12 => (time, alt_time),
                CoarseHourCycle::H23H24 => (alt_time, time),
            }
        };

        Self {
            date: (&other.date_formats).into(),
            time_h11_h12,
            time_h23_h24,
            preferred_hour_cycle,
            length_combinations: length_combinations_v1,
        }
    }
}

#[test]
fn test_basic() {
    use icu_locid_macros::langid;

    let cldr_paths = crate::cldr_paths::for_test();
    let provider = DatePatternsProvider::try_from(&cldr_paths as &dyn CldrPaths)
        .expect("Failed to retrieve provider");

    let cs_dates: DataPayload<calendar::DatePatternsV1Marker> = provider
        .load_payload(&DataRequest {
            resource_path: ResourcePath {
                key: key::DATE_PATTERNS_V1,
                options: ResourceOptions {
                    variant: Some("gregory".into()),
                    langid: Some(langid!("cs")),
                },
            },
        })
        .expect("Failed to load payload")
        .take_payload()
        .expect("Failed to retrieve payload");

    assert_eq!("d. M. y", cs_dates.get().date.medium.to_string());
}

#[test]
fn test_with_numbering_system() {
    use icu_locid_macros::langid;

    let cldr_paths = crate::cldr_paths::for_test();
    let provider = DatePatternsProvider::try_from(&cldr_paths as &dyn CldrPaths)
        .expect("Failed to retrieve provider");

    let cs_dates: DataPayload<calendar::DatePatternsV1Marker> = provider
        .load_payload(&DataRequest {
            resource_path: ResourcePath {
                key: key::DATE_PATTERNS_V1,
                options: ResourceOptions {
                    variant: Some("gregory".into()),
                    langid: Some(langid!("haw")),
                },
            },
        })
        .expect("Failed to load payload")
        .take_payload()
        .expect("Failed to retrieve payload");

    assert_eq!("d MMM y", cs_dates.get().date.medium.to_string());
    // TODO(#308): Support numbering system variations. We currently throw them away.
    assert_eq!("d/M/yy", cs_dates.get().date.short.to_string());
}
