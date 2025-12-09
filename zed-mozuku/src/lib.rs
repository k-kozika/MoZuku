use zed_extension_api as zed;

struct MoZukuExtension {
    // ... state
}

impl zed::Extension for MoZukuExtension {
    fn new() -> Self
        where
            Self: Sized {
        
    }
}

zed::register_extension!(MoZukuExtension);
