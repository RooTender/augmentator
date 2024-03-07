use std::collections::HashMap;

use crate::transformations::*;

type TransformationFactoryFn = Box<dyn Fn() -> Box<dyn ImageTransformation>>;

pub struct TransformationFactory {
    registry: HashMap<String, TransformationFactoryFn>,
}

impl TransformationFactory {
    pub fn new() -> Self {
        let mut factory = TransformationFactory {
            registry: HashMap::new(),
        };
        // Register all transformations
        factory.register::<ShiftH>("hor_shift");
        factory.register::<ShiftV>("ver_shift");
        // TODO: implement crop, resize
        factory.register::<Rotate90>("rotate90");
        factory.register::<Rotate180>("rotate180");
        factory.register::<Rotate270>("rotate270");
        factory.register::<FlipH>("mirror");
        factory.register::<FlipV>("flip");
        factory.register::<HueRotate>("hue_rotation");
        factory.register::<Saturate>("saturation");
        factory.register::<Brighten>("brightness");
        factory.register::<Contrast>("contrast");
        factory.register::<Grayscale>("grayscale");
        factory.register::<Invert>("invert");
        // TODO: implement color_norm
        factory
    }

    pub fn create(&self, name: &str) -> Option<Box<dyn ImageTransformation>> {
        self.registry.get(name).map(|constructor| constructor())
    }

    fn register<T: 'static + ImageTransformation + Default>(&mut self, name: &str) {
        let constructor = Box::new(|| Box::new(T::default()) as Box<dyn ImageTransformation>);
        self.registry.insert(name.to_string(), constructor);
    }
}
