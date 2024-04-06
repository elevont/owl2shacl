// SPDX-FileCopyrightText: 2023 - 2024 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::collections::HashSet;
use std::fs::File;

use enum_map::EnumMap;
use once_cell::sync::Lazy;
use oxigraph::io::GraphFormat;
use oxigraph::model::vocab::rdf;

use const_format::concatcp;
use oxigraph::model::{GraphName, GraphNameRef, NamedNodeRef, Quad, Term};
use oxigraph::sparql::Query;
use oxigraph::sparql::QueryResults;
use oxigraph::sparql::QuerySolution;
use oxigraph::sparql::Variable;
use oxigraph::store::Store;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::config::Config;
use crate::config::OdityHandling;
use crate::config::RDProperty;
use crate::vocab::{basics, owl, sh};

type Error = Box<dyn std::error::Error + Sync + Send>;
type Res<O> = Result<O, Error>;

const QUERY_PRELUDE: &str = r"
#@base           <https://w3id.org/valueflows>
#PREFIX vf:      <#>
PREFIX vf:      <https://w3id.org/valueflows#>
PREFIX vfs:     <https://w3id.org/valueflows/shapes#>
PREFIX rdf:     <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
PREFIX owl:     <http://www.w3.org/2002/07/owl#>
PREFIX rdfs:    <http://www.w3.org/2000/01/rdf-schema#>
PREFIX foaf:    <http://xmlns.com/foaf/0.1/>
PREFIX org:     <http://www.w3.org/ns/org#>
PREFIX schema:  <http://schema.org/>
PREFIX dcterms: <http://purl.org/dc/terms/>
PREFIX dcam:    <http://purl.org/dc/dcam/>
PREFIX dcid:    <https://datacommons.org/browser/>
PREFIX om2:     <http://www.ontology-of-units-of-measure.org/resource/om-2/>
PREFIX xsd:     <http://www.w3.org/2001/XMLSchema#>
PREFIX time:    <http://www.w3.org/2006/time#>
PREFIX geo:     <http://www.w3.org/2003/01/geo/wgs84_pos#>
PREFIX vs:      <http://www.w3.org/2003/06/sw-vocab-status/ns#>
PREFIX dtype:   <http://www.linkedmodel.org/schema/dtype#>
PREFIX dfc:     <http://www.virtual-assembly.org/DataFoodConsortium/BusinessOntology#>
PREFIX skos:    <http://www.w3.org/2004/02/skos/core#>
PREFIX sh:      <http://www.w3.org/ns/shacl#>

";

macro_rules! query_parser {
    ($const:ident, $query_str:ident) => {
        pub static $const: Lazy<Query> = Lazy::new(|| {
            let query_str = concatcp!(QUERY_PRELUDE, '\n', $query_str);
            std::fs::write(
                &format!("target/{}.sparql.txt", stringify!($const)),
                query_str,
            )
            .expect("Failed to write query to file!");
            Query::parse(query_str, None).unwrap()
        });
    };
}

const QS_CLASSES: &str = r"
SELECT ?s
WHERE {
    {
        VALUES ?t {
            rdfs:Class owl:Class
        }
        ?s rdf:type ?t
    }
    UNION
    {
        ?s rdfs:subClassOf ?o
    }
}
ORDER BY ?s
";
query_parser!(Q_CLASSES, QS_CLASSES);

// const QS_PROPERTIES: &str = r#"
// SELECT ?s ?label ?description ?domain ?range ?domainIncludes ?rangeIncludes
// WHERE {
//     VALUES ( ?t ?t_domainIncludes ?t_rangeIncludes ) {
//         ( ( rdf:Property owl:ObjectProperty owl:DatatypeProperty owl:AnnotationProperty )
//         ( schema:domainIncludes dcam:domainIncludes )
//         ( schema:rangeIncludes dcam:rangeIncludes) )
//     }
//     ?s rdf:type ?t
//     OPTIONAL { ?s rdfs:label ?label }
//     OPTIONAL { ?s rdfs:comment ?description }
//     OPTIONAL { ?s rdfs:domain ?domain }
//     OPTIONAL { ?s rdfs:range ?range }
//     OPTIONAL { ?s ?t_domainIncludes ?domainIncludes }
//     OPTIONAL { ?s ?t_rangeIncludes ?rangeIncludes }
// }
// ORDER BY ?s
// "#;
const QS_PROPERTIES: &str = r"
SELECT
    ?s ?t ?label ?description ?cardinality ?maxCardinality ?minCardinality
    # HACK Due to this bug in OxiGraph, we first have to use the STR() function for the GROUP_CONCAT() argument: <https://github.com/oxigraph/oxigraph/issues/297>
    ( GROUP_CONCAT( DISTINCT STR(?domain); separator=',' ) as ?domainAndList )
    ( GROUP_CONCAT( DISTINCT STR(?range); separator=',' ) as ?rangeAndList )
    ( GROUP_CONCAT( DISTINCT STR(?domainIncludes); separator=',' ) as ?domainIncludesList )
    ( GROUP_CONCAT( DISTINCT STR(?rangeIncludes); separator=',' ) as ?rangeIncludesList )
    ( GROUP_CONCAT( DISTINCT STR(?domainOred); separator=',' ) as ?domainOredList )
    ( GROUP_CONCAT( DISTINCT STR(?rangeOred); separator=',' ) as ?rangeOredList )
