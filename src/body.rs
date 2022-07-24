extern crate nalgebra as na;

pub struct Body {
    pub tets: na::OMatrix<usize, na::Const<617>, na::Const<4>>,
    pub faces: na::OMatrix<usize, na::Const<576>, na::Const<3>>,
    pub positions_init: na::OMatrix<f32, na::Const<909>, na::Const<1>>,
    pub positions: na::OMatrix<f32, na::Const<909>, na::Const<1>>,
    pub velocities: na::OMatrix<f32, na::Const<909>, na::Const<1>>,
    pub forces: na::OMatrix<f32, na::Const<909>, na::Const<1>>,
}

impl Body {
    pub fn new(
        positions_int: na::OMatrix<f32, na::Const<909>, na::Const<1>>,
        tets: na::OMatrix<usize, na::Const<617>, na::Const<4>>,
        faces: na::OMatrix<usize, na::Const<576>, na::Const<3>>,
    ) -> Self {
        let mut positions_int = positions_int;
        transform(&mut positions_int);
        let mut zeros = positions_int.clone();
        zeros.fill(0.0);
        Self {
            tets: tets,
            faces: faces,
            positions_init: positions_int,
            positions: positions_int,
            velocities: zeros,
            forces: zeros,
        }
    }

    pub fn generate_vertices(&self) -> Vec<f32> {
        let mut gen_vertices = vec![];
        for face in self.faces.row_iter() {
            for index in face.iter() {
                let mut vertix = vec![
                    self.positions[index * 3],
                    self.positions[index * 3 + 1],
                    self.positions[index * 3 + 2],
                ];
                gen_vertices.append(&mut vertix);
            }
        }
        gen_vertices
    }

    pub fn generate_normals(&self) -> Vec<f32> {
        let mut gen_normals = vec![];
        for face in self.faces.row_iter() {
            let v1 = na::Vector3::new(
                self.positions[face[0] * 3],
                self.positions[face[0] * 3 + 1],
                self.positions[face[0] * 3 + 2],
            );
            let v2 = na::Vector3::new(
                self.positions[face[1] * 3],
                self.positions[face[1] * 3 + 1],
                self.positions[face[1] * 3 + 2],
            );
            let v3 = na::Vector3::new(
                self.positions[face[2] * 3],
                self.positions[face[2] * 3 + 1],
                self.positions[face[2] * 3 + 2],
            );
            let v4 = (v3 - v1).cross(&(v3 - v2));
            for _index in face.iter() {
                let mut normal = vec![v4[0], v4[1], v4[2]];
                gen_normals.append(&mut normal);
            }
        }
        gen_normals
    }

    #[allow(dead_code)]
    pub fn shink(&mut self) {
        for i in 0..(self.positions.len() / 3) {
            self.positions[i * 3 + 0] = 0.;
            self.positions[i * 3 + 1] = 0.;
            self.positions[i * 3 + 2] = 0.251;
        }
    }
}

fn transform(
    positions: &mut na::Matrix<
        f32,
        na::Const<909_usize>,
        na::Const<1_usize>,
        na::ArrayStorage<f32, 909_usize, 1_usize>,
    >,
) {
    for i in 0..(positions.len() / 3) {
        let mut vec = na::vector![0., 0., 0.];
        vec[0] = positions[i * 3 + 0];
        vec[1] = positions[i * 3 + 1];
        vec[2] = positions[i * 3 + 2];

        let scale_value = 0.1;
        let scale_mat = na::matrix![scale_value,0.,0.;
                                0.,scale_value,0.;
                                0.,0.,scale_value;];

        let axis = na::Vector3::x_axis();
        let rotation_mat = na::Rotation3::from_axis_angle(&axis, 0. * std::f32::consts::PI / 180.);

        let translation_vec = na::vector![0., 0., 0.0];
        vec = scale_mat * vec; // scale
        vec = rotation_mat * vec; // rotate
        vec = translation_vec + vec; // translate
        positions[i * 3 + 0] = vec[0];
        positions[i * 3 + 1] = vec[1];
        positions[i * 3 + 2] = vec[2];
    }
}
