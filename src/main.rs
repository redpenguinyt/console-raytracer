use gemini_engine::elements::{
    view::{ColChar, View, Wrapping},
    Vec2D,
};
use gemini_engine::elements3d::Vec3D;
use raytracing::{Colour, RayScene, RaySphere};

const VIEW_WIDTH: f64 = 1.0;
const VIEW_HEIGHT: f64 = 1.0;
const VIEW_DEPTH: f64 = 1.0;

fn main() {
    let mut canvas = View::new(500, 180, ColChar::BACKGROUND);

    let scene = RayScene::new(
        VIEW_WIDTH,
        VIEW_HEIGHT,
        VIEW_DEPTH,
        Vec3D::ZERO,
        vec![
            RaySphere::new(Vec3D::new(0.0, -1.0, 3.0), 1.0, Colour::RED),
            RaySphere::new(Vec3D::new(2.0, 0.0, 4.0), 1.0, Colour::BLUE),
            RaySphere::new(Vec3D::new(-2.0, 0.0, 4.0), 1.0, Colour::GREEN),
        ],
    );

    for x in 0..canvas.width as isize {
        for y in 0..canvas.height as isize {
            let canvas_point = Vec2D::new(x, canvas.height as isize - y - 1);
            // 2. Determine which square on the viewport corresponds to this pixel
            let view_pos =
                scene.canvas_to_viewport(Vec2D { x, y } - canvas.center(), canvas.size());

            // 3. Determine the colour seen through that square
            let colour = scene.trace_ray(view_pos, 1.0, f64::INFINITY).as_modifier();

            // 4. Paint the pixel with that clour
            let fill_char = ColChar::SOLID.with_mod(colour);
            canvas.plot(canvas_point, fill_char, Wrapping::Panic);
        }
    }

    canvas.display_render().unwrap();
}