WHERE {
    VALUES ?t {
        rdf:Property owl:ObjectProperty owl:DatatypeProperty owl:AnnotationProperty
    } .
    ?s rdf:type ?t .
    OPTIONAL {
        ?s rdfs:label ?label .
    } .
    OPTIONAL {
        ?s rdfs:comment ?description .
    } .
    OPTIONAL {
        ?s owl:cardinality ?cardinality .
    } .
    OPTIONAL {
        ?s owl:maxCardinality ?maxCardinality .
    } .
    OPTIONAL {
        ?s owl:minCardinality ?minCardinality .
    } .
    OPTIONAL {
        ?s rdfs:domain ?domain .
        OPTIONAL {
            ?domain owl:unionOf ?domainOredUnion .
            ?domainOredUnion rdf:rest*/rdf:first ?domainOred .
        }
    } .
    OPTIONAL {
        ?s rdfs:range ?range .
        OPTIONAL {
            ?range owl:unionOf ?rangeOredUnion .
            ?rangeOredUnion rdf:rest*/rdf:first ?rangeOred .
        }
    } .
    OPTIONAL {
        ?s schema:domainIncludes | dcam:domainIncludes | dcid:domainIncludes ?domainIncludes .
    } .
    OPTIONAL {
        ?s schema:rangeIncludes | dcam:rangeIncludes | dcid:rangeIncludes ?rangeIncludes .
    } .
}
GROUP BY ?s ?t ?label ?description ?cardinality ?maxCardinality ?minCardinality
ORDER BY ?s
";
query_parser!(Q_PROPERTIES, QS_PROPERTIES);

// TODO Use this! :
// https://www.linkedin.com/pulse/six-secret-sparql-ninja-tricks-kurt-cagle/
// select ?chapterTitle where {
//     values ?book {book:_StormCrow}
//     ?book rdf:rest*/rdf:first ?chapter.
//     ?chapter chapter:hasTitle ?chapterTitle.
//     }
//
// TODO Or this:
// <https://stackoverflow.com/a/72031892/586229>
// SELECT ?s ?p ?o
//   { ?e  :user_id   123 .
//     ?e  (<>|!<>)*  ?s  .
//     ?s  ?p         ?o
//   }

macro_rules! ins {
    ($store:expr, $subj:expr, $pred:expr, $obj:expr) => {
        $store.insert(&Quad::new($subj, $pred, $obj, GraphName::DefaultGraph))?;
    };
}

macro_rules! ins_opt {
    ($store:expr, $subj:expr, $pred:expr, $solution:expr, $obj_ident:ident) => {
        if let Some(obj) = $solution.get(stringify!($obj_ident)) {
            ins!($store, $subj, $pred, obj.as_ref());
        }
    };
}

macro_rules! type2shape {
    ($shape_var:ident, $orig:expr) => {
        let subj_iri = if let Term::NamedNode(nn) = $orig {
            nn.as_str()
        } else {
            panic!("Only named-node properties subjects are supported!");
        };

        let shape_iri = format!("{subj_iri}Shape");
        let $shape_var = NamedNodeRef::new(&shape_iri)?;
        log::info!("Shape:    {}", $shape_var);
    };
}

fn convert_classes(store_owl: &Store, store_shacl: &mut Store) -> Res<()> {
    log::info!("Converting classes ...");
    if let QueryResults::Solutions(solutions) = store_owl.query(Q_CLASSES.to_owned())? {
        for sol_res in solutions {
            let sol = sol_res?;
            let subj = sol.get("s").unwrap();
            log::info!("Class: {subj}");

            type2shape!(shape, subj);

            ins!(store_shacl, shape, rdf::TYPE, sh::NODE_SHAPE);
            ins!(store_shacl, shape, sh::TARGET_CLASS, subj.clone());
            ins!(store_shacl, shape, sh::CLOSED, *basics::BOOL_FALSE);
        }
        log::info!("Converting classes - done.");
    } else {
        log::warn!("No classes found.");
    }

    Ok(())
}

