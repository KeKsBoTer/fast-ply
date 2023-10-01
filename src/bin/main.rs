use std::{io::BufReader, path::Path};

use byteorder::LittleEndian;
use fast_ply::PlyElement;
use fast_ply_derive::PlyElement;

#[derive(Debug, PlyElement)]
struct Vertex {
    // #[ply(x, y, z)]
    pos: [f32; 3],

    // #[ply(nx, ny, nz)]
    normal: [f32; 3],

    // #[ply(f_dc)]
    f_dc: [f32; 3],

    // #[ply(f_rest)]
    f_rest: [f32; 45],

    opacity: f32,

    // #[ply(scale)]
    scale: [f32; 3],

    // #[ply(rot)]
    rot: [f32; 4],
}

fn main() {
    let path = Path::new("point_cloud_30000.ply");
    let f = std::fs::File::open(path).unwrap();
    let mut reader = BufReader::new(f);

    let p = ply_rs::parser::Parser::<ply_rs::ply::DefaultElement>::new();
    let header = p.read_header(&mut reader).unwrap();
    let v = <Vertex as PlyElement>::read::<LittleEndian, _>(&mut reader).unwrap();

    println!("{v:?}");
}
