use base64::{engine::general_purpose, Engine as _};
use image::codecs::gif::GifDecoder;
use image::{AnimationDecoder, DynamicImage, ImageFormat};
use std::fs::File;
use std::io::{BufReader, Cursor};

#[tauri::command]
pub fn first_frame_from_gif(path: String) -> Result<String, String> {
    let file = File::open(&path).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);

    let decoder = GifDecoder::new(reader).map_err(|e| e.to_string())?;

    let mut frames = decoder.into_frames();
    let frame_resoult = frames.next().ok_or("No frames found")?;
    let frame = frame_resoult.map_err(|e| e.to_string())?;

    let image = DynamicImage::ImageRgba8(frame.into_buffer());
    let mut buf = Vec::new();
    image
        .write_to(&mut Cursor::new(&mut buf), ImageFormat::Png)
        .map_err(|e| e.to_string())?;
    let encoded = general_purpose::STANDARD.encode(&buf);
    Ok(encoded)
}
