mod load_mesh;
mod load_obj;
mod sort_mesh_tets;

#[allow(unused_variables)]
fn main() {
    // load mesh and obj
    let (mesh_vertices, mesh_tets) = load_mesh::load_mesh();
    let (obj_vertices, obj_faces) = load_obj::load_obj();

    // sort mesh vertices and tets
    let mut mesh_vertices_sorted = mesh_vertices.clone();
    mesh_vertices_sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mesh_tets_sorted =
        sort_mesh_tets::sort_mesh_tets(&mesh_vertices, &mesh_tets, &mesh_vertices_sorted);

    // generate the new obj_faces_tet by matching with obj_faces
    let mut obj_faces_new = obj_faces.clone();
    for (i, face) in obj_faces.iter().enumerate() {
        obj_faces_new[i] = [
            mesh_vertices_sorted
                .binary_search_by(|v| v.partial_cmp(&obj_vertices[face[0] - 1]).unwrap())
                .unwrap(),
            mesh_vertices_sorted
                .binary_search_by(|v| v.partial_cmp(&obj_vertices[face[1] - 1]).unwrap())
                .unwrap(),
            mesh_vertices_sorted
                .binary_search_by(|v| v.partial_cmp(&obj_vertices[face[2] - 1]).unwrap())
                .unwrap(),
        ];
    }
    // println!("{:?}", mesh_vertices_sorted.concat().len());
    // println!("{:?}", mesh_tets_sorted.len());
    // println!("{:?}", obj_faces_new.len());

    println!("{:?}", mesh_vertices_sorted);
    // println!("{:?}", mesh_tets_sorted);
    // println!("{:?}", obj_faces_new);
}
