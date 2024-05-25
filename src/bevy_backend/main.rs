use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    time::FixedTimestep,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup,setup)
        .add_systems(
            Update,
            redraw_shapes)
        
        .run();
}

const X_EXTENT: f32 = 600.0;
const Y_EXTENT: f32 = 300.0;

#[derive(Resource)]
struct ShapeData {
    shapes: Vec<(Mesh2dHandle, Color, f32, f32)>,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    let shapes = vec![
        (meshes.add(Mesh::from(shape::Circle { radius: 50.0 })), 0.0),
        (meshes.add(Mesh::from(shape::Quad::new(Vec2::new(50.0, 100.0)))), 0.0),
        (meshes.add(Mesh::from(shape::RegularPolygon::new(50.0, 6))), 0.0),
        (
            meshes.add(Mesh::from(shape::Quad::new(Vec2::new(50.0, 50.0)))),
            0.0,
        ),
        (
            meshes.add(Mesh::from(shape::Quad::new(Vec2::new(100.0, 50.0)))),
            0.0,
        ),
    ];
    let num_shapes = shapes.len();
    let mut shape_data = vec![];

    for (i, (mesh, y)) in shapes.into_iter().enumerate() {
        let color = Color::hsl(360.0 * i as f32 / num_shapes as f32, 0.95, 0.7);
        let x = -X_EXTENT / 2.0 + i as f32 / (num_shapes - 1) as f32 * X_EXTENT;

        shape_data.push((Mesh2dHandle(mesh), color, x, y));
    }

    commands.insert_resource(ShapeData { shapes: shape_data });
}

fn redraw_shapes(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    shape_data: Res<ShapeData>,
) {
    let mut new_shapes = vec![];

    for (mesh, color, x, mut y) in shape_data.shapes.iter() {
        y += 10.0; // Move up by 10 units
        if *y > Y_EXTENT {
            y = -Y_EXTENT;
        }

        commands.spawn(MaterialMesh2dBundle {
            mesh: *mesh,
            material: materials.add(*color),
            transform: Transform::from_xyz(*x, y, 0.0),
            ..default()
        });

        new_shapes.push((*mesh, *color, *x, y));
    }

    commands.insert_resource(ShapeData { shapes: new_shapes });
}
