use crate::errors::SketchError;
use crate::types::SketchConfig;

pub fn validate_sketch_config(cfg: &SketchConfig) -> Result<(), SketchError> {
    if cfg.width < 4 || cfg.depth == 0 || cfg.depth > 32 {
        return Err(SketchError::InvalidDimension("width >= 4 and 1 <= depth <= 32 required"));
    }
    Ok(())
}
