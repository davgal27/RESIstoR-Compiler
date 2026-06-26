use ascent::ascent;
use crate::parser::ir::Block;
use crate::parser::ir::Label;
use crate::parser::cfg::Cfg;

ascent! {
	//facts
	relation edge(Label, Label);
	relation entry(Label);

	// derived
	relation reachable(Label);
	
	// rules 
	reachable(block.clone()) <-- entry(block);
	reachable(to.clone()) <-- reachable(from), edge(from,to);
}

pub fn get_unreachables(cfg: &Cfg) -> Vec<Block> {
	let mut prog = AscentProgram::default();

	for (from, to) in &cfg.edges{
		prog.edge.push((from.clone(), to.clone()));
	}

	prog.entry.push((cfg.entry.clone(),));

	// run ascent:
	prog.run();

	let mut unreachable_blocks: Vec<Block> = Vec::new();

	for block in &cfg.blocks {
		let mut in_reachable = false;

		if prog.reachable.contains(&(block.label.clone(),)) {
			in_reachable = true;
		}

		if in_reachable == false {
			unreachable_blocks.push(block.clone());
		}
	}

	unreachable_blocks
}