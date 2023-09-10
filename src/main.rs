use gemini_engine::elements::{
    view::{ColChar, View, Wrapping},
    Vec2D,
};
use gemini_engine::elements3d::Vec3D;
use raytracing::{Colour, Light, RayScene, RaySphere};

const VIEW_WIDTH: f64 = 1.0;
const VIEW_HEIGHT: f64 = 1.0;
const VIEW_DEPTH: f64 = 1.0;

fn main() {
    let mut canvas = View::new(500, 170, ColChar::BACKGROUND);

    let scene = RayScene::new(
        VIEW_WIDTH,
        VIEW_HEIGHT,
        VIEW_DEPTH,
        Vec3D::ZERO,
        vec![
            RaySphere::new(
                Vec3D::new(0.0, -1.0, 3.0),
                1.0,
                Colour::new(255, 0, 0),
                500.0,
            ), // Red
            RaySphere::new(
                Vec3D::new(2.0, 0.0, 4.0),
                1.0,
                Colour::new(0, 0, 255),
                500.0,
            ), // Blue
            RaySphere::new(
                Vec3D::new(-2.0, 0.0, 4.0),
                1.0,
                Colour::new(0, 255, 0),
                10.0,
            ), // Green
            RaySphere::new(
                Vec3D::new(0.0, -5001.0, 0.0),
                5000.0,
                Colour::new(255, 255, 0),
                1000.0,
            ), // Yellow
        ],
        vec![
            Light::new_ambient(0.2),
            Light::new_point(0.6, Vec3D::new(2.0, 1.0, 0.0)),
            Light::new_directional(0.2, Vec3D::new(1.0, 4.0, 4.0)),
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
