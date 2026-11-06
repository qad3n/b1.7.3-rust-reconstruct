use crate::nbt::NBTBase;
use jvm::io::{DataInputStream, DataOutputStream, InputStream, IoResult, OutputStream};
use jvm::throwable;
use jvm::zip::{GZIPInputStream, GZIPOutputStream};
use std::rc::Rc;

pub struct CompressedStreamTools;

impl CompressedStreamTools
{
    pub fn readCompressed<S: InputStream>(input: S) -> IoResult<Rc<NBTBase>>
    {
        let mut datainputstream: DataInputStream<GZIPInputStream<S>> =
            DataInputStream::new(GZIPInputStream::new(input)?);
        let result: IoResult<Rc<NBTBase>> = Self::read(&mut datainputstream);
        let closed: IoResult<()> = datainputstream.m_in.close();
        let tag: Rc<NBTBase> = result?;
        closed?;
        Ok(tag)
    }

    pub fn writeGzippedCompoundToOutputStream<S: OutputStream>(nbt: &Rc<NBTBase>, output: S) -> IoResult<()>
    {
        let mut dataoutputstream: DataOutputStream<GZIPOutputStream<S>> =
            DataOutputStream::new(GZIPOutputStream::new(output)?);
        let result: IoResult<()> = Self::write(nbt, &mut dataoutputstream);
        let closed: IoResult<()> = dataoutputstream.close();
        result?;
        closed
    }

    pub fn read<S: InputStream>(input: &mut DataInputStream<S>) -> IoResult<Rc<NBTBase>>
    {
        let nbtbase: Rc<NBTBase> = NBTBase::readTag(input)?;
        if nbtbase.getType() == 10
        {
            return Ok(nbtbase);
        }
        Err(jvm::io::ioError(&throwable::IO, "Root tag must be a named compound tag"))
    }

    pub fn write<S: OutputStream>(nbt: &Rc<NBTBase>, output: &mut DataOutputStream<S>) -> IoResult<()>
    {
        NBTBase::writeTag(nbt, output)
    }
}
