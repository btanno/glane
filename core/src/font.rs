use super::*;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use std::sync::Arc;

pub struct FontFace {
    pub path: PathBuf,
    pub data: Vec<u8>,
    pub index: u32,
}

impl FontFace {
    #[inline]
    pub fn from_file(path: impl AsRef<Path>, index: u32) -> std::io::Result<Arc<Self>> {
        let path = path.as_ref();
        let data = {
            let mut reader = BufReader::new(File::open(path)?);
            let mut buffer = vec![];
            reader.read_to_end(&mut buffer)?;
            buffer
        };
        Ok(Arc::new(Self {
            path: path.into(),
            data,
            index,
        }))
    }

    #[inline]
    pub fn from_os_default() -> std::io::Result<Arc<Self>> {
        if cfg!(windows) {
            // Yu Gothic UI
            Self::from_file("C:\\Windows\\Fonts\\YuGothM.ttc", 1)
        } else {
            Err(std::io::ErrorKind::NotFound.into())
        }
    }
}

impl std::fmt::Debug for FontFace {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FontFace {{ data, index: {} }}", self.index)
    }
}

#[derive(Clone, Debug)]
pub struct Font {
    pub face: Arc<FontFace>,
    pub size: f32,
}

impl Font {
    #[inline]
    pub fn new(face: &Arc<FontFace>, size: f32) -> Self {
        Self {
            face: face.clone(),
            size,
        }
    }

    #[inline]
    pub fn global_bounding_size(&self) -> LogicalSize<f32> {
        let face = rustybuzz::Face::from_slice(&self.face.data, self.face.index).unwrap();
        let size = self.size * 96.0 / 72.0;
        let scale = size / face.units_per_em() as f32;
        let bounding = face.global_bounding_box();
        LogicalSize::new(
            (bounding.x_max - bounding.x_min) as f32 * scale,
            (bounding.y_max - bounding.y_min) as f32 * scale,
        )
    }
}

pub fn bounding_box_with_str(font: &Font, s: &str) -> LogicalRect<f32> {
    let face = rustybuzz::Face::from_slice(&font.face.data, font.face.index).unwrap();
    let size = font.size * 96.0 / 72.0;
    let scale = size / face.units_per_em() as f32;
    let bounding = face.global_bounding_box();
    let bottom = (bounding.y_max - bounding.y_min) as f32 * scale;
    if s.is_empty() {
        return LogicalRect::new(0.0, 0.0, 0.0, bottom);
    }
    let mut buffer = rustybuzz::UnicodeBuffer::new();
    buffer.push_str(s);
    let glyph_buffer = rustybuzz::shape(&face, &[], buffer);
    let positions = glyph_buffer.glyph_positions();
    LogicalRect::new(
        positions[0].x_offset as f32 * scale,
        0.0,
        positions
            .iter()
            .map(|p| p.x_advance as f32)
            .sum::<f32>() * scale,
        bottom,
    )
}
