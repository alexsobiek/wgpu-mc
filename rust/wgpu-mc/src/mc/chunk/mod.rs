use crate::mc::block::{BlockPos, BlockState};


use crate::render::world::chunk::BakedChunk;

use std::sync::Arc;
use std::convert::TryInto;
use arc_swap::ArcSwap;
use dashmap::DashMap;

use crate::mc::BlockManager;

pub const CHUNK_WIDTH: usize = 16;
pub const CHUNK_AREA: usize = CHUNK_WIDTH * CHUNK_WIDTH;
pub const CHUNK_HEIGHT: usize = 256;
pub const CHUNK_VOLUME: usize = CHUNK_AREA * CHUNK_HEIGHT;
pub const CHUNK_SECTION_HEIGHT: usize = 1;
pub const CHUNK_SECTIONS_PER: usize = CHUNK_HEIGHT / CHUNK_SECTION_HEIGHT;
pub const SECTION_VOLUME: usize = CHUNK_AREA * CHUNK_SECTION_HEIGHT;

type ChunkPos = (i32, i32);

#[derive(Clone, Debug)]
pub struct ChunkSection {
    pub empty: bool,
    pub blocks: Box<[BlockState; SECTION_VOLUME]>,
    pub offset_y: usize
}

pub struct RenderLayers {
    terrain: Box<[ChunkSection; CHUNK_SECTIONS_PER]>,
    transparent: Box<[ChunkSection; CHUNK_SECTIONS_PER]>,
    grass: Box<[ChunkSection; CHUNK_SECTIONS_PER]>
}

#[derive(Debug)]
pub struct Chunk {
    pub pos: ChunkPos,
    pub sections: Box<[ChunkSection; CHUNK_SECTIONS_PER]>,
    pub baked: Option<BakedChunk>
}

impl Chunk {
    #[must_use]
    pub fn new(pos: ChunkPos, blocks: Box<[BlockState; CHUNK_AREA * CHUNK_HEIGHT]>) -> Self {
        let sections: Box<[ChunkSection; CHUNK_SECTIONS_PER]> = (0..CHUNK_SECTIONS_PER).map(|section| {
            let start_index = section * SECTION_VOLUME;
            let end_index = (section + 1) * SECTION_VOLUME;
            let block_section: Box<[BlockState; SECTION_VOLUME]> = (start_index..end_index).map(|index| {
                blocks[index]
            }).collect::<Box<[BlockState]>>().try_into().unwrap();

            ChunkSection {
                empty: !blocks.iter().any(|state| state.packed_key.is_some()),
                blocks: block_section,
                offset_y: section * CHUNK_SECTION_HEIGHT
            }
        }).collect::<Box<[ChunkSection]>>().try_into().unwrap();

        Self {
            pos,
            sections,
            baked: None
        }
    }

    #[must_use]
    pub fn blockstate_at_pos(&self, pos: BlockPos) -> BlockState {
        let x = (pos.0 % 16) as usize;
        let y = (pos.1) as usize;
        let z = (pos.2 % 16) as usize;

        self.sections[y].blocks[(z * CHUNK_WIDTH) + x]
    }

    pub fn bake(&mut self, block_manager: &BlockManager, device: &wgpu::Device) {
        let baked = BakedChunk::bake(block_manager, self, device);
        self.baked = Some(baked);
    }
}

pub struct ChunkManager {
    //Due to floating point inaccuracy at large distances,
    //we need to keep the model coordinates as close to 0,0,0 as possible
    pub chunk_origin: ArcSwap<ChunkPos>,
    pub loaded_chunks: DashMap<ChunkPos, ArcSwap<Chunk>>
}

impl ChunkManager {
    #[must_use]
    pub fn new() -> Self {
        ChunkManager {
            chunk_origin: ArcSwap::new(Arc::new((0, 0))),
            loaded_chunks: DashMap::new(),
        }
    }

    //TODO: parallelize
    // pub fn bake_meshes(&mut self, blocks: &[Box<dyn Block>]) {
    //     self.loaded_chunks.iter_mut().for_each(
    //         |chunk| chunk.generate_vertices(blocks, self.chunk_origin));
    // }
    //
    // pub fn upload_buffers(&mut self, device: &wgpu::Device) {
    //     self.loaded_chunks.iter_mut().for_each(|chunk| chunk.upload_buffer(device));
    // }
}

impl Default for ChunkManager {
    fn default() -> Self {
        Self::new()
    }
}
