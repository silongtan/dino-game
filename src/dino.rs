use bevy::prelude::*;
use bevy_prototype_lyon::prelude as lyon;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
struct Dino;

fn spawn_dino(mut commands: Commands) {
    let ball_pos = Vec2::new(
        crate::PIXELS_PER_METER * 0.3,
        crate::PIXELS_PER_METER * -0.2,
    );

    let shape_ball = lyon::shapes::Circle {
        radius: crate::PIXELS_PER_METER * 0.03,
        center: Vec2::ZERO,
    };

    commands
        .spawn(lyon::GeometryBuilder::build_as(
            &shape_ball,
            lyon::DrawMode::Outlined {
                fill_mode: lyon::FillMode::color(Color::BLACK),
                outline_mode: lyon::StrokeMode::new(Color::TEAL, 2.0),
            },
            Transform::default(),
        ))
        .insert(RigidBody::Dynamic)
        .insert(Sleeping::disabled())
        .insert(Ccd::enabled())
        .insert(Collider::ball(shape_ball.radius))
        .insert(Transform::from_xyz(ball_pos.x, ball_pos.y, 0.0))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Restitution::coefficient(0.7))
        .insert(Ball);
}
