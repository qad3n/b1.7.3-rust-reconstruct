pub struct SoundSystem;

#[allow(clippy::too_many_arguments)]
impl SoundSystem
{
    pub fn new() -> SoundSystem { SoundSystem }

    pub fn cleanup(&self) {}

    pub fn playing(&self, _sourcename: &str) -> bool { false }

    pub fn play(&self, _sourcename: &str) {}

    pub fn stop(&self, _sourcename: &str) {}

    pub fn setVolume(&self, _sourcename: &str, _value: f32) {}

    pub fn setPitch(&self, _sourcename: &str, _value: f32) {}

    pub fn backgroundMusic(&self, _sourcename: &str, _url: &str, _identifier: &str, _toLoop: bool) {}

    pub fn newSource(
        &self,
        _priority: bool,
        _sourcename: &str,
        _url: &str,
        _identifier: &str,
        _toLoop: bool,
        _x: f32,
        _y: f32,
        _z: f32,
        _attModel: i32,
        _distOrRoll: f32,
    )
    {
    }

    pub fn newStreamingSource(
        &self,
        _priority: bool,
        _sourcename: &str,
        _url: &str,
        _identifier: &str,
        _toLoop: bool,
        _x: f32,
        _y: f32,
        _z: f32,
        _attModel: i32,
        _distOrRoll: f32,
    )
    {
    }

    pub fn setListenerPosition(&self, _x: f32, _y: f32, _z: f32) {}

    pub fn setListenerOrientation(&self, _lookX: f32, _lookY: f32, _lookZ: f32, _upX: f32, _upY: f32, _upZ: f32) {}
}

impl Default for SoundSystem
{
    fn default() -> Self { Self::new() }
}
