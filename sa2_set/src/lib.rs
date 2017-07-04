extern crate byteorder;

use std::io::{self, Read};

use byteorder::{ReadBytesExt, ByteOrder, LittleEndian, BigEndian};

#[derive(Clone,Copy,Debug)]
pub enum Object {
    Emerald,
    EmeraldF,
    Unknown(u16),
}

impl Object {
    fn from_u16(num: u16) -> Object {
        match num {
            0x000f => Object::Emerald,
            0x0052 => Object::EmeraldF,
            _ => Object::Unknown(num),
        }
    }
}

#[derive(Clone,Copy,Debug)]
pub struct Rotation {
    pub x: u16,
    pub y: u16,
    pub z: u16,
}

impl Rotation {
    fn from_read<R, E>(readable: &mut R) -> io::Result<Rotation>
        where R: Read,
              E: ByteOrder,
    {
        let x = readable.read_u16::<E>()?;
        let y = readable.read_u16::<E>()?;
        let z = readable.read_u16::<E>()?;

        Ok(Rotation {
            x: x,
            y: y,
            z: z,
        })
    }
}

#[derive(Clone,Copy,Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Position {
    fn from_read<R, E>(readable: &mut R) -> io::Result<Position>
        where R: Read,
              E: ByteOrder,
    {
        let x = readable.read_f32::<E>()?;
        let y = readable.read_f32::<E>()?;
        let z = readable.read_f32::<E>()?;

        Ok(Position {
            x: x,
            y: y,
            z: z,
        })
    }
}

#[derive(Clone,Copy,Debug)]
pub struct SetObject {
    pub object: Object,
    pub rotation: Rotation,
    pub position: Position,
    pub attr1: u32,
    pub attr2: u32,
    pub attr3: u32,
}

impl SetObject {
    fn from_read<R, E>(readable: &mut R) -> io::Result<SetObject>
        where R: Read,
              E: ByteOrder,
    {
        let raw_obj_clip = readable.read_u16::<E>()?;
        let object = Object::from_u16(raw_obj_clip);
        let rotation = Rotation::from_read::<R, E>(readable)?;
        let position = Position::from_read::<R, E>(readable)?;
        let attr1 = readable.read_u32::<E>()?;
        let attr2 = readable.read_u32::<E>()?;
        let attr3 = readable.read_u32::<E>()?;

        Ok(SetObject {
            object: object,
            rotation: rotation,
            position: position,
            attr1: attr1,
            attr2: attr2,
            attr3: attr3,
        })
    }
}

#[derive(Clone,Debug)]
pub struct SetFile {
    objects: Vec<SetObject>,
}

impl SetFile {
    pub fn from_read<P, R>(readable: &mut R) -> io::Result<SetFile>
        where R: Read,
              P: Platform
    {
        let num_objects = readable.read_u32::<P::Endianess>()?;

        // TODO: XXX
        readable.read_u32::<P::Endianess>()?;
        readable.read_u32::<P::Endianess>()?;
        readable.read_u32::<P::Endianess>()?;
        readable.read_u32::<P::Endianess>()?;
        readable.read_u32::<P::Endianess>()?;
        readable.read_u32::<P::Endianess>()?;
        readable.read_u32::<P::Endianess>()?;

        let mut objects = Vec::with_capacity(num_objects as usize);

        for _ in 0..num_objects {
            objects.push(SetObject::from_read::<_, P::Endianess>(readable)?);
        }

        Ok(SetFile {
            objects: objects,
        })
    }

    pub fn into_vec(self) -> Vec<SetObject> {
        self.objects
    }
}

pub trait Platform {
    type Endianess: ByteOrder;
}

pub struct Dreamcast;

impl Platform for Dreamcast {
    type Endianess = LittleEndian;
}

pub struct Pc;

impl Platform for Pc {
    type Endianess = BigEndian;
}

pub struct GameCube;

impl Platform for GameCube {
    type Endianess = BigEndian;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
