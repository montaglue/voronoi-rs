use super::plane::Plane;

pub struct HalfEdge {
    end_vertex: usize,
    opp: usize,
    face: usize,
    next: usize,
}

impl HalfEdge {
    fn disable(&mut self) {
        self.end_vertex = usize::MAX;
    }

    fn is_disabled(&self) -> bool {
        self.end_vertex == usize::MAX
    }
}

pub struct Face {
    he: usize,
    plane: Option<Plane<f64>>,
    diameter: f64,
    visibility_checked_on_iteration: usize,
    is_visible_face_on_current_iteration: u8,
    in_face_stack: u8,
    horizon_edges_on_current_iteration: u8,
    point_on_positive_side: Vec<usize>,
}

impl Face {
    pub fn new() -> Self {
        Face {
            he: usize::MAX,
            plane: None,
            diameter: 0.0,
            visibility_checked_on_iteration: 0,
            is_visible_face_on_current_iteration: 0,
            in_face_stack: 0,
            horizon_edges_on_current_iteration: 0,
            point_on_positive_side: Vec::new(),
        }
    }

    pub fn disable(&mut self) {
        self.he = usize::MAX;
    }

    pub fn is_disabled(&self) -> bool {
        self.he == usize::MAX
    }
}

pub struct Mesh {
    faces: Vec<Face>,
    disabled_faces: Vec<usize>,
    half_edges: Vec<HalfEdge>,
    disabled_edges: Vec<usize>,
}

impl Mesh {
    pub fn add_face(&mut self) -> usize {
        todo!()
    }

    pub fn add_half_edge(&mut self) -> usize {
        todo!()
    }
}