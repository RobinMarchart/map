use serde::{Deserialize,Serialize};
use sha2::{Digest, Sha256};
use structured_digest::Digestable;
use crate::Position;

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct ElementChange<P:Position<P>> {
    pub id:u64,
    pub variant:Option<u16>,
    pub position:Option<P>,
}
impl<P:Position<P>> Digestable for ElementChange<P>{
    fn update_le<D:sha2::Digest>(&self,digest:&mut D) {
        self.id.update_le(digest);
        self.variant.update_le(digest);
        self.position.update_le(digest)
    }

    fn update_be<D:sha2::Digest>(&self,digest:&mut D) {
        self.id.update_be(digest);
        self.variant.update_be(digest);
        self.position.update_be(digest)
    }
}

#[derive(Serialize,Deserialize,Debug, Clone)]
pub struct EntityChange<P:Position<P>> {
    pub id: u64,
    pub ent_type: Option<String>,
    pub variant: Option<u8>,
    pub orientation: Option<u8>,
    pub base_position: Option<P>,
    pub elements: Option<Option<Vec<ElementChange<P>>>>,
}

impl<P:Position<P>> Digestable for EntityChange<P>{
    fn update_le<D:sha2::Digest>(&self,digest:&mut D) {
        self.id.update_le(digest);
        self.ent_type.update_le(digest);
        self.variant.update_le(digest);
        self.orientation.update_le(digest);
        self.base_position.update_le(digest);
        self.elements.update_le(digest)
    }

    fn update_be<D:sha2::Digest>(&self,digest:&mut D) {
        self.id.update_be(digest);
        self.ent_type.update_be(digest);
        self.variant.update_be(digest);
        self.orientation.update_be(digest);
        self.base_position.update_be(digest);
        self.elements.update_be(digest)
    }
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Change<P:Position<P>>{
    pub parents: Vec<[u8;32]>,
    pub changes:Vec<EntityChange<P>>,
}

impl <P:Position<P>> Digestable for Change<P>{
    fn update_le<D:sha2::Digest>(&self,digest:&mut D) {
        let mut d= D::new();
        self.parents.update_le(&mut d);
        digest.update(d.finalize());
        self.changes.update_le(digest);
    }

    fn update_be<D:sha2::Digest>(&self,digest:&mut D) {
        let mut d=D::new();
        self.parents.update_be(&mut d);
        digest.update(d.finalize());
        self.changes.update_be(digest);
    }
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct ChangeMessage<P:Position<P>>{
    pub change:Change<P>,
    pub sha256:[u8;32]
}
impl<P:Position<P>> From<Change<P>> for ChangeMessage<P>{
    fn from(c: Change<P>) -> Self {
        let mut hasher=Sha256::new();
        c.update_le(&mut hasher);
        ChangeMessage{
            change:c,
            sha256:hasher.finalize().into()
        }
    }
}
