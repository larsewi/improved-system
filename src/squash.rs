use crate::block::Block;

pub fn squash(parent: Block, current: Block) -> Result<Block, Box<dyn std::error::Error>> {
    log::debug!("Merging {:#?} into {:#?}", current, parent);
    return Ok(parent);
}
