pub struct Grid {
    size: f32,
    divisin: i32,
}

impl Grid {
    pub fn new() -> Self {
        Self {
            size: 5.,
            divisin: 40,
        }
    }

    pub fn generate_vertices(&self) -> Vec<f32> {
        let dt: f32 = 2. / self.divisin as f32;
        let mut gen_vertices = vec![];
        for i in 0..self.divisin + 1 {
            let sc = self.size - (i as f32 * dt * self.size);
            let mut x = vec![sc, self.size, 0., sc, -self.size, 0.];
            gen_vertices.append(&mut x);
            let mut y = vec![self.size, sc, 0., -self.size, sc, 0.];
            gen_vertices.append(&mut y);
        }
        gen_vertices
    }

    pub fn generate_normals(&self) -> Vec<f32> {
        let mut gen_normals = vec![];
        for _i in 0..((self.divisin + 1) * 2 * 2) {
            let mut normal = vec![0., 0., 1.];
            gen_normals.append(&mut normal);
        }
        gen_normals
    }
}
