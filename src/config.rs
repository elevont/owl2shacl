// SPDX-FileCopyrightText: 2024 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use clap::ValueEnum;
use enum_map::{Enum, EnumMap};
use strum_macros::{EnumIter, EnumString, VariantNames, IntoStaticStr};

/**
 * How to behave in case an odity is detected
 * in the source Ontology.
 */
#[derive(
    Debug,
    ValueEnum,
    EnumString,
    VariantNames,
    EnumIter,
    IntoStaticStr,
    PartialEq,
    Eq,
    PartialOrd,
    Copy,
    Clone,
)]
pub enum OdityHandling {
    Ignore,
    Warn,
    Error,
}

impl Default for OdityHandling {
    fn default() -> Self {
        Self::Warn
    }
}

impl OdityHandling {
    pub const fn ignore(self) -> bool {
        matches!(self, Self::Ignore)
    }
}

#[derive(Clone, Copy, Debug, Enum, EnumIter)]
pub enum RDProperty {
    Range,
    Domain,
}

impl RDProperty {
    pub const fn to_str(self) -> &'static str {
        match self {
            Self::Range => "range",
            Self::Domain => "domain",
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Config /*<S: ::std::hash::BuildHasher>*/ {
    /**
     * What to do if the source Ontology contains properties
     * with `rdfs:range`/`rdfs:domain` that specifies a list/array of classes,
     * which means that possible objects have to implement
     * _all_ of these classes,
     * which is often not what was intended.
     */
    pub and_list_detected: EnumMap<RDProperty, OdityHandling>,
    /**
     * What to do if the source Ontology contains properties
     * using both `rdfs:range` and `*:rangeIncludes`,
     * or respectively `rdfs:domain` and `*:domainIncludes`,
     * which is somewhat ill-defined.
     */
    pub style_mix_property: EnumMap<RDProperty, OdityHandling>,
    /**
     * What to do if the source Ontology contains both properties
     * using `rdfs:range` and others using `*:rangeIncludes`,
     * or respectively `rdfs:domain` and others using `*:domainIncludes`,
     * which is technically ok, but might be confusing.
     */
    pub style_mix_ontology: EnumMap<RDProperty, OdityHandling>,
}
