// build cfg function and cfg struct for after parsing
/*
Nodes: Blocks
Edges: Block to another block
*/
use super::ir::*;

#[derive(Debug)]
pub struct Cfg {
    pub blocks: Vec<Block>,
    pub edges: Vec<(Label, Label)>,
    pub entry: Label,
}

// one cfg for one function
pub fn build_cfg(function: &Function) -> Cfg {
    let blocks = function.blocks.clone();
    let mut edges = Vec::new();
    let entry = function.entry.clone();

    for block in &function.blocks {
        match &block.term {
            Term::Jump(label) => {
                edges.push((block.label.clone(), label.clone()));
            }

            Term::CJump(_, label_one, label_two) => {
                edges.push((block.label.clone(), label_one.clone()));
                edges.push((block.label.clone(), label_two.clone()));
            }

            Term::Return(_) => {
                // return doesnt have edges
            }
        }
    }

    Cfg {
        blocks,
        edges,
        entry,
    }
}