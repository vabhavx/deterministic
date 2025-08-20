//! Merkle Tree implementation for deterministic dependency verification
//!
//! This module provides a cryptographically secure Merkle tree implementation
//! using ring for hashing and std for core functionality.

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Result as IoResult};
use std::path::Path;
use ring::digest::{self, SHA256};

/// Trait for parsing lockfiles from different package managers
pub trait LockfileParser {
    /// Parses a lockfile and extracts dependency information
    fn parse_lockfile<P: AsRef<Path>>(&self, path: P) -> IoResult<Vec<Dependency>>;
    
    /// Returns the package manager type
    fn package_manager(&self) -> &'static str;
}

/// Represents a dependency with its metadata
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Dependency {
    pub name: String,
    pub version: String,
    pub integrity: Option<String>,
    pub resolved: Option<String>,
}

/// Node in the Merkle tree
#[derive(Debug, Clone)]
pub struct MerkleNode {
    hash: Vec<u8>,
    left: Option<Box<MerkleNode>>,
    right: Option<Box<MerkleNode>>,
}

/// Merkle tree for dependency verification
#[derive(Debug)]
pub struct MerkleTree {
    root: Option<MerkleNode>,
    leaves: Vec<Vec<u8>>,
}

impl MerkleTree {
    /// Creates a new empty Merkle tree
    pub fn new() -> Self {
        Self {
            root: None,
            leaves: Vec::new(),
        }
    }
    
    /// Builds the Merkle tree from a list of dependencies
    pub fn build_from_dependencies(&mut self, dependencies: Vec<Dependency>) -> Result<(), String> {
        // Convert dependencies to leaf hashes
        self.leaves = dependencies
            .into_iter()
            .map(|dep| self.hash_dependency(&dep))
            .collect();
        
        if self.leaves.is_empty() {
            return Err("Cannot build tree from empty dependency list".to_string());
        }
        
        // Build the tree bottom-up
        self.root = Some(self.build_tree_recursive(&self.leaves));
        Ok(())
    }
    
    /// Calculates the root hash of the Merkle tree
    pub fn root_hash(&self) -> Option<Vec<u8>> {
        self.root.as_ref().map(|node| node.hash.clone())
    }
    
    /// Returns the root hash as a hexadecimal string
    pub fn root_hash_hex(&self) -> Option<String> {
        self.root_hash().map(|hash| hex::encode(hash))
    }
    
    /// Verifies a dependency is included in the tree (proof of inclusion)
    pub fn verify_inclusion(&self, dependency: &Dependency, proof: &[Vec<u8>]) -> bool {
        if let Some(root) = &self.root {
            let leaf_hash = self.hash_dependency(dependency);
            self.verify_proof(&leaf_hash, proof, &root.hash)
        } else {
            false
        }
    }
    
    /// Generates a proof of inclusion for a given dependency
    pub fn generate_proof(&self, dependency: &Dependency) -> Option<Vec<Vec<u8>>> {
        let leaf_hash = self.hash_dependency(dependency);
        
        // Find the index of the leaf
        let leaf_index = self.leaves.iter().position(|h| h == &leaf_hash)?;
        
        // Generate proof path
        self.generate_proof_recursive(&self.root.as_ref()?, leaf_index, self.leaves.len())
    }
    
    /// Private method to hash a dependency consistently
    fn hash_dependency(&self, dependency: &Dependency) -> Vec<u8> {
        let mut context = digest::Context::new(&SHA256);
        context.update(dependency.name.as_bytes());
        context.update(b":");
        context.update(dependency.version.as_bytes());
        
        if let Some(integrity) = &dependency.integrity {
            context.update(b":");
            context.update(integrity.as_bytes());
        }
        
        if let Some(resolved) = &dependency.resolved {
            context.update(b":");
            context.update(resolved.as_bytes());
        }
        
        context.finish().as_ref().to_vec()
    }
    
