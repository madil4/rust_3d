use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

pub fn load_mesh() -> (Vec<[f64; 3]>, Vec<[usize; 4]>) {
  let path = Path::new("tree.mesh");
  let file = match File::open(&path) {
    Err(why) => panic!("couldn't open {}", why),
    Ok(file) => file,
  };
  let reader = BufReader::new(file);
  let mut num_of_vert: usize = 0;
  let mut num_of_tets: usize = 0;
  let mut vertices: Vec<[f64; 3]> = vec![[0.; 3]; 0];
  let mut tets: Vec<[usize; 4]> = vec![[0; 4]; 0];
  let presesion: f64 = 1e11;

  for (i, line) in reader.lines().enumerate() {
    // reand number of vertices
    let line_clone = line.unwrap().clone();
    if i == 3 {
      num_of_vert = line_clone.parse().unwrap();
      // println!("{:?}", num_of_vert);
    }
    // read vertices
    if i > 3 && i <= num_of_vert + 3 {
      let vertix: Vec<&str> = line_clone.split(" ").collect();
      // print!("\n{:?}", vertix);
      let n0: f64 = vertix[0].parse().unwrap();
      let n1: f64 = vertix[1].parse().unwrap();
      let n2: f64 = vertix[2].parse().unwrap();
      vertices.append(&mut vec![[
        (n0 * presesion).round() / presesion,
        (n1 * presesion).round() / presesion,
        (n2 * presesion).round() / presesion,
      ]]);
    }
    // read number of tets
    if i == num_of_vert + 3 + 4 {
      num_of_tets = line_clone.parse().unwrap();
      // println!("{:?}", num_of_tets);
    }
    // read tets
    if i > num_of_vert + 3 + 4 && i <= 3 + num_of_vert + 4 + num_of_tets {
      let tetrahedron: Vec<&str> = line_clone.split(" ").collect();
      // print!("\n{:?}", tetrahedron);
      tets.append(&mut vec![[
        tetrahedron[0].parse().unwrap(),
        tetrahedron[1].parse().unwrap(),
        tetrahedron[2].parse().unwrap(),
        tetrahedron[3].parse().unwrap(),
      ]]);
    }
  }
  (vertices, tets)
}
