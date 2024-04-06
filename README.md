<!--
SPDX-FileCopyrightText: 2024 Robin Vobruba <hoijui.quaero@gmail.com>

SPDX-License-Identifier: CC0-1.0
-->

# `owl2shacl`

[![License: AGPL-3.0-or-later](
    https://img.shields.io/badge/License-AGPL--3.0--or--later-blue.svg)](
    LICENSE.txt)
[![REUSE status](
    https://api.reuse.software/badge/github.com/hoijui/owl2shacl)](
    https://api.reuse.software/info/github.com/hoijui/owl2shacl)
[![Repo](
    https://img.shields.io/badge/Repo-GitHub-555555&logo=github.svg)](
    https://github.com/hoijui/owl2shacl)
[![Package Releases](
    https://img.shields.io/crates/v/owl2shacl.svg)](
    https://crates.io/crates/owl2shacl)
[![Documentation Releases](
    https://docs.rs/owl2shacl/badge.svg)](
    https://docs.rs/owl2shacl)
[![Dependency Status](
    https://deps.rs/repo/github/hoijui/owl2shacl/status.svg)](
    https://deps.rs/repo/github/hoijui/owl2shacl)
[![Build Status](
    https://github.com/hoijui/owl2shacl/workflows/build/badge.svg)](
    https://github.com/hoijui/owl2shacl/actions)

[![In cooperation with Open Source Ecology Germany](
    https://raw.githubusercontent.com/osegermany/tiny-files/master/res/media/img/badge-oseg.svg)](
    https://opensourceecology.de)

A [CLI] tool that tries to convert simple [OWL] ontologies into [SHACL] shapes.
OWL ontologies define logical relationships.
SHACL shapes define a data scheme, and allow to validate data against them.
Striclty speaking, as these are different things,
such a conversion is thus illegal/wrong in the ideological/theoretical sense.
Thus this tool is not generally applicable, but only under the sircumstance,
that the OWL ontolog is actually written as a data specification -
if it is understood as a kind of distributed database schema,
rather then for logical inference.
Not only that, but it also has to conform to certain, very narrow rules,
and only a few basic properties are translated into SHACL;
the rest is ignored.

## Supported OWL ontologies

Roughtly speaking,
this tool supports a 25% subset of [OWL Lite](
https://www.w3.org/TR/2004/REC-owl-features-20040210/#s2.1),
plus some third party properties.

More specifically:

- OWL Lite
  - [x] [Class (Thing, Nothing)](https://www.w3.org/TR/2004/REC-owl-features-20040210/#Class)
  - [x] [rdfs:subClassOf](https://www.w3.org/TR/2004/REC-owl-features-20040210/#subClassOf)
  - [ ] [rdf:Property](https://www.w3.org/TR/2004/REC-owl-features-20040210/#property)
  - [ ] [rdfs:subPropertyOf](https://www.w3.org/TR/2004/REC-owl-features-20040210/#subPropertyOf)
  - [x] [rdfs:domain](https://www.w3.org/TR/2004/REC-owl-features-20040210/#domain)
  - [x] [rdfs:range](https://www.w3.org/TR/2004/REC-owl-features-20040210/#range)
  - [ ] [Individual](https://www.w3.org/TR/2004/REC-owl-features-20040210/#Individual)
  - [ ] [equivalentClass](https://www.w3.org/TR/2004/REC-owl-features-20040210/#equivalentClass)
  - [ ] [equivalentProperty](https://www.w3.org/TR/2004/REC-owl-features-20040210/#equivalentProperty)
  - [ ] [sameAs](https://www.w3.org/TR/2004/REC-owl-features-20040210/#sameAs)
  - [ ] [differentFrom](https://www.w3.org/TR/2004/REC-owl-features-20040210/#differentFrom)
  - [ ] [AllDifferent](https://www.w3.org/TR/2004/REC-owl-features-20040210/#AllDifferent)
  - [ ] [distinctMembers](https://www.w3.org/TR/2004/REC-owl-features-20040210/#AllDifferent)
  - [x] [ObjectProperty](https://www.w3.org/TR/2004/REC-owl-features-20040210/#property)
  - [x] [DatatypeProperty](https://www.w3.org/TR/2004/REC-owl-features-20040210/#property)
  - [ ] [inverseOf](https://www.w3.org/TR/2004/REC-owl-features-20040210/#inverseOf)
  - [ ] [TransitiveProperty](https://www.w3.org/TR/2004/REC-owl-features-20040210/#TransitiveProperty)
  - [ ] [SymmetricProperty](https://www.w3.org/TR/2004/REC-owl-features-20040210/#SymmetricProperty)
  - [ ] [FunctionalProperty](https://www.w3.org/TR/2004/REC-owl-features-20040210/#FunctionalProperty)
  - [ ] [InverseFunctionalProperty](https://www.w3.org/TR/2004/REC-owl-features-20040210/#InverseFunctionalProperty)
  - [ ] [Restriction](https://www.w3.org/TR/2004/REC-owl-features-20040210/#s3.4)
  - [ ] [onProperty](https://www.w3.org/TR/2004/REC-owl-features-20040210/#s3.4)
  - [ ] [allValuesFrom](https://www.w3.org/TR/2004/REC-owl-features-20040210/#allValuesFrom)
  - [ ] [someValuesFrom](https://www.w3.org/TR/2004/REC-owl-features-20040210/#someValuesFrom)
  - [x] [minCardinality](https://www.w3.org/TR/2004/REC-owl-features-20040210/#minCardinality)
  - [x] [maxCardinality](https://www.w3.org/TR/2004/REC-owl-features-20040210/#maxCardinality)
  - [x] [cardinality](https://www.w3.org/TR/2004/REC-owl-features-20040210/#Cardinality)
  - [ ] [Ontology](https://www.w3.org/TR/2004/REC-owl-features-20040210/#s3.8)
  - [ ] [imports](https://www.w3.org/TR/2004/REC-owl-features-20040210/#s3.8)
  - [ ] [intersectionOf](https://www.w3.org/TR/2004/REC-owl-features-20040210/#intersectionOf)
  - [ ] [versionInfo](https://www.w3.org/TR/2004/REC-owl-features-20040210/#s3.10)
  - [ ] [priorVersion](https://www.w3.org/TR/2004/REC-owl-features-20040210/#s3.10)
  - [ ] [backwardCompatibleWith](https://www.w3.org/TR/2004/REC-owl-features-20040210/#s3.10)
  - [ ] [incompatibleWith](https://www.w3.org/TR/2004/REC-owl-features-20040210/#s3.10)
  - [ ] [DeprecatedClass](https://www.w3.org/TR/2004/REC-owl-features-20040210/#s3.10)
  - [ ] [DeprecatedProperty](https://www.w3.org/TR/2004/REC-owl-features-20040210/#s3.10)
  - [x] [rdfs:label](https://www.w3.org/TR/2004/REC-owl-features-20040210/#s3.9)
  - [x] [rdfs:comment](https://www.w3.org/TR/2004/REC-owl-features-20040210/#s3.9)
  - [ ] [rdfs:seeAlso](https://www.w3.org/TR/2004/REC-owl-features-20040210/#s3.9)
  - [ ] [rdfs:isDefinedBy](https://www.w3.org/TR/2004/REC-owl-features-20040210/#s3.9)
  - [ ] [AnnotationProperty](https://www.w3.org/TR/2004/REC-owl-features-20040210/#s3.9)
  - [ ] [OntologyProperty](https://www.w3.org/TR/2004/REC-owl-features-20040210/#s3.9)
  - [ ] [xsd datatypes](https://www.w3.org/TR/2004/REC-owl-features-20040210/#s3.7)
- others
  - [x] domainIncludes
    - [x] [schema:domainIncludes](http://schema.org/domainIncludes)
    - [x] [dcam:domainIncludes](http://purl.org/dc/dcam/domainIncludes)
    - [x] [dcid:domainIncludes](https://datacommons.org/browser/domainIncludes)
    - [x] rdfs:domain + [owl:unionOf](http://www.w3.org/2002/07/owl#unionOf)
  - [x] rangeIncludes
    - [x] [schema:rangeIncludes](http://schema.org/rangeIncludes)
    - [x] [dcam:rangeIncludes](http://purl.org/dc/dcam/rangeIncludes)
    - [x] [dcid:rangeIncludes](https://datacommons.org/browser/rangeIncludes)
    - [x] rdfs:range + [owl:unionOf](http://www.w3.org/2002/07/owl#unionOf)

## How to compile

You need to install Rust(lang) and Cargo.

Then get the whole repo plus git sub-modules with:

```bash
git clone --recurse-submodules https://github.com/hoijui/owl2shacl.git
cd owl2shacl
```

Then you can run:

```bash
cargo build --release
```

If all goes well, the executable can be found at `target/release/owl2shacl`.

## Get the tool

As for now, you have two choices:

1. [Compile it](#how-to-compile) yourself
1. Download a Linux x86\_64 statically linked binary from
   [the releases page](https://github.com/hoijui/owl2shacl/releases)

[CLI]: https://en.wikipedia.org/wiki/Command-line_interface
[OWL]: 
[SHACL]: 

