use animation::skeleton::Skeleton;
use animation::clip::Clip;

/// Pure animation data, separate from geometry
#[derive(Clone)]
pub struct CPUSkeleton {
    pub skeleton: Option<Skeleton>,
    pub clips: Vec<Clip>,
    pub name: Option<String>,
}

impl CPUSkeleton {
    pub fn new() -> Self {
        Self {
            skeleton: None,
            clips: Vec::new(),
            name: None,
        }
    }

    pub fn with_skeleton(mut self, skeleton: Skeleton) -> Self {
        self.skeleton = Some(skeleton);
        self
    }

    pub fn add_clip(&mut self, clip: Clip) {
        self.clips.push(clip);
    }

    pub fn has_skeleton(&self) -> bool {
        self.skeleton.is_some()
    }

    pub fn clip_count(&self) -> usize {
        self.clips.len()
    }

    pub fn get_clip(&self, index: usize) -> Option<&Clip> {
        self.clips.get(index)
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }
}
