use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

pub fn load_obj() -> (Vec<[f64; 3]>, Vec<[usize; 3]>) {
  let path = Path::new("tree.obj");
  let file = match File::open(&path) {
    Err(why) => panic!("couldn't open {}", why),
    Ok(file) => file,
  };
  let reader = BufReader::new(file);
  let mut vertices: Vec<[f64; 3]> = vec![[0.; 3]; 0];
  let mut faces: Vec<[usize; 3]> = vec![[0; 3]; 0];
  let presesion: f64 = 1e11;

  for line in reader.lines() {
    let line_clone = line.unwrap().clone();

    let line_as_str: Vec<&str> = line_clone.split(" ").collect();
    if line_as_str[0] == "v" {
      // let n0: f64 = line_as_str[0].parse().unwrap(); = "v"
      let n1: f64 = line_as_str[1].parse().unwrap();
      let n2: f64 = line_as_str[2].parse().unwrap();
      let n3: f64 = line_as_str[3].parse().unwrap();
      vertices.append(&mut vec![[
        (n1 * presesion).round() / presesion,
        (n2 * presesion).round() / presesion,
        (n3 * presesion).round() / presesion,
      ]]);
    }

    if line_as_str[0] == "f" {
      faces.append(&mut vec![[
        line_as_str[1].parse().unwrap(),
        line_as_str[2].parse().unwrap(),
        line_as_str[3].parse().unwrap(),
      ]]);
    }
  }
  (vertices, faces)
}
