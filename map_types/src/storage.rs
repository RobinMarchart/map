use std::{collections::HashMap, error::Error, fmt::Display};

use crate::{ChangeMessage, Position};
use futures::Future;
use petgraph::graph::{DiGraph, NodeIndex};
use std::borrow::Borrow;

#[derive(Debug, Clone)]
pub struct NoSuchBranch {
    branch: (String, String),
}
impl From<(String, String)> for NoSuchBranch {
    fn from(b: (String, String)) -> Self {
        NoSuchBranch { branch: b }
    }
}
impl Display for NoSuchBranch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Branch {}:{} does not exist",
            self.branch.0, self.branch.1
        )
    }
}
impl Error for NoSuchBranch {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }
}
#[derive(Debug, Clone)]
pub struct MissingParent {
    parent: [u8; 32],
}
impl From<[u8; 32]> for MissingParent {
    fn from(p: [u8; 32]) -> Self {
        MissingParent { parent: p }
    }
}
impl Display for MissingParent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parent {:?} missing", self.parent)
    }
}
impl Error for MissingParent {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }
}

pub trait StorageBackend<P: Position<P>> {
    type EmptyFuture: Future<Output = Result<(), Box<dyn Error>>>+Unpin;
    fn add_change(self, m: &ChangeMessage<P>) -> Self::EmptyFuture;
    fn set_branch(self, b: &(String, String), n: [u8; 32]) -> Self::EmptyFuture;
}

enum StoreError {
    Branch(NoSuchBranch),
    Parent(MissingParent),
}

impl From<StoreError> for Box<dyn Error> {
    fn from(e: StoreError) -> Self {
        match e {
            StoreError::Branch(b) => b.into(),
            StoreError::Parent(p) => p.into(),
        }
    }
}

pub struct Storage<P: Position<P>, S: StorageBackend<P>> {
    store: DiGraph<[u8; 32], (), u32>,
    node_map: HashMap<[u8; 32], NodeIndex<u32>>,
    branches: HashMap<(String, String), [u8; 32]>,
    change_handlers: Vec<(
        u32,
        Box<
            dyn Fn(
                &ChangeMessage<P>,
            ) -> Option<Box<dyn Future<Output = Result<(), Box<dyn Error>>>+Unpin>>,
        >,
    )>,
    backend: S,
}

impl<P: Position<P>, S: StorageBackend<P>> Storage<P, S> {
    fn add_change_to_branch(
        &mut self,
        m: &ChangeMessage<P>,
        b: &(String, String),
    ) -> Result<impl Future<Output = Result<(), Box<dyn Error>>>+Unpin, StoreError> {
        if self.branches.contains_key(b) {
            for p in m.change.parents.iter() {
                if !self.node_map.contains_key(p) {
                    return Err(StoreError::Parent(MissingParent::from(p.clone())));
                }
            }
            return async {
                let change_p = self.backend.add_change(m);
                let node = self.store.add_node(m.sha256.clone());
                m.change.parents.iter().for_each(|p| {
                    self.store.add_edge(
                        self.node_map.get(p).expect("just tested!").clone(),
                        node,
                        (),
                    );
                });
                self.node_map.insert(m.sha256.clone(), node);
                change_p.await?;
                let branch_p = self.backend.set_branch(b, m.sha256);
                self.branches
                    .get_mut(b)
                    .expect("just tested")
                    .copy_from_slice(m.sha256.borrow());
                branch_p.await?;
                for p in self.change_handlers.iter().map(|h| h.1(m)) {
                    match p {
                        None => (),
                        Some(pr) => match pr.await {
                            Ok(_) => (),
                            Err(e) => return Err(e),
                        },
                    };
                }
            }
        } else {
            return Err(StoreError::Branch(NoSuchBranch::from(b.clone())));
        }
    }
}
