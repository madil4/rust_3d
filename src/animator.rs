extern crate nalgebra as na;
use crate::body::Body;

pub struct Animator {}

impl<'a> Animator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn aimate(&self, body: &'a mut Body) -> &'a Body {
        let dt = 0.033;
        let xn = body.positions.clone();
        body.forces.fill(0.0);
        compute_gravity_forces(body);
        compute_damping_forces(body);
        compute_elastic_forces(body);
        for i in 0..body.positions.len() {
            body.positions[i] += body.velocities[i] * dt + body.forces[i] * dt.powf(2.);
            body.velocities[i] = (body.positions[i] - xn[i]) / dt
        }
        // clamp_bottom(body);
        body
    }
}

fn compute_gravity_forces(body: &mut Body) {
    for i in 0..(body.forces.len() / 3) {
        body.forces[i * 3 + 2] += -0.;
    }
}

fn compute_damping_forces(body: &mut Body) {
    for i in 0..(body.forces.len()) {
        body.forces[i] += -body.velocities[i] * 0.2;
    }
}

fn compute_elastic_forces(body: &mut Body) {
    let mut forces = na::DVector::from_element(body.positions.len(), 0.);
    for tet in body.tets.row_iter() {
        // compute rest shape
        let n1 = body.positions_init.slice((tet[0] * 3, 0), (3, 1));
        let n2 = body.positions_init.slice((tet[1] * 3, 0), (3, 1));
        let n3 = body.positions_init.slice((tet[2] * 3, 0), (3, 1));
        let n4 = body.positions_init.slice((tet[3] * 3, 0), (3, 1));
        let mut dm = na::DMatrix::from_element(3, 3, 0.);
        let n14 = n1 - n4;
        let n24 = n2 - n4;
        let n34 = n3 - n4;
        dm.set_column(0, &na::Vector3::new(n14[0], n14[1], n14[2]));
        dm.set_column(1, &na::Vector3::new(n24[0], n24[1], n24[2]));
        dm.set_column(2, &na::Vector3::new(n34[0], n34[1], n34[2]));
        // compute current shape
        let n1 = body.positions.slice((tet[0] * 3, 0), (3, 1));
        let n2 = body.positions.slice((tet[1] * 3, 0), (3, 1));
        let n3 = body.positions.slice((tet[2] * 3, 0), (3, 1));
        let n4 = body.positions.slice((tet[3] * 3, 0), (3, 1));
        let mut ds = na::DMatrix::from_element(3, 3, 0.);
        let n14 = n1 - n4;
        let n24 = n2 - n4;
        let n34 = n3 - n4;
        ds.set_column(0, &na::Vector3::new(n14[0], n14[1], n14[2]));
        ds.set_column(1, &na::Vector3::new(n24[0], n24[1], n24[2]));
        ds.set_column(2, &na::Vector3::new(n34[0], n34[1], n34[2]));
        // compute stress
        let k = 5000.;
        let dm_inv = dm.clone().try_inverse().unwrap();
        let f = ds * dm_inv;
        let f_t = f.transpose();
        let e = 0.5 * (f + f_t) - na::Matrix3::identity();
        let p = k * e;
        // compute forces
        let w = f32::abs(-0.5 * dm.determinant());
        let dm_inv = dm.clone().try_inverse().unwrap();
        let h = -p * dm_inv.transpose() * w;
        let f1_h = h.column(0);
        let f2_h = h.column(1);
        let f3_h = h.column(2);
        let f4_h = -f1_h - f2_h - f3_h;
        // update forces
        let mut f1 = forces.slice_mut((tet[0] * 3, 0), (3, 1));
        f1.set_column(0, &(&f1 + &na::vector![f1_h[0], f1_h[1], f1_h[2]]));
        let mut f2 = forces.slice_mut((tet[1] * 3, 0), (3, 1));
        f2.set_column(0, &(&f2 + &na::vector![f2_h[0], f2_h[1], f2_h[2]]));
        let mut f3 = forces.slice_mut((tet[2] * 3, 0), (3, 1));
        f3.set_column(0, &(&f3 + &na::vector![f3_h[0], f3_h[1], f3_h[2]]));
        let mut f4 = forces.slice_mut((tet[3] * 3, 0), (3, 1));
        f4.set_column(0, &(&f4 + &na::vector![f4_h[0], f4_h[1], f4_h[2]]));
    }
    for i in 0..body.forces.len() {
        body.forces[i] += forces[i];
    }
}

#[allow(dead_code)]
fn clamp_bottom(body: &mut Body) {
    for i in 0..(body.positions.len() / 3) {
        if body.positions[i * 3 + 2] < 0.0 {
            body.positions[i * 3 + 2] = 0.0;
        }
    }
}
