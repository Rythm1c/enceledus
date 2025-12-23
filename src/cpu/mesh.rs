use super::primitive::Primitive;

#[derive(Clone, Default)]
pub struct Mesh {
    pub primitives: Vec<Primitive>,
}
impl Mesh {
    pub fn new() -> Self {
        Self {
            primitives: Vec::new(),
        }
    }

    pub fn index_count(&self) -> usize {
        self.primitives
            .iter()
            .map(|primitive| primitive.index_count())
            .sum()
    }

    pub fn vertex_count(&self) -> usize {
        self.primitives
            .iter()
            .map(|primitive| primitive.vertex_count())
            .sum()
    }

    pub fn get_bounds(&self) -> (f32, f32) {
        let bounds = self
            .primitives
            .iter()
            .map(|primitive| primitive.get_bounds());

        let minimum = bounds
            .clone()
            .map(|bound| bound.0)
            .reduce(f32::min)
            .expect("could not get mesh minimum");

        let maximum = bounds
            .map(|bound| bound.1)
            .reduce(f32::max)
            .expect("could not get mesh maximum");

        (minimum, maximum)
    }
}
