use committer::generate_trie_config;
use committer::patricia_merkle_tree::original_skeleton_tree::config::OriginalSkeletonTreeConfig;
use committer::patricia_merkle_tree::original_skeleton_tree::errors::OriginalSkeletonTreeError;
use committer::patricia_merkle_tree::original_skeleton_tree::tree::OriginalSkeletonTreeResult;
use committer::patricia_merkle_tree::types::NodeIndex;

use crate::block_committer::input::StarknetStorageValue;
use crate::patricia_merkle_tree::leaf::leaf_impl::ContractState;
use crate::patricia_merkle_tree::types::CompiledClassHash;

generate_trie_config!(OriginalSkeletonStorageTrieConfig, StarknetStorageValue);

generate_trie_config!(OriginalSkeletonClassesTrieConfig, CompiledClassHash);

pub(crate) struct OriginalSkeletonContractsTrieConfig;

impl OriginalSkeletonTreeConfig<ContractState> for OriginalSkeletonContractsTrieConfig {
    fn compare_modified_leaves(&self) -> bool {
        false
    }

    fn compare_leaf(
        &self,
        _index: &NodeIndex,
        _previous_leaf: &ContractState,
    ) -> OriginalSkeletonTreeResult<bool> {
        Ok(false)
    }
}

impl OriginalSkeletonContractsTrieConfig {
    pub(crate) fn new() -> Self {
        Self
    }
}