use serde::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::convert::TryFrom;
use std::hash::{Hash, Hasher};
use std::ops::{Add, AddAssign, Sub, SubAssign, Neg};
use structured_digest::Digestable;

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Position4 {
    pub x: i16,
    pub y: i16,
}
#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq)]
pub struct Position6Axial {
    pub x: i16,
    pub y: i16,
}
impl PartialEq<Position6Cube> for Position6Axial {
    fn eq(&self, other: &Position6Cube) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Hash for Position6Axial {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq)]
pub struct Position6Cube {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}
impl PartialEq<Position6Axial> for Position6Cube {
    fn eq(&self, other: &Position6Axial) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Hash for Position6Cube {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

pub trait PositionHelper:Sized {
fn neighbours(&self)->Vec<Self>;
}

pub trait Position<T: Position<T>>: Add<T> + AddAssign<T> + Sub<T> + SubAssign<T> + Neg + PositionHelper + Digestable {
    fn field_length(&self, other: &T) -> u32;
    fn line_length(&self, other: &T) -> f64;
}

impl PositionHelper for Position4{
    fn neighbours(&self)->Vec<Self> {
        vec![
            self.clone()+Position4::from((1,0)),
            self.clone()+Position4::from((0,1)),
            self.clone()+Position4::from((-1,0)),
            self.clone()+Position4::from((0,-1))
        ]
    }
}

impl Position<Position4> for Position4 {
    fn field_length(&self, other: &Position4) -> u32 {
        u32::try_from(
            (i32::from(self.x) - i32::from(other.x)).abs()
                + (i32::from(self.y) - i32::from(other.y)),
        )
            .expect("Result should be >=0")
    }

    fn line_length(&self, other: &Position4) -> f64 {
        f64::from(
            (i32::from(self.x) - i32::from(other.x)) * (i32::from(self.x) - i32::from(other.x))
                + (i32::from(self.y) - i32::from(other.y))
                * (i32::from(self.y) - i32::from(other.y)),
        )
            .sqrt()
    }
}

impl PositionHelper for Position6Axial{
    fn neighbours(&self)->Vec<Self> {
        vec![
            self.clone()+Position6Axial::from((1,0)),
            self.clone()+Position6Axial::from((0,1)),
            self.clone()+Position6Axial::from((-1,1)),
            self.clone()+Position6Axial::from((-1,0)),
            self.clone()+Position6Axial::from((0,-1)),
            self.clone()+Position6Axial::from((1,-1))
        ]
    }
}

impl Position<Position6Axial> for Position6Axial {
    fn field_length(&self, other: &Position6Axial) -> u32 {
        self.to_cube().field_length(other.to_cube().borrow())
    }

    fn line_length(&self, other: &Position6Axial) -> f64 {
        let dx = i32::from(self.x - other.x);
        let dy = i32::from(self.y - other.y);
        f64::from(dx * dx + dy * dy + dx * dy).sqrt()
    }
}

impl PositionHelper for Position6Cube{
    fn neighbours(&self)->Vec<Self> {
        vec![
            self.clone()+Position6Cube::from((1,0,-1)),
            self.clone()+Position6Cube::from((0,1,-1)),
            self.clone()+Position6Cube::from((-1,1,0)),
            self.clone()+Position6Cube::from((-1,0,1)),
            self.clone()+Position6Cube::from((0,-1,1)),
            self.clone()+Position6Cube::from((1,-1,0))
        ]
    }
}

impl Position<Position6Cube> for Position6Cube {
    fn field_length(&self, other: &Position6Cube) -> u32 {
        u32::try_from(
            (i32::from(self.x) - i32::from(other.x)).abs()
                + (i32::from(self.y) - i32::from(other.y)).abs()
                + (i32::from(self.z) - i32::from(other.z)).abs(),
        )
            .expect("Result should be >=0")
    }

    fn line_length(&self, other: &Position6Cube) -> f64 {
        self.to_axial().line_length(other.to_axial().borrow())
    }
}

impl Position<Position6Cube> for Position6Axial {
    fn field_length(&self, other: &Position6Cube) -> u32 {
        self.to_cube().field_length(other)
    }

    fn line_length(&self, other: &Position6Cube) -> f64 {
        self.line_length(other.to_axial().borrow())
    }
}

impl Position<Position6Axial> for Position6Cube {
    fn field_length(&self, other: &Position6Axial) -> u32 {
        self.field_length(other.to_cube().borrow())
    }

    fn line_length(&self, other: &Position6Axial) -> f64 {
        self.to_axial().line_length(other)
    }
}

pub trait Position6: From<Position6Axial> + From<Position6Cube> {
    fn to_axial(&self) -> Position6Axial;
    fn to_cube(&self) -> Position6Cube;
}

impl From<Position6Axial> for Position6Cube {
    fn from(other: Position6Axial) -> Self {
        Position6Cube {
            x: other.x,
            y: other.y,
            z: -(other.x + other.y),
        }
    }
}

impl From<Position6Cube> for Position6Axial {
    fn from(other: Position6Cube) -> Self {
        Position6Axial {
            x: other.x,
            y: other.y,
        }
    }
}

impl Position6 for Position6Axial {
    fn to_axial(&self) -> Position6Axial {
        self.clone()
    }

    fn to_cube(&self) -> Position6Cube {
        Position6Cube {
            x: self.x,
            y: self.y,
            z: -(self.x + self.y),
        }
    }
}

impl Position6 for Position6Cube {
    fn to_axial(&self) -> Position6Axial {
        Position6Axial {
            x: self.x,
            y: self.y,
        }
    }

    fn to_cube(&self) -> Position6Cube {
        self.clone()
    }
}

impl Add for Position4 {
    type Output = Position4;

    fn add(self, rhs: Position4) -> Self::Output {
        Position4 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Position4 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
impl Sub for Position4 {
    type Output = Position4;

    fn sub(self, rhs: Self) -> Self::Output {
        Position4 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl SubAssign for Position4 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Add for Position6Axial {
    type Output = Position6Axial;

    fn add(self, rhs: Self) -> Self::Output {
        Position6Axial {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl AddAssign for Position6Axial {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
impl Sub for Position6Axial {
    type Output = Position6Axial;

    fn sub(self, rhs: Self) -> Self::Output {
        Position6Axial {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl SubAssign for Position6Axial {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Add for Position6Cube {
    type Output = Position6Cube;

    fn add(self, rhs: Self) -> Self::Output {
        Position6Cube {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl AddAssign for Position6Cube {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}
impl Sub for Position6Cube {
    type Output = Position6Cube;

    fn sub(self, rhs: Self) -> Self::Output {
        Position6Cube {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
impl SubAssign for Position6Cube {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Add<Position6Cube> for Position6Axial {
    type Output = Position6Axial;

    fn add(self, rhs: Position6Cube) -> Self::Output {
        Position6Axial{
            x:self.x+rhs.x,
            y:self.y+rhs.y
        }
    }
}
impl AddAssign<Position6Cube> for Position6Axial {
    fn add_assign(&mut self, rhs: Position6Cube) {
        self.x+=rhs.x;
        self.y+=rhs.y;
    }
}
impl Sub<Position6Cube> for Position6Axial {
    type Output = Position6Axial;

    fn sub(self, rhs: Position6Cube) -> Self::Output {
        Position6Axial{
            x:self.x-rhs.x,
            y:self.y-rhs.y
        }
    }
}
impl SubAssign<Position6Cube> for Position6Axial {
    fn sub_assign(&mut self, rhs: Position6Cube) {
        self.x-=rhs.x;
        self.y-=rhs.y;
    }
}

impl Add<Position6Axial> for Position6Cube {
    type Output = Position6Cube;

    fn add(self, rhs: Position6Axial) -> Self::Output {
        Position6Cube{
            x:self.x+rhs.x,
            y:self.y+rhs.y,
            z:self.z-(rhs.x+rhs.y)
        }
    }
}
impl AddAssign<Position6Axial> for Position6Cube {
    fn add_assign(&mut self, rhs: Position6Axial) {
        self.x+=rhs.x;
        self.y+=rhs.y;
        self.x-=rhs.x+rhs.y
    }
}
impl Sub<Position6Axial> for Position6Cube {
    type Output = Position6Cube;

    fn sub(self, rhs: Position6Axial) -> Self::Output {
        Position6Cube{
            x:self.x-rhs.x,
            y:self.y-rhs.y,
            z:self.z+rhs.y+rhs.x
        }
    }
}
impl SubAssign<Position6Axial> for Position6Cube {
    fn sub_assign(&mut self, rhs: Position6Axial) {
        self.x-=rhs.x;
        self.y-=rhs.y;
        self.z+=rhs.x+rhs.y;
    }
}

impl Neg for Position4{
    type Output=Position4;

    fn neg(self) -> Self::Output {
        Position4{
            x:!self.x,
            y:!self.y
        }
    }
}
impl Neg for Position6Axial{
    type Output=Position6Axial;

    fn neg(self) -> Self::Output {
        Position6Axial{
            x:!self.x,
            y:!self.y
        }
    }
}
impl Neg for Position6Cube{
    type Output=Position6Cube;

    fn neg(self) -> Self::Output {
        Position6Cube{
            x:!self.x,
            y:!self.y,
            z:!self.z
        }
    }
}

impl From<(i16,i16)> for Position4{
    fn from(f: (i16,i16)) -> Self {
        Position4{x:f.0,y:f.1}
    }
}
impl From<(i16,i16)> for Position6Axial{
    fn from(f: (i16,i16)) -> Self {
        Position6Axial{x:f.0,y:f.1}
    }
}

impl From<(i16,i16,i16)> for Position6Cube{
    fn from(f: (i16,i16,i16)) -> Self {
        Position6Cube{x:f.0,y:f.1,z:f.2}
    }
}

impl Digestable for Position4{
    fn update_le<D:sha2::Digest>(&self,digest:&mut D) {
        self.x.update_le(digest);
        self.y.update_le(digest);
    }

    fn update_be<D:sha2::Digest>(&self,digest:&mut D) {
        self.x.update_be(digest);
        self.y.update_be(digest)
    }
}
impl Digestable for Position6Axial{
    fn update_le<D:sha2::Digest>(&self,digest:&mut D) {
        self.x.update_le(digest);
        self.y.update_le(digest)
    }

    fn update_be<D:sha2::Digest>(&self,digest:&mut D) {
        self.x.update_be(digest);
        self.y.update_be(digest)
    }
}
impl Digestable for Position6Cube{
    fn update_le<D:sha2::Digest>(&self,digest:&mut D) {
        self.x.update_le(digest);
        self.y.update_le(digest)
    }

    fn update_be<D:sha2::Digest>(&self,digest:&mut D) {
        self.x.update_be(digest);
        self.y.update_be(digest)
    }
}
