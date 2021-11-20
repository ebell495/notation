use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_prototype_lyon::entity::ShapeBundle;

pub trait Shape {
    fn _create(&self, commands: &mut Commands, entity: Entity);
    fn create(&self, commands: &mut Commands, parent: Entity) -> Entity {
        let entity = commands.spawn().id();
        commands.entity(parent).push_children(&[entity]);
        self._create(commands, entity);
        entity
    }
    fn update(&self, commands: &mut Commands, entity: Entity) {
        commands.entity(entity).remove_bundle::<ShapeBundle>();
        self._create(commands, entity);
    }
}

pub trait SingleShape<T: Geometry> : Shape {
    fn get_shape(&self) -> T;
    fn get_colors(&self) -> ShapeColors;
    fn get_draw_mode(&self) -> DrawMode;
    fn get_transform(&self) -> Transform;
    fn _do_create(&self, commands: &mut Commands, entity: Entity) {
        let shape = self.get_shape();
        let colors = self.get_colors();
        let draw_mode = self.get_draw_mode();
        let transform = self.get_transform();
        commands.entity(entity).insert_bundle(GeometryBuilder::build_as(
            &shape, colors, draw_mode, transform,
        ));
    }
}

pub trait ShapeOp<Theme, S: Shape> : Clone + Send + Sync + 'static {
    fn get_shape(&self, theme: &Theme) -> S;
    fn create(&self, commands: &mut Commands, theme: &Theme, parent: Entity) -> Entity {
        let shape = self.get_shape(theme);
        let shape_entity = shape.create(commands, parent);
        commands.entity(shape_entity).insert(self.clone());
        shape_entity
    }
    fn update(&self, commands: &mut Commands, theme: &Theme, entity: Entity) {
        let shape = self.get_shape(theme);
        shape.update(commands, entity);
    }
}