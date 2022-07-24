pub fn sort_mesh_tets(
  mesh_vertices: &Vec<[f64; 3]>,
  mesh_tets: &Vec<[usize; 4]>,
  mesh_vertices_sorted: &Vec<[f64; 3]>,
) -> Vec<[usize; 4]> {
  let mut mesh_tets_sorted = mesh_tets.clone();
  for (i, tet) in mesh_tets.iter().enumerate() {
    // the tet indeces start from 1
    mesh_tets_sorted[i] = [
      mesh_vertices_sorted
        .binary_search_by(|v| v.partial_cmp(&mesh_vertices[tet[0] - 1]).unwrap())
        .unwrap(),
      mesh_vertices_sorted
        .binary_search_by(|v| v.partial_cmp(&mesh_vertices[tet[1] - 1]).unwrap())
        .unwrap(),
      mesh_vertices_sorted
        .binary_search_by(|v| v.partial_cmp(&mesh_vertices[tet[2] - 1]).unwrap())
        .unwrap(),
      mesh_vertices_sorted
        .binary_search_by(|v| v.partial_cmp(&mesh_vertices[tet[3] - 1]).unwrap())
        .unwrap(),
    ];
  }
  mesh_tets_sorted
}
