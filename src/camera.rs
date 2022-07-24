use nalgebra::Perspective3;

pub struct Camera {
    rotation_x: f32,
    rotation_y: f32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            rotation_x: 0.,
            rotation_y: 0.,
        }
    }

    pub fn set_rotations(&mut self, rotation_x: f32, rotation_y: f32) {
        self.rotation_x = rotation_x;
        self.rotation_y = rotation_y;
    }

    pub fn get_world_to_camera(&self) -> [f32; 16] {
        const RD: f32 = std::f32::consts::PI / 180.;
        let rotation_matrix = mult_matrix_4(
            rotation_matrix_x(self.rotation_x),
            rotation_matrix_y(self.rotation_y),
        );
        let translation_matrix = translation_matrix(0., 0., -2.414213); //-1 / tan(pi/8)
        let perspective_matrix_tmp: Perspective3<f32> = Perspective3::new(1., 45. * RD, 0.01, 100.);
        let mut perspective: [f32; 16] = [0.; 16];
        perspective.copy_from_slice(perspective_matrix_tmp.as_matrix().as_slice());
        let mat1 = mult_matrix_4(rotation_matrix, translation_matrix);
        let world_to_camera = mult_matrix_4(mat1, perspective);
        world_to_camera
    }
}

fn rotation_matrix_x(angle: f32) -> [f32; 16] {
    [
        1.,
        0.,
        0.,
        0.,
        0.,
        angle.cos(),
        angle.sin(),
        0.,
        0.,
        -angle.sin(),
        angle.cos(),
        0.,
        0.,
        0.,
        0.,
        1.,
    ]
}

fn rotation_matrix_y(angle: f32) -> [f32; 16] {
    [
        angle.cos(),
        0.,
        -angle.sin(),
        0.,
        0.,
        1.,
        0.,
        0.,
        angle.sin(),
        0.,
        angle.cos(),
        0.,
        0.,
        0.,
        0.,
        1.,
    ]
}

fn translation_matrix(tx: f32, ty: f32, tz: f32) -> [f32; 16] {
    let mut return_var = [0.; 16];

    return_var[0] = 1.;
    return_var[5] = 1.;
    return_var[10] = 1.;
    return_var[15] = 1.;

    return_var[12] = tx;
    return_var[13] = ty;
    return_var[14] = tz;

    return_var
}

fn mult_matrix_4(a: [f32; 16], b: [f32; 16]) -> [f32; 16] {
    let mut return_var = [0.; 16];

    return_var[0] = a[0] * b[0] + a[1] * b[4] + a[2] * b[8] + a[3] * b[12];
    return_var[1] = a[0] * b[1] + a[1] * b[5] + a[2] * b[9] + a[3] * b[13];
    return_var[2] = a[0] * b[2] + a[1] * b[6] + a[2] * b[10] + a[3] * b[14];
    return_var[3] = a[0] * b[3] + a[1] * b[7] + a[2] * b[11] + a[3] * b[15];

    return_var[4] = a[4] * b[0] + a[5] * b[4] + a[6] * b[8] + a[7] * b[12];
    return_var[5] = a[4] * b[1] + a[5] * b[5] + a[6] * b[9] + a[7] * b[13];
    return_var[6] = a[4] * b[2] + a[5] * b[6] + a[6] * b[10] + a[7] * b[14];
    return_var[7] = a[4] * b[3] + a[5] * b[7] + a[6] * b[11] + a[7] * b[15];

    return_var[8] = a[8] * b[0] + a[9] * b[4] + a[10] * b[8] + a[11] * b[12];
    return_var[9] = a[8] * b[1] + a[9] * b[5] + a[10] * b[9] + a[11] * b[13];
    return_var[10] = a[8] * b[2] + a[9] * b[6] + a[10] * b[10] + a[11] * b[14];
    return_var[11] = a[8] * b[3] + a[9] * b[7] + a[10] * b[11] + a[11] * b[15];

    return_var[12] = a[12] * b[0] + a[13] * b[4] + a[14] * b[8] + a[15] * b[12];
    return_var[13] = a[12] * b[1] + a[13] * b[5] + a[14] * b[9] + a[15] * b[13];
    return_var[14] = a[12] * b[2] + a[13] * b[6] + a[14] * b[10] + a[15] * b[14];
    return_var[15] = a[12] * b[3] + a[13] * b[7] + a[14] * b[11] + a[15] * b[15];

    return_var
}
