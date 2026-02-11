use anyhow::{Context, Result};
use chemcore::daylight::read_smiles;
use chemcore::molecule::Element;
use gamma::graph::{Graph, Vertex};
use gamma::traversal::DepthFirst;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/*
Gaurav Sablok
codeprog@icloud.com
*/

pub fn smiles(smiles: &str) -> Result<String, Box<dyn Error>> {
    let file = File::open(smiles).expect("file not present");
    let fileread = BufReader::new(file);
    let mut vecsmiles: Vec<String> = Vec::new();
    for i in fileread.lines() {
        let line = i.expect("file not present");
        let linevec = line.split(";").collect::<Vec<_>>()[0].to_string();
        vecsmiles.push(linevec);
    }
    let molecules: Vec<(chemcore::molecule::Molecule, usize, usize)> = Vec::new();
    for i in vecsmiles.iter() {
        let value = i;
        let molecule: chemcore::molecule::Molecule =
            read_smiles(i, None).expect("failed to present smiles");
        let vertex = molecule.vertex_count();
        let edge = molecule.edge_count();
        let valueinsert: (chemcore::molecule::Molecule, usize, usize) = (molecule, vertex, edge);
        molecules.push(valueinsert);

        /*
        Graph construction and traversal
        */

        let mut finalvec: Vec<HashSet<usize>> = Vec::new();
        for i in molecules.iter() {
            let mut visited = HashSet::new();
            let traversal = DepthFirst::new(&i.0, 0).expect("file not present");
            for step in traversal {
                let atom_idx = step.vertex;
                let atom = molecule.vertex_value(atom_idx).context("Missing atom")?;
                let symbol = match atom.element {
                    Element::C => "C",
                    Element::O => "O",
                    _ => "?",
                };
                println!(
                    "Atom {}: {} (degree: {})",
                    atom_idx,
                    symbol,
                    molecule.degree(atom_idx)
                );
                visited.insert(atom_idx);
            }
            finalvec.push(visited);
        }

        /*
        creating an encoder for the GNN.
        */

        let mut vecgnn: Vec<(String, usize)> = Vec::new();
        for i in molecules.iter() {
            for idx in 0..i.0.vertex_count() {
                let atom = molecule.vertex_value(idx).unwrap();
                let symbol = format!("{:?}", atom.element);
                let degree = molecule.degree(idx).unwrap();
                let encoding: (String, usize) = (symbol, degree);
                vecgnn.push(encoding);
            }
        }
    }
    Ok("String graph has been written".to_string())
}
