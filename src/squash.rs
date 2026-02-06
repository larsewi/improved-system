use crate::block::Block;

pub fn squash(_current: Block, parent: Block) -> Result<Block, Box<dyn std::error::Error>> {
    // TODO: Implement squash logic
    log::debug!("squash()");
    return Ok(parent);
}