// fn to_lit_str(term: &Term) -> Res<&str> {
//     if let Term::Literal(literal) = term {
//         Ok(literal.value())
//     } else {
//         Err("".into())
//     }
// }

#[derive(Debug, EnumIter, PartialEq, Eq, PartialOrd, Copy, Clone, Hash)]
enum ListCollectionMethod {
    And,
    Includes,
    Ored,
}

impl ListCollectionMethod {
    pub const fn to_var_postfix(self) -> &'static str {
        match self {
            Self::And => "AndList",
            Self::Includes => "IncludesList",
            Self::Ored => "OredList",
        }
    }

    pub const fn is_and(self) -> bool {
        match self {
            Self::And => true,
            Self::Includes | Self::Ored => false,
        }
    }
}

fn convert_property_range_or_domain(
    store_shacl: &mut Store,
    shape: NamedNodeRef,
    config: &Config,
    sol: &QuerySolution,
    prop: RDProperty,
) -> Res<HashSet<ListCollectionMethod>> {
    let prop_str = prop.to_str();
    // let mut used = EnumMap::from_fn(|_| HashSet::new());
    let mut used = HashSet::new();
    let is_dataype_prop = if let Term::NamedNode(nn) = sol
        .get("t")
        .expect("Required SPARQL var ?t (rdfs:type) missing")
    {
        nn.eq(&owl::DATATYPE_PROPERTY)
    } else {
        panic!("Only named-node properties subjects are supported as `rdfs:type` objects!");
    };
    for collection_method in ListCollectionMethod::iter() {
        let list_var = Variable::new(format!("{prop_str}{}", collection_method.to_var_postfix()))?;
        if let Some(list) = sol.get(&list_var) {
            match list {
                Term::Literal(lit) => {
                    let lit_str = lit.as_ref().value();
                    if !lit_str.is_empty() {
                        used.insert(collection_method);
                        log::info!("    {list_var}:");
                        let parts = lit_str.split(',').collect::<Vec<_>>();
                        let num_parts = parts.len();
                        for part in parts {
                            log::info!("      - {part}:");
                            // type2shape!(part_shape, part);
                            let part_shape_iri = format!("{part}Shape");
                            let part_shape = NamedNodeRef::new(&part_shape_iri)?;
                            if collection_method.is_and() {
                                let is_list = num_parts > 1;
                                if is_list {
                                    match config.and_list_detected[prop] {
                                        OdityHandling::Ignore => {
                                            continue;
                                        }
                                        OdityHandling::Warn => {
                                            log::warn!("And list detected for property {prop_str}; this is not supported in our to-SHACL converter.");
                                        }
                                        OdityHandling::Error => {
                                            return Err(format!("And list detected for property {prop_str}; this is not supported in our to-SHACL converter.").into());
                                        }
                                    }
                                }
                                match prop {
                                    RDProperty::Range => {
                                        if is_dataype_prop {
                                            if num_parts == 1 {
                                                ins!(store_shacl, shape, sh::DATA_TYPE, part_shape);
                                            } else {
                                                todo!(); // TODO
                                            }
                                        } else {
                                            if num_parts == 1 {
                                                ins!(store_shacl, shape, sh::CLASS, part_shape);
                                            } else {
                                                todo!(); // TODO
                                            }
                                        }
                                    }
                                    RDProperty::Domain => {
                                        if num_parts == 1 {
                                            ins!(store_shacl, part_shape, sh::PROPERTY, shape);
                                        } else {
                                            todo!(); // TODO
                                        }
                                    }
                                }
                            } else {
                                match prop {
                                    RDProperty::Range => {
                                        if is_dataype_prop {
                                            ins!(store_shacl, shape, sh::DATA_TYPE, part_shape);
                                        } else {
                                            ins!(store_shacl, shape, sh::CLASS, part_shape);
                                        }
                                    }
                                    RDProperty::Domain => {
                                        ins!(store_shacl, part_shape, sh::PROPERTY, shape);
                                    }
                                }
                            }
                        }
                    }
                }
                // Term::Triple(_) => panic!("RDF triple as or-ed/unionOf-listed object/value for rdfs:domain is not (yet) supported."),
                Term::NamedNode(_) | Term::BlankNode(_) | Term::Triple(_) => {
                    panic!("Type for SPARQL variable {list_var} should be Literal(string).")
                }
            }
        }
    }
    let action = config.style_mix_property[prop];
    if !action.ignore() && (used.len() > 1) {
        let msg = format!(
            "Mixed styles of {} definitions in Property: {}",
            prop_str,
            used.iter()
                .map(|style| format!("{style:?}"))
                .collect::<Vec<_>>()
                .join(", ")
        );
        match action {
            OdityHandling::Error => return Err(msg.into()),
            OdityHandling::Warn => log::warn!("{msg}"),
            OdityHandling::Ignore => panic!("This should never happen"),
        }
    }

    Ok(used)
}

