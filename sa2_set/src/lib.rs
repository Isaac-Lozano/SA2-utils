extern crate byteorder;
#[macro_use] extern crate serde_derive;

use std::io::{self, Read, Write};

use byteorder::{ReadBytesExt, WriteBytesExt, ByteOrder, LittleEndian, BigEndian};

#[derive(Clone,Copy,Debug,Default,Serialize,Deserialize)]
pub struct Object(pub u16);

#[derive(Clone,Copy,Debug,Default,Serialize,Deserialize)]
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

    pub fn write_data<P, W>(&self, writeable: &mut W) -> io::Result<()>
        where P: Platform,
              W: Write,
    {
        writeable.write_u16::<P::Endianess>(self.x)?;
        writeable.write_u16::<P::Endianess>(self.y)?;
        writeable.write_u16::<P::Endianess>(self.z)?;
        Ok(())
    }
}

#[derive(Clone,Copy,Debug,Default,Serialize,Deserialize)]
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

    pub fn write_data<P, W>(&self, writeable: &mut W) -> io::Result<()>
        where P: Platform,
              W: Write,
    {
        writeable.write_f32::<P::Endianess>(self.x)?;
        writeable.write_f32::<P::Endianess>(self.y)?;
        writeable.write_f32::<P::Endianess>(self.z)?;
        Ok(())
    }
}

#[derive(Clone,Copy,Debug,Default,Serialize,Deserialize)]
pub struct SetObject {
    pub object: Object,
    pub rotation: Rotation,
    pub position: Position,
    pub attr1: f32,
    pub attr2: f32,
    pub attr3: f32,
}

impl SetObject {
    fn from_read<R, E>(readable: &mut R) -> io::Result<SetObject>
        where R: Read,
              E: ByteOrder,
    {
        let object = Object(readable.read_u16::<E>()?);
        let rotation = Rotation::from_read::<R, E>(readable)?;
        let position = Position::from_read::<R, E>(readable)?;
        let attr1 = readable.read_f32::<E>()?;
        let attr2 = readable.read_f32::<E>()?;
        let attr3 = readable.read_f32::<E>()?;

        Ok(SetObject {
            object: object,
            rotation: rotation,
            position: position,
            attr1: attr1,
            attr2: attr2,
            attr3: attr3,
        })
    }

    pub fn write_data<P, W>(&self, writeable: &mut W) -> io::Result<()>
        where P: Platform,
              W: Write,
    {
        writeable.write_u16::<P::Endianess>(self.object.0)?;
        self.rotation.write_data::<P, _>(writeable)?;
        self.position.write_data::<P, _>(writeable)?;
        writeable.write_f32::<P::Endianess>(self.attr1)?;
        writeable.write_f32::<P::Endianess>(self.attr2)?;
        writeable.write_f32::<P::Endianess>(self.attr3)?;
        Ok(())
    }
}

#[derive(Clone,Debug,Default,Serialize,Deserialize)]
pub struct SetFile(pub Vec<SetObject>);

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

        let mut objects = Vec::new();

        for _ in 0..num_objects {
            objects.push(SetObject::from_read::<_, P::Endianess>(readable)?);
        }

        Ok(SetFile(objects))
    }

    pub fn write_data<P, W>(&self, writeable: &mut W) -> io::Result<()>
        where P: Platform,
              W: Write,
    {
        writeable.write_u32::<P::Endianess>(self.0.len() as u32)?;

        // TODO: XXX
        writeable.write_u32::<P::Endianess>(0)?;
        writeable.write_u32::<P::Endianess>(0)?;
        writeable.write_u32::<P::Endianess>(0)?;
        writeable.write_u32::<P::Endianess>(0)?;
        writeable.write_u32::<P::Endianess>(0)?;
        writeable.write_u32::<P::Endianess>(0)?;
        writeable.write_u32::<P::Endianess>(0)?;

        for object in self.0.iter() {
            object.write_data::<P, _>(writeable)?;
        }

        Ok(())
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
