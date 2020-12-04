#![feature(num_as_ne_bytes,min_const_generics)]
use digest::Digest;

pub trait Digestable{
    fn update_le<D:Digest>(&self,digest:&mut D);
    fn update_be<D:Digest>(&self,digest:&mut D);
}

impl Digestable for i8{
    fn update_le<D:Digest>(&self,digest:&mut D) {
        digest.update(self.to_le_bytes())
    }

    fn update_be<D:Digest>(&self,digest:&mut D) {
        digest.update(self.to_be_bytes())
    }
}
impl Digestable for i16{
    fn update_le<D:Digest>(&self,digest:&mut D) {
        digest.update(self.to_le_bytes())
    }

    fn update_be<D:Digest>(&self,digest:&mut D) {
        digest.update(self.to_be_bytes())
    }
}
impl Digestable for i32{
    fn update_le<D:Digest>(&self,digest:&mut D) {
        digest.update(self.to_le_bytes())
    }

    fn update_be<D:Digest>(&self,digest:&mut D) {
        digest.update(self.to_be_bytes())
    }
}
impl Digestable for i64{
    fn update_le<D:Digest>(&self,digest:&mut D) {
        digest.update(self.to_le_bytes())
    }

    fn update_be<D:Digest>(&self,digest:&mut D) {
        digest.update(self.to_be_bytes())
    }
}
impl Digestable for i128{
    fn update_le<D:Digest>(&self,digest:&mut D) {
        digest.update(self.to_le_bytes())
    }

    fn update_be<D:Digest>(&self,digest:&mut D) {
        digest.update(self.to_be_bytes())
    }
}
impl Digestable for u8{
    fn update_le<D:Digest>(&self,digest:&mut D) {
        digest.update(self.to_le_bytes())
    }

    fn update_be<D:Digest>(&self,digest:&mut D) {
        digest.update(self.to_be_bytes())
    }
}
impl Digestable for u16{
    fn update_le<D:Digest>(&self,digest:&mut D) {
        digest.update(self.to_le_bytes())
    }

    fn update_be<D:Digest>(&self,digest:&mut D) {
        digest.update(self.to_be_bytes())
    }
}
impl Digestable for u32{
    fn update_le<D:Digest>(&self,digest:&mut D) {
        digest.update(self.to_le_bytes())
    }

    fn update_be<D:Digest>(&self,digest:&mut D) {
        digest.update(self.to_be_bytes())
    }
}
impl Digestable for u64{
    fn update_le<D:Digest>(&self,digest:&mut D) {
        digest.update(self.to_le_bytes())
    }

    fn update_be<D:Digest>(&self,digest:&mut D) {
        digest.update(self.to_be_bytes())
    }
}
impl Digestable for u128{
    fn update_le<D:Digest>(&self,digest:&mut D) {
        digest.update(self.to_le_bytes())
    }

    fn update_be<D:Digest>(&self,digest:&mut D) {
        digest.update(self.to_be_bytes())
    }
}
impl<T:Digestable> Digestable for Option<T>{
    fn update_le<D:Digest>(&self,digest:&mut D) {
        match self {
            None=>{(0 as u8).update_le(digest);},
            Some(x)=>{
                (1 as u8).update_le(digest);
                x.update_le(digest);
            }
        }
    }

    fn update_be<D:Digest>(&self,digest:&mut D) {
        match self {
            None=>(0 as u8).update_be(digest),
            Some(x)=>{
                (1 as u8).update_be(digest);
                x.update_be(digest);
            }
        }
    }
}

impl<T:Digestable> Digestable for Vec<T>{
    fn update_le<D:Digest>(&self,digest:&mut D) {
        self.iter().for_each(|x|x.update_le(digest))
    }

    fn update_be<D:Digest>(&self,digest:&mut D) {
        self.iter().for_each(|x|x.update_be(digest))
    }
}
impl<T:Digestable,const N:usize> Digestable for [T;N]{
    fn update_le<D:Digest>(&self,digest:&mut D) {
        self.iter().for_each(|x|x.update_le(digest))
    }

    fn update_be<D:Digest>(&self,digest:&mut D) {
        self.iter().for_each(|x|x.update_be(digest))
    }
}
impl Digestable for &str{
    fn update_le<D:Digest>(&self,digest:&mut D) {
        digest.update(self.as_bytes())
    }

    fn update_be<D:Digest>(&self,digest:&mut D) {
        digest.update(self.as_bytes())
    }
}
impl Digestable for &[u8]{
    fn update_le<D:Digest>(&self,digest:&mut D) {
        digest.update(self)
    }

    fn update_be<D:Digest>(&self,digest:&mut D) {
        digest.update(self)
    }
}
impl Digestable for String{
    fn update_le<D:Digest>(&self,digest:&mut D) {
        self.as_str().update_le(digest)
    }

    fn update_be<D:Digest>(&self,digest:&mut D) {
        self.as_str().update_be(digest)
    }
}