fn convert_properties(store_owl: &Store, store_shacl: &mut Store, config: &Config) -> Res<()> {
    log::info!("Converting properties ...");
    if let QueryResults::Solutions(solutions) = store_owl.query(Q_PROPERTIES.to_owned())? {
        let mut used_prop_styles = EnumMap::from_fn(|_| HashSet::new());
        // let mut used_domain_prop_styles = HashSet::new();
        for sol_res in solutions {
            let sol = sol_res?;
            let subj = sol.get("s").unwrap();
            log::info!("");
            log::info!("Property: {subj}");

            type2shape!(shape, subj);

            ins!(store_shacl, shape, rdf::TYPE, sh::PROPERTY_SHAPE);
            ins!(store_shacl, shape, sh::PATH, subj.clone());
            ins_opt!(store_shacl, shape, sh::NAME, sol, label);
            ins_opt!(store_shacl, shape, sh::DESCRIPTION, sol, description);
            ins_opt!(store_shacl, shape, sh::MIN_COUNT, sol, minCardinaity);
            ins_opt!(store_shacl, shape, sh::MAX_COUNT, sol, maxCardinaity);
            ins_opt!(store_shacl, shape, sh::MIN_COUNT, sol, cardinaity);
            ins_opt!(store_shacl, shape, sh::MAX_COUNT, sol, cardinaity);

            for (prop, used_style) in &mut used_prop_styles {
                used_style.extend(convert_property_range_or_domain(
                    store_shacl,
                    shape,
                    config,
                    &sol,
                    prop,
                )?);
            }
        }
        for (prop, used_style) in &mut used_prop_styles {
            let action = config.style_mix_ontology[prop];
            if !action.ignore() && (used_style.len() > 1) {
                let msg = format!(
                    "Mixed styles of {} definitions in Ontology: {}",
                    prop.to_str(),
                    used_style
                        .iter()
                        .map(|style| format!("{style:?}"))
                        .collect::<Vec<_>>()
                        .join(", ")
                );
                match action {
                    OdityHandling::Error => return Err(msg.into()),
                    OdityHandling::Warn => log::warn!("{msg}"),
                    OdityHandling::Ignore => panic!("This should never happen"),
                }
            }
        }
        log::info!("");
        log::info!("Converting properties done.");
    } else {
        log::warn!("No properties found.");
    }

    // https://spinrdf.org/shacl-and-owl.html

    // :prop1 a owl:ObjectProperty ;
    //   rdfs:domain :A ;
    //   rdfs:range :B .

    // In SHACL

    // :AShape a sh:NodeShape ;
    //  sh:targetClass :A ;
    //  sh:property [sh:path :prop1] ;
    //  sh:closed true .

    // :ADomainShape a sh:NodeShape ;
    //  sh:targetSubjectsOf :prop1 ;
    //  sh:class :A .

    // :prop1RangeShape a sh:NodeShape ;
    //  sh:targetObjectsOf :prop1 ;
    //  sh:class :B .

    // ex:IdentifierShape
    // 	a sh:PropertyShape ;
    // 	sh:targetSubjectsOf ex:identifier ;
    // 	sh:path ex:identifier ;
    // 	sh:maxCount 1 .

    Ok(())
}

pub fn convert(store_owl: &Store, config: &Config) -> Res<()> {
    let mut store_shacl = Store::new()?;

    convert_classes(store_owl, &mut store_shacl)?;
    convert_properties(store_owl, &mut store_shacl, config)?;

    // Output
    // let mut serializer = serializer!(prefixes);
    // let output = serializer.serialize_graph(&mut scm)?.as_str();
    // println!("The resulting graph\n{}", output);

    let mut shacl_file = File::create("target/shacl.ttl")?;
    store_shacl.dump_graph(
        &mut shacl_file,
        GraphFormat::Turtle,
        GraphNameRef::DefaultGraph,
    )?;

    Ok(())
}
