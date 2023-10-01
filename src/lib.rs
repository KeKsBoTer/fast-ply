use byteorder::{ByteOrder, ReadBytesExt};
// use cgmath::{Point3, Quaternion, Vector3, Vector4};
use std::{
    io::{self, BufReader},
    mem::MaybeUninit,
};

pub trait PlyElement: Sized {
    fn read<B: ByteOrder, R: io::Read>(reader: &mut BufReader<R>) -> std::io::Result<Self>;
}

pub trait PlyProperty: Sized {
    fn read<B: ByteOrder, R: io::Read>(reader: &mut BufReader<R>) -> std::io::Result<Self>;
    fn read_into<B: ByteOrder, R: io::Read>(
        reader: &mut BufReader<R>,
        target: &mut [Self],
    ) -> std::io::Result<()> {
        for i in 0..target.len() {
            target[i] = Self::read::<B, _>(reader)?;
        }
        Ok(())
    }
}

impl PlyProperty for f32 {
    #[inline(always)]
    fn read<B: ByteOrder, R: io::Read>(reader: &mut BufReader<R>) -> std::io::Result<Self> {
        reader.read_f32::<B>()
    }

    fn read_into<B: ByteOrder, R: io::Read>(
        reader: &mut BufReader<R>,
        target: &mut [Self],
    ) -> std::io::Result<()> {
        reader.read_f32_into::<B>(target)?;
        Ok(())
    }
}

impl<const N: usize, T: PlyProperty + Sized> PlyProperty for [T; N] {
    #[inline(always)]
    fn read<B: ByteOrder, R: io::Read>(reader: &mut BufReader<R>) -> std::io::Result<Self> {
        let mut result: [T; N] = unsafe { MaybeUninit::uninit().assume_init() };
        T::read_into::<B, _>(reader, &mut result)?;
        return Ok(result);
    }
}

#[cfg(feature = "cgmath")]
impl<F: PlyProperty + Clone> PlyProperty for Vector3<F> {
    #[inline(always)]
    fn read<B: ByteOrder, R: io::Read>(reader: &mut BufReader<R>) -> std::io::Result<Self> {
        Ok(<[F; 3]>::read::<B, _>(reader)?.into())
    }
}

#[cfg(feature = "cgmath")]
impl<F: PlyProperty + Clone> PlyProperty for Vector4<F> {
    #[inline(always)]
    fn read<B: ByteOrder, R: io::Read>(reader: &mut BufReader<R>) -> std::io::Result<Self> {
        Ok(<[F; 4]>::read::<B, _>(reader)?.into())
    }
}
#[cfg(feature = "cgmath")]
impl<F: PlyProperty + Clone> PlyProperty for Point3<F> {
    #[inline(always)]
    fn read<B: ByteOrder, R: io::Read>(reader: &mut BufReader<R>) -> std::io::Result<Self> {
        Ok(<[F; 3]>::read::<B, _>(reader)?.into())
    }
}