    /// Recursively builds the Merkle tree
    fn build_tree_recursive(&self, hashes: &[Vec<u8>]) -> MerkleNode {
        if hashes.len() == 1 {
            return MerkleNode {
                hash: hashes[0].clone(),
                left: None,
                right: None,
            };
        }
        
        let mid = (hashes.len() + 1) / 2;
        let left_child = self.build_tree_recursive(&hashes[..mid]);
        let right_child = if mid < hashes.len() {
            Some(self.build_tree_recursive(&hashes[mid..]))
        } else {
            None
        };
        
        let combined_hash = if let Some(ref right) = right_child {
            self.hash_combine(&left_child.hash, &right.hash)
        } else {
            left_child.hash.clone()
        };
        
        MerkleNode {
            hash: combined_hash,
            left: Some(Box::new(left_child)),
            right: right_child.map(Box::new),
        }
    }
    
    /// Combines two hashes using SHA256
    fn hash_combine(&self, left: &[u8], right: &[u8]) -> Vec<u8> {
        let mut context = digest::Context::new(&SHA256);
        context.update(left);
        context.update(right);
        context.finish().as_ref().to_vec()
    }
    
    /// Verifies a Merkle proof
    fn verify_proof(&self, leaf_hash: &[u8], proof: &[Vec<u8>], root_hash: &[u8]) -> bool {
        let mut current_hash = leaf_hash.to_vec();
        
        for sibling_hash in proof {
            current_hash = self.hash_combine(&current_hash, sibling_hash);
        }
        
        current_hash == root_hash
    }
    
    /// Recursively generates proof of inclusion
    fn generate_proof_recursive(
        &self,
        node: &MerkleNode,
        leaf_index: usize,
        total_leaves: usize,
    ) -> Option<Vec<Vec<u8>>> {
        // Implementation would go here - simplified for scaffolding
        // This is a complex recursive algorithm that tracks the path
        // from leaf to root and collects sibling hashes
        Some(Vec::new())
    }
}

impl Default for MerkleTree {
    fn default() -> Self {
        Self::new()
    }
}

/// npm lockfile parser implementation
pub struct NpmLockfileParser;

impl LockfileParser for NpmLockfileParser {
    fn parse_lockfile<P: AsRef<Path>>(&self, path: P) -> IoResult<Vec<Dependency>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut dependencies = Vec::new();
        
        // Simple parsing implementation - would need proper JSON parsing
        // This is scaffolding code
        for line in reader.lines() {
            let line = line?;
            // Parse npm lockfile format
            // Implementation would extract name, version, integrity, resolved
        }
        
        Ok(dependencies)
    }
    
    fn package_manager(&self) -> &'static str {
        "npm"
    }
}

/// pip requirements.txt parser implementation
pub struct PipLockfileParser;

impl LockfileParser for PipLockfileParser {
    fn parse_lockfile<P: AsRef<Path>>(&self, path: P) -> IoResult<Vec<Dependency>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut dependencies = Vec::new();
        
        // Simple parsing implementation - would need proper requirements.txt parsing
        // This is scaffolding code
        for line in reader.lines() {
            let line = line?;
            // Parse requirements.txt format
            // Implementation would extract name, version
        }
        
        Ok(dependencies)
    }
    
    fn package_manager(&self) -> &'static str {
        "pip"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_merkle_tree_creation() {
        let mut tree = MerkleTree::new();
        assert!(tree.root_hash().is_none());
    }
    
    #[test]
    fn test_dependency_hashing() {
        let tree = MerkleTree::new();
        let dep = Dependency {
            name: "test-package".to_string(),
            version: "1.0.0".to_string(),
            integrity: Some("sha512-abc123".to_string()),
            resolved: None,
        };
        
        let hash1 = tree.hash_dependency(&dep);
        let hash2 = tree.hash_dependency(&dep);
        assert_eq!(hash1, hash2, "Dependency hashing should be deterministic");
    }
    
    #[test]
    fn test_build_tree_from_dependencies() {
        let mut tree = MerkleTree::new();
        let dependencies = vec![
            Dependency {
                name: "package-a".to_string(),
                version: "1.0.0".to_string(),
                integrity: None,
                resolved: None,
            },
            Dependency {
                name: "package-b".to_string(),
                version: "2.0.0".to_string(),
                integrity: None,
                resolved: None,
            },
        ];
        
        assert!(tree.build_from_dependencies(dependencies).is_ok());
        assert!(tree.root_hash().is_some());
    }
}
