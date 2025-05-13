use mupdf::{Document, Matrix};
use image::DynamicImage;
use leptonica_sys::{pixCreate, pixGetData, pixGetWpl, pixDestroy};
use tesseract_sys::*;
use std::ffi::CString;
use anyhow::{Result, anyhow};
use std::time::Instant;

/// Extrair texto via OCR a partir de uma imagem em memória (usando Tesseract FFI)
fn extract_text_from_image(img: &DynamicImage) -> Result<String> {
    let img = img.to_luma8();

    let width = img.width();
    let height = img.height();
    let width_i32 = width as i32;
    let height_i32 = height as i32;

    let data = img.into_raw();
    assert_eq!(data.len(), (width * height) as usize);

    unsafe {
        let mut pixs = pixCreate(width_i32, height_i32, 8);
        if pixs.is_null() {
            return Err(anyhow!("Falha ao criar PIX"));
        }

        let stride = pixGetWpl(pixs) as isize;
        let pixels = pixGetData(pixs);

        for y in 0..height {
            let src_row = data.as_ptr().offset((y * width) as isize);
            let dst_row = pixels.offset(y as isize * stride);
            std::ptr::copy_nonoverlapping(src_row, dst_row as *mut u8, width as usize);
        }

        // OCR com Tesseract
        let api = TessBaseAPICreate();
        if TessBaseAPIInit3(api, std::ptr::null(), CString::new("por").unwrap().as_ptr()) != 0 {
            return Err(anyhow!("Falha ao inicializar Tesseract"));
        }

        TessBaseAPISetImage2(api, pixs);

        // Setar DPI manualmente para evitar aviso "Invalid resolution 0 dpi"
        TessBaseAPISetSourceResolution(api, 300);

        let text = TessBaseAPIGetUTF8Text(api);
        let text_str = if !text.is_null() {
            CString::from_vec_unchecked(Vec::from(std::ffi::CStr::from_ptr(text).to_bytes()))
                .into_string()
                .map_err(|_| anyhow!("Texto OCR inválido"))?
        } else {
            String::new()
        };

        TessBaseAPIEnd(api);
        TessBaseAPIDelete(api);
        pixDestroy(&mut pixs);

        Ok(text_str)
    }
}

/// Renderiza uma página do PDF como imagem na memória
fn render_pdf_page(pdf_data: &[u8], page_index: i32) -> Result<DynamicImage> {
    let doc = Document::from_bytes(pdf_data, "")?;

    let num_pages = doc.pages()?.count() as i32;
    if page_index >= num_pages {
        return Err(anyhow!("Número da página fora do intervalo"));
    }

    let page = doc.load_page(page_index)?;
    let matrix = Matrix::new(2.0, 0.0, 0.0, 2.0, 0.0, 0.0);

    let cs = mupdf::Colorspace::device_rgb();
    let pixmap = page.to_pixmap(&matrix, &cs, false, false)?;

    let width = pixmap.width() as u32;
    let height = pixmap.height() as u32;
    let samples = pixmap.samples().to_vec();

    let img = image::RgbImage::from_raw(width, height, samples)
        .ok_or_else(|| anyhow!("Falha ao criar imagem RGB"))?;

    Ok(DynamicImage::ImageRgb8(img))
}

fn main() -> Result<()> {
    let start = Instant::now();

    // URL do PDF
    let pdf_url = "https://vflhuqqzjmgkdhjgxzni.supabase.co/storage/v1/object/public/pdf/68c410e5-a63c-41a2-a62e-5252acc6880d/25da709a-56dc-4c68-ac76-d7f28a2d2c95.pdf";

    // Baixar PDF diretamente para memória
    let client = reqwest::blocking::Client::new();
    let response = client.get(pdf_url).send()?;
    let pdf_data = response.bytes()?.to_vec();

    // Carregar PDF em memória
    let doc = Document::from_bytes(&pdf_data, "")?;
    let num_pages = doc.pages()?.count();

    println!("Número total de páginas no PDF: {}", num_pages);

    let mut total_chars = 0;

    // Processar páginas sequencialmente
    for page_index in 0..num_pages as i32 {
        println!("Processando página {}...", page_index + 1);

        match render_pdf_page(&pdf_data, page_index) {
            Ok(img) => {
                match extract_text_from_image(&img) {
                    Ok(text) => {
                        let char_count = text.len();
                        total_chars += char_count;
                        println!("Página {}: {} caracteres", page_index + 1, char_count);
                    },
                    Err(e) => eprintln!("Erro ao extrair texto da página {}: {}", page_index + 1, e),
                }
            },
            Err(e) => eprintln!("Erro ao renderizar página {}: {}", page_index + 1, e),
        }
    }

    println!(
        "Total de caracteres extraídos em todo o documento: {}",
        total_chars
    );

    let duration = start.elapsed();
    println!("Tempo total de execução: {:?}", duration);

    Ok(())
}