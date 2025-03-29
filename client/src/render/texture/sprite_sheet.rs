use nalgebra::Vector2;
extern crate transform;

/*
 * The spritesheet is a collection of cells
 * It acts as a lookup table for getting the frame corresponding to the orientation
 * and progress in an animation
 */
pub struct SpriteSheet {
    /*
     * first index is dir
     * second index is animation frame
     *
     * dir_index[0][3] is the third frame of dir 0
     *
     */
    dir_index: Vec<Vec<transform::Rect>>,
    start_dir: Vector2<f32>,
    per_index_angle: f32,
}

impl SpriteSheet {
    pub fn FromRegularGrid(w_count: u8, h_count: u8, image_width: u32, image_height: u32) -> Self {
        let cell_height = image_height as f32 / h_count as f32;
        let cell_width = image_width as f32 / w_count as f32;
        //TODO(oleg): pre allocate
        let mut dir_index = Vec::new();
        for i in 0..h_count {
            dir_index.push(Vec::new());
        }

        for x in 0..w_count {
            for y in 0..h_count {
                let cell = transform::Rect {
                    left: x as f32 * cell_width,
                    right: x as f32 * (cell_width + 1.0),
                    bottom: y as f32 * cell_height,
                    top: y as f32 * (cell_width + 1.0),
                };
                dir_index[y as usize].push(cell);
            }
        }

        return Self {
            dir_index: dir_index,
            start_dir: Vector2::new(0.0, -1.0),
            per_index_angle: 360.0 / h_count as f32,
        };
    }

    /*
     * Returns the cell correspoding to the specificed direction the
     * character is facing in, and progress % of the way in, with 0 being the
     * start of the animation, and 1 being the end of the animation
     *
     * progress must be from 0-1
     */
    pub fn GetCell(&self, dir: &Vector2<f32>, progress: f32) -> &transform::Rect {
        /*
         * Take the dot product of the dir and the start dir to find the angle
         * We know we take equal steps so we know the angle per index
         * Find the closes index by doing modulus
         */
        let angle = self.start_dir.dot(dir).to_degrees();
        let dir_index = (angle / self.per_index_angle) as usize;
        let anim_index = (progress * self.dir_index[dir_index].len() as f32) as usize;
        return &self.dir_index[dir_index][anim_index];
    }
}
