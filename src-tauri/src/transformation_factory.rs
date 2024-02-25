use std::collections::HashMap;

use crate::transformations::*;

type TransformationFactoryFn = Box<dyn Fn() -> Box<dyn ImageTransformation>>;

struct TransformationFactory {
    registry: HashMap<String, TransformationFactoryFn>,
}

impl TransformationFactory {
    fn new() -> Self {
        let mut factory = TransformationFactory {
            registry: HashMap::new(),
        };
        // Register all transformations
        factory.register("hor_shift", Box::new(|| Box::new(ShiftH) as Box<dyn ImageTransformation>));
        factory.register("ver_shift", Box::new(|| Box::new(ShiftV) as Box<dyn ImageTransformation>));
        // TODO: implement crop, resize
        factory.register("rotate90", Box::new(|| Box::new(Rotate90) as Box<dyn ImageTransformation>));
        factory.register("rotate180", Box::new(|| Box::new(Rotate180) as Box<dyn ImageTransformation>));
        factory.register("rotate270", Box::new(|| Box::new(Rotate270) as Box<dyn ImageTransformation>));
        factory.register("mirror", Box::new(|| Box::new(FlipH) as Box<dyn ImageTransformation>));
        factory.register("flip", Box::new(|| Box::new(FlipV) as Box<dyn ImageTransformation>));
        factory.register("hue_rotation", Box::new(|| Box::new(HueRotate) as Box<dyn ImageTransformation>));
        factory.register("saturation", Box::new(|| Box::new(Saturate) as Box<dyn ImageTransformation>));
        factory.register("brightness", Box::new(|| Box::new(Brighten) as Box<dyn ImageTransformation>));
        factory.register("contrast", Box::new(|| Box::new(Contrast) as Box<dyn ImageTransformation>));
        factory.register("grayscale", Box::new(|| Box::new(Grayscale) as Box<dyn ImageTransformation>));
        factory.register("invert", Box::new(|| Box::new(Invert) as Box<dyn ImageTransformation>));
        // TODO: implement color_norm
        factory
    }

    fn register(&mut self, name: &str, constructor: TransformationFactoryFn) {
        self.registry.insert(name.to_string(), constructor);
    }

    fn create(&self, name: &str) -> Option<Box<dyn ImageTransformation>> {
        self.registry.get(name).map(|constructor| constructor())
    }
}
