use super::*;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use std::sync::Arc;

pub struct FontFile {
    path: PathBuf,
    data: Vec<u8>,
    collection: Vec<String>,
}

impl FontFile {
    pub fn new(path: impl AsRef<Path>) -> std::io::Result<Arc<Self>> {
        let path = path.as_ref();
        let data = {
            let mut reader = BufReader::new(File::open(path)?);
            let mut buffer = vec![];
            reader.read_to_end(&mut buffer)?;
            buffer
        };
        let len = ttf_parser::fonts_in_collection(&data).unwrap_or(1);
        let collection = (0..len)
            .map(|index| {
                let face = ttf_parser::Face::parse(&data, index)
                    .map_err(|_| std::io::ErrorKind::InvalidData)?;
                let names = face.names();
                Ok((0..names.len())
                    .map(|i| names.get(i).unwrap())
                    .find(|n| n.name_id == 1)
                    .map(|n| n.to_string().unwrap())
                    .unwrap())
            })
            .collect::<std::io::Result<Vec<_>>>()?;
        Ok(Arc::new(Self {
            path: path.into(),
            data,
            collection,
        }))
    }

    #[inline]
    pub fn path(&self) -> &Path {
        &self.path
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &String> {
        self.collection.iter()
    }
}

impl std::fmt::Debug for FontFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "FontFile {{ path: {:?}, collection: {:?} }}",
            self.path, self.collection
        )
    }
}

#[derive(Clone, Debug)]
pub struct FontFace {
    file: Arc<FontFile>,
    index: usize,
}

impl FontFace {
    #[inline]
    pub fn new(file: &Arc<FontFile>, index: usize) -> std::io::Result<Self> {
        Ok(Self {
            file: file.clone(),
            index: (index < file.collection.len())
                .then_some(index)
                .ok_or(std::io::ErrorKind::NotFound)?,
        })
    }

    #[inline]
    pub fn from_font_family_name(
        file: &Arc<FontFile>,
        name: impl AsRef<str>,
    ) -> std::io::Result<Self> {
        Ok(Self {
            file: file.clone(),
            index: file
                .collection
                .iter()
                .enumerate()
                .find(|(_, n)| *n == name.as_ref())
                .map(|(i, _)| i)
                .ok_or(std::io::ErrorKind::NotFound)?,
        })
    }

    #[inline]
    pub fn from_os_default() -> std::io::Result<Self> {
        if cfg!(windows) {
            let file = FontFile::new("C:\\Windows\\Fonts\\YuGothM.ttc")?;
            Ok(Self::from_font_family_name(&file, "Yu Gothic UI")?)
        } else {
            Err(std::io::ErrorKind::NotFound.into())
        }
    }

    #[inline]
    pub fn font_family_name(&self) -> &str {
        &self.file.collection[self.index]
    }

    #[inline]
    pub fn font_file(&self) -> &Arc<FontFile> {
        &self.file
    }
}

impl PartialEq for FontFace {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.file, &other.file) && self.index == other.index
    }
}

impl Eq for FontFace {}

#[derive(Clone, Debug, PartialEq)]
pub struct Font {
    pub face: FontFace,
    pub size: f32,
}

impl Font {
    #[inline]
    pub fn new(face: &FontFace, size: f32) -> Self {
        Self {
            face: face.clone(),
            size,
        }
    }

    #[inline]
    pub fn global_bounding_size(&self) -> LogicalSize<f32> {
        let face =
            rustybuzz::Face::from_slice(&self.face.file.data, self.face.index as u32).unwrap();
        let size = self.size * 96.0 / 72.0;
        let scale = size / face.units_per_em() as f32;
        let bounding = face.global_bounding_box();
        LogicalSize::new(
            (bounding.x_max - bounding.x_min) as f32 * scale,
            (bounding.y_max - bounding.y_min) as f32 * scale,
        )
    }
}

mod bounding_box {
    use super::*;
    use std::collections::VecDeque;
    use std::sync::Mutex;

    #[derive(Debug)]
    struct Element {
        font: Font,
        rects: VecDeque<(String, LogicalRect<f32>)>,
    }

    #[derive(Debug)]
    pub struct Cache {
        elements: Mutex<Vec<Element>>,
        max_size: usize,
    }

    impl Cache {
        pub fn new(max_size: usize) -> Arc<Self> {
            Arc::new(Self {
                elements: Mutex::new(vec![]),
                max_size,
            })
        }

        pub fn get(&self, font: &Font, s: &str) -> LogicalRect<f32> {
            let mut elements = self.elements.lock().unwrap();
            let element =
                if let Some(element) = elements.iter_mut().find(|element| &element.font == font) {
                    element
                } else {
                    elements.push(Element {
                        font: font.clone(),
                        rects: VecDeque::with_capacity(self.max_size),
                    });
                    elements.last_mut().unwrap()
                };
            if let Some(index) = element.rects.iter().position(|(str, _rc)| str == s) {
                let obj = element.rects.remove(index).unwrap();
                let rect = obj.1;
                element.rects.push_front(obj);
                return rect;
            }
            if element.rects.len() >= self.max_size {
                element.rects.pop_back();
            }
            let rect = Self::create(font, s);
            let s = s.to_string();
            element.rects.push_front((s, rect));
            rect
        }

        fn create(font: &Font, s: &str) -> LogicalRect<f32> {
            let face =
                rustybuzz::Face::from_slice(&font.face.file.data, font.face.index as u32).unwrap();
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
                positions.iter().map(|p| p.x_advance as f32).sum::<f32>() * scale,
                bottom,
            )
        }
    }
}

pub use bounding_box::Cache as BoundingBoxCache;

pub fn bounding_box_with_str(ctx: &Context, font: &Font, s: &str) -> LogicalRect<f32> {
    ctx.bounding_box_cache.get(font, s)
}
