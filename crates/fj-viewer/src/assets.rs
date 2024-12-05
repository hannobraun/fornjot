pub struct Assets<'a> {
    pub cube_obj: &'a [u8],
    pub cube_mtl: &'a [u8],
    pub front_texture: &'a [u8],
    pub right_texture: &'a [u8],
    pub rear_texture: &'a [u8],
    pub left_texture: &'a [u8],
    pub top_texture: &'a [u8],
    pub bottom_texture: &'a [u8],
}

impl Assets<'_> {
    pub fn get_instance() -> Self {
        let cube_obj: &[u8] =
            include_bytes!("../assets/navigation_cube/cube.obj");
        let cube_mtl: &[u8] =
            include_bytes!("../assets/navigation_cube/cube.mtl");
        let front_texture: &[u8] =
            include_bytes!("../assets/navigation_cube/front.png");
        let right_texture: &[u8] =
            include_bytes!("../assets/navigation_cube/right.png");
        let rear_texture: &[u8] =
            include_bytes!("../assets/navigation_cube/rear.png");
        let left_texture: &[u8] =
            include_bytes!("../assets/navigation_cube/left.png");
        let top_texture: &[u8] =
            include_bytes!("../assets/navigation_cube/top.png");
        let bottom_texture: &[u8] =
            include_bytes!("../assets/navigation_cube/bottom.png");

        Self {
            cube_obj,
            cube_mtl,
            front_texture,
            right_texture,
            rear_texture,
            left_texture,
            top_texture,
            bottom_texture,
        }
    }

    pub fn get_asset(&self, file_name: &str) -> &[u8] {
        match file_name {
            "cube.obj" => self.cube_obj,
            "cube.mtl" => self.cube_mtl,
            "front.png" => self.front_texture,
            "right.png" => self.right_texture,
            "rear.png" => self.rear_texture,
            "left.png" => self.left_texture,
            "top.png" => self.top_texture,
            "bottom.png" => self.bottom_texture,
            _ => unreachable!(
                "An unknown asset: {} is trying to be loaded",
                file_name
            ),
        }
    }
}
