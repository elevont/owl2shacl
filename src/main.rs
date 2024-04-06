// SPDX-FileCopyrightText: 2023 - 2024 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

mod config;
mod convert;
mod vocab;

use std::fs::File;
use std::io::BufReader;

use config::Config;
use log::LevelFilter;
use oxigraph::io::GraphFormat;
// use oxigraph::model::vocab::xsd;

use oxigraph::model::GraphName;
use oxigraph::store::Store;

type Error = Box<dyn std::error::Error + Sync + Send>;
type Res<O> = Result<O, Error>;

const SAMPLE_OWL_ONTOLOGY_TURTLE_FILE_PATHS: [(&str, &str, &str); 3] = [
    (
        "res/ont/okh-losh-snapshot-2024-01-07.ttl",
        "okh_losh",
        "https://w3id.org/oseg/ont/okh",
    ),
    (
        "res/ont/vf.ttl",
        "value_flows",
        "https://w3id.org/valueflows",
    ),
    (
        "res/ont/test-onto-mult-val.ttl",
        "test_multi_valued_properties",
        "https://example.org/ont/testmv",
    ),
];

fn load_onto_into_store(store: &mut Store, onto_id: usize) -> Res<()> {
    if let Some((ont_file, _ont_name, ont_base_iri)) =
        SAMPLE_OWL_ONTOLOGY_TURTLE_FILE_PATHS.get(onto_id)
    {
        let buf_ttl_ont_reader = BufReader::new(File::open(ont_file)?);
        // let graph_name = GraphName::NamedNode(NamedNode::new(*ont_base_iri)?);
        let graph_name = GraphName::DefaultGraph;
        store.load_graph(
            buf_ttl_ont_reader,
            GraphFormat::Turtle,
            graph_name.as_ref(),
            Some(ont_base_iri),
        )?;
    }

    Ok(())
}

// fn oxigraph_query_example() -> Res<()> {
//     let mut store = Store::new().unwrap();

//     // insertion
//     let ex = NamedNode::new("http://example.com").unwrap();
//     let quad = Quad::new(ex.clone(), ex.clone(), ex.clone(), GraphName::DefaultGraph);
//     store.insert(&quad).unwrap();

//     for ont_id in 0..SAMPLE_OWL_ONTOLOGY_TURTLE_FILE_PATHS.len() {
//         load_onto_into_store(&mut store, ont_id)?;
//     }

//     // quad filter
//     let results = store
//         .quads_for_pattern(Some(ex.as_ref().into()), None, None, None)
//         .collect::<Result<Vec<Quad>, _>>()
//         .unwrap();
//     assert_eq!(vec![quad], results);

//     // SPARQL query
//     if let QueryResults::Solutions(mut solutions) =
//         store.query("SELECT ?s WHERE { ?s ?p ?o }").unwrap()
//     {
//         assert_eq!(
//             solutions.next().unwrap().unwrap().get("s"),
//             Some(&ex.into())
//         );
//     }

//     Ok(())
// }

fn load_source() -> Res<Store> {
    let mut store_owl = Store::new()?;
    log::info!("Loading ...");
    load_onto_into_store(&mut store_owl, 1)?;
    log::info!("Loaded.");
    log::info!("store_owl len: {}", store_owl.len()?);
    log::info!("Optimizing ...");
    store_owl.optimize()?;
    log::info!("Optimized.");
    Ok(store_owl)
}

fn main() -> Res<()> {
    env_logger::builder().filter_level(LevelFilter::Info).init();
    // oxigraph_query_example()
    let store_owl = load_source()?;
    let config = Config::default();
    convert::convert(&store_owl, &config)
    // construct_convert()
}
