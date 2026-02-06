use crate::block::Block;
use crate::delta;

pub fn squash(mut parent: Block, current: Block) -> Result<Block, Box<dyn std::error::Error>> {
    log::debug!("squash()");

    for current_delta in &current.payload {
        if let Some(parent_delta) = parent.payload.iter_mut().find(|d| d.name == current_delta.name)
        {
            delta::merge_deltas(parent_delta, current_delta);
        } else {
            parent.payload.push(current_delta.clone());
        }
    }

    Ok(parent)
}
