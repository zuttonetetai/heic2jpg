use libheif_rs::{HeifContext, ColorSpace, RgbChroma};
use image::{RgbImage, ImageFormat};
use std::fs;
use std::path::Path;

fn convert_heic_to_jpg(input_path: &Path, output_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // HEICファイルを読み込む
    let ctx = HeifContext::read_from_file(input_path.to_str().ok_or("Invalid input path")?)?;
    let handle = ctx.primary_image_handle()?;

    // HEICイメージをデコード
    let image = handle.decode(ColorSpace::Rgb(RgbChroma::Rgb), None)?;

    // RgbImageに変換
    let width = image.width() as u32;
    let height = image.height() as u32;
    let mut rgb_image = RgbImage::new(width, height);
    
    // プレーンデータをコピー
    let planes = image.planes();
    let interleaved_plane = planes.interleaved.ok_or("No interleaved plane data")?;
    let bytes = interleaved_plane.data;
    rgb_image.copy_from_slice(&bytes);

    // JPGとして保存
    rgb_image.save_with_format(output_path, ImageFormat::Jpeg)?;
    Ok(())
}

fn process_folder(input_folder: &str, output_folder: &str) -> Result<(), Box<dyn std::error::Error>> {
    // 出力フォルダが存在しない場合は作成
    fs::create_dir_all(output_folder)?;

    // 入力フォルダ内のファイルを走査
    for entry in fs::read_dir(input_folder)? {
        let entry = entry?;
        let path = entry.path();

        // HEICファイルのみを処理
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("heic") {
            let file_name = path.file_stem().and_then(|s| s.to_str()).ok_or("Invalid file name")?;
            let output_path = Path::new(output_folder).join(format!("{}.jpg", file_name));

            convert_heic_to_jpg(&path, &output_path)?;
            println!("変換が完了しました: {:?} -> {:?}", path, output_path);
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_folder = "input_heic";  // HEICファイルが格納されているフォルダ
    let output_folder = "output_jpg"; // 変換後のJPGファイルを保存するフォルダ
    
    process_folder(input_folder, output_folder)?;
    println!("全ての変換が完了しました。");
    
    Ok(())
}