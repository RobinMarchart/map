use std::{collections::HashMap, error::Error, fmt::Display};

use StorageBackend::addChange;
use petgraph::graph::{DiGraph,NodeIndex};
use crate::{ChangeMessage, Position};
use futures::Future;

#[derive(Debug, Clone)]
pub struct NoSuchBranch{
branch:(String,String),
}
impl From<(String,String)> for NoSuchBranch{
    fn from(b: (String,String)) -> Self {
        NoSuchBranch{branch:b}
    }
}
impl Display for NoSuchBranch{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"Branch {}:{} does not exist",self.branch.0,self.branch.1)
    }
}
impl Error for NoSuchBranch{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }
}
#[derive(Debug,Clone)]
pub struct MissingParent{
    parent:[u8;32],
}
impl From<[u8;32]> for MissingParent{
    fn from(p: [u8;32]) -> Self {
        MissingParent{parent:p}
    }
}
impl Display for MissingParent{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"Parent {:?} missing",self.parent)
    }
}
impl Error for MissingParent{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }
}

pub trait StorageBackend<P:Position<P>>{
    type EmptyFuture:Future<Output=Result<(),Box<dyn Error>>>;
    fn addChange(m:&ChangeMessage<P>)->Self::EmptyFuture;
    fn setBranch(b:&(String,String),n:[u8;32])->Self::EmptyFuture;
}

enum StoreError{
    Branch(NoSuchBranch),
    Parent(MissingParent)
}

impl From<StoreError> for Box<dyn Error>{
    fn from(e: StoreError) -> Self {
        match e {
            StoreError::Branch(b)=>b.into(),
            StoreError::Parent(p)=>p.into()
        }
    }
}

pub struct Storage<P:Position<P>,S:StorageBackend<P>>{
    store:DiGraph<[u8;32],(),u32>,
    node_map:HashMap<[u8;32],NodeIndex<u32>>,
    branches:HashMap<(String,String),[u8;32]>,
    change_handlers:Vec<(u32,Box<dyn Fn(&ChangeMessage<P>)->()>)>,
    backend:S,
}

impl<P:Position<P>,S:StorageBackend<P>> Storage<P,S>{

    fn add_change_to_branch(&mut self,m:&ChangeMessage<P>,b:&(String,String))->Result<impl Future<Output=Result<(),Box<dyn Error>>>,StoreError>{
        if self.branches.contains_key(b) {
            for p in m.change.parents.iter(){
                if !self.node_map.contains_key(p){
                    return Err(StoreError::Parent(MissingParent::from(p.clone())));
                }
            }
            async{

            let change_p=addChange(self.backend, uuM);
            LETu a node= self.store.add_node(m.sha256.clone());

            self.node_map.insert(m.sha256.clone(), node):
            }
        }else {
            return Err(StoreError::Branch(NoSuchBranch::from(b.clone())));
        }
    }
}
