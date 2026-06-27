use std::path::{Path, PathBuf};
use std::sync::Mutex;
use base64::Engine;
use image::ImageFormat;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, State};

// ── Extensões suportadas ───────────────────────────────────────────────────

/// Extensões abertas diretamente via WebView2 (SVG como vetor real).
const SVG_EXTS: &[&str] = &["svg", "svgz"];

/// Extensões decodificadas pelo backend Rust via crate `image` e enviadas
/// como base64 para o frontend. Inclui WebP animado via fallback de frame.
const RASTER_EXTS: &[&str] = &[
    "png", "jpg", "jpeg", "webp", "gif", "bmp", "tiff", "tif",
    "avif", "ico",
];

/// Extensões que o frontend recebe como URL de arquivo local (via protocolo
/// asset:// do Tauri), sem conversão base64 — adequado para formatos que o
/// WebView2 já suporta nativamente: PNG, JPG, GIF estático, WebP estático.
const NATIVE_WEBVIEW_EXTS: &[&str] = &["png", "jpg", "jpeg", "gif", "webp", "ico"];

// ── Estado compartilhado ───────────────────────────────────────────────────

/// Estado da sessão do viewer: pasta atual, índice da imagem aberta,
/// e lista ordenada de imagens da pasta.
pub struct ViewerState {
    /// Lista de caminhos de imagens na pasta atual, em ordem alfabética.
    pub files: Vec<PathBuf>,
    /// Índice da imagem atualmente exibida em `files`.
    pub current_index: usize,
}

impl Default for ViewerState {
    fn default() -> Self {
        Self {
            files: Vec::new(),
            current_index: 0,
        }
    }
}

// ── Tipos de resposta para o frontend ─────────────────────────────────────

/// Descreve o tipo de conteúdo que o frontend deve renderizar.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ImageKind {
    /// SVG: o frontend renderiza como `<iframe>` ou `<object>` apontando para
    /// o asset local, garantindo renderização vetorial via WebView2.
    Svg,
    /// Raster nativo: o WebView2 já suporta o formato; usar `<img src=asset>`.
    NativeRaster,
    /// Raster via base64: formato não suportado nativamente pelo WebView2
    /// (ex: TIFF, AVIF, BMP) → decodificado no Rust e enviado como data URL.
    Base64Raster,
}

/// Payload enviado ao frontend ao carregar uma imagem.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ImagePayload {
    /// Tipo de conteúdo (determina como o frontend renderiza).
    pub kind: ImageKind,
    /// Caminho absoluto do arquivo (usado para asset:// e title da janela).
    pub path: String,
    /// Nome do arquivo (para exibir na barra de título).
    pub filename: String,
    /// Posição na lista (1-based) e total — ex: "3 / 47".
    pub position: String,
    /// Dados base64 com data URL completa — preenchido só quando kind = Base64Raster.
    /// Ex: "data:image/png;base64,iVBORw0KGgo..."
    pub data_url: Option<String>,
    /// MIME type do arquivo — usado para construir data URL no frontend.
    pub mime: String,
}

// ── Helpers ────────────────────────────────────────────────────────────────

/// Retorna a extensão do arquivo em minúsculas, ou string vazia.
fn ext_lower(path: &Path) -> String {
    path.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase()
}

/// Verifica se o caminho é uma imagem suportada pelo viewer.
fn is_supported(path: &Path) -> bool {
    let ext = ext_lower(path);
    SVG_EXTS.contains(&ext.as_str()) || RASTER_EXTS.contains(&ext.as_str())
}

/// Lista todas as imagens suportadas em uma pasta, ordenadas por nome.
fn list_images_in_dir(dir: &Path) -> Vec<PathBuf> {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return Vec::new();
    };

    let mut images: Vec<PathBuf> = entries
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.is_file() && is_supported(p))
        .collect();

    // Ordenação case-insensitive por nome de arquivo
    images.sort_by(|a, b| {
        let na = a.file_name().unwrap_or_default().to_string_lossy().to_lowercase();
        let nb = b.file_name().unwrap_or_default().to_string_lossy().to_lowercase();
        na.cmp(&nb)
    });

    images
}

/// Determina o MIME type com base na extensão.
fn mime_for_ext(ext: &str) -> &'static str {
    match ext {
        "svg" | "svgz" => "image/svg+xml",
        "png"          => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "webp"         => "image/webp",
        "gif"          => "image/gif",
        "bmp"          => "image/bmp",
        "tiff" | "tif" => "image/tiff",
        "avif"         => "image/avif",
        "ico"          => "image/x-icon",
        _              => "application/octet-stream",
    }
}

/// Constrói o `ImagePayload` para o arquivo no índice dado.
/// Lê o arquivo do disco e decide a estratégia de exibição.
fn build_payload(files: &[PathBuf], index: usize) -> Result<ImagePayload, String> {
    let path = &files[index];
    let ext = ext_lower(path);
    let filename = path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();
    let position = format!("{} / {}", index + 1, files.len());
    let mime = mime_for_ext(&ext).to_string();
    let path_str = path.to_string_lossy().to_string();

    if SVG_EXTS.contains(&ext.as_str()) {
        // SVG: frontend usa asset:// — renderização vetorial via WebView2
        return Ok(ImagePayload {
            kind: ImageKind::Svg,
            path: path_str,
            filename,
            position,
            data_url: None,
            mime,
        });
    }

    if NATIVE_WEBVIEW_EXTS.contains(&ext.as_str()) {
        // WebView2 suporta nativamente: PNG, JPG, GIF, WebP, ICO
        return Ok(ImagePayload {
            kind: ImageKind::NativeRaster,
            path: path_str,
            filename,
            position,
            data_url: None,
            mime,
        });
    }

    // Formatos não suportados pelo WebView2 (TIFF, AVIF, BMP):
    // decodificar com `image` crate e enviar como PNG base64.
    let img = image::open(path)
        .map_err(|e| format!("Falha ao abrir imagem: {e}"))?;

    let mut png_bytes: Vec<u8> = Vec::new();
    img.write_to(
        &mut std::io::Cursor::new(&mut png_bytes),
        ImageFormat::Png,
    )
    .map_err(|e| format!("Falha ao converter para PNG: {e}"))?;

    let b64 = base64::engine::general_purpose::STANDARD.encode(&png_bytes);
    let data_url = format!("data:image/png;base64,{b64}");

    Ok(ImagePayload {
        kind: ImageKind::Base64Raster,
        path: path_str,
        filename,
        position,
        data_url: Some(data_url),
        // MIME reportado como PNG pois convertemos internamente
        mime: "image/png".to_string(),
    })
}

// ── Comandos Tauri (chamados pelo frontend via invoke) ────────────────────

/// Abre um arquivo de imagem pelo caminho absoluto.
/// Reconstrói a lista de imagens da pasta e posiciona no arquivo solicitado.
#[tauri::command]
pub fn open_file(
    path: String,
    state: State<'_, Mutex<ViewerState>>,
    app: AppHandle,
) -> Result<ImagePayload, String> {
    let p = PathBuf::from(&path);

    let dir = p
        .parent()
        .ok_or("Arquivo não tem pasta pai")?
        .to_path_buf();

    let files = list_images_in_dir(&dir);

    // Encontra o índice do arquivo solicitado na lista ordenada
    let index = files
        .iter()
        .position(|f| f == &p)
        .unwrap_or(0);

    let payload = build_payload(&files, index)?;

    // Atualiza título da janela: "filename — WinImgView"
    if let Some(win) = app.get_webview_window("main") {
        let _ = win.set_title(&format!("{} — WinImgView", payload.filename));
    }

    // Persiste estado para navegação posterior
    let mut st = state.lock().map_err(|e| e.to_string())?;
    st.files = files;
    st.current_index = index;

    Ok(payload)
}

/// Avança para a próxima imagem da pasta (wrap-around no final).
#[tauri::command]
pub fn next_image(
    state: State<'_, Mutex<ViewerState>>,
    app: AppHandle,
) -> Result<ImagePayload, String> {
    let mut st = state.lock().map_err(|e| e.to_string())?;

    if st.files.is_empty() {
        return Err("Nenhuma imagem carregada".to_string());
    }

    // Wrap-around: após a última imagem, volta para a primeira
    st.current_index = (st.current_index + 1) % st.files.len();
    let payload = build_payload(&st.files, st.current_index)?;

    if let Some(win) = app.get_webview_window("main") {
        let _ = win.set_title(&format!("{} — WinImgView", payload.filename));
    }

    Ok(payload)
}

/// Volta para a imagem anterior da pasta (wrap-around no início).
#[tauri::command]
pub fn prev_image(
    state: State<'_, Mutex<ViewerState>>,
    app: AppHandle,
) -> Result<ImagePayload, String> {
    let mut st = state.lock().map_err(|e| e.to_string())?;

    if st.files.is_empty() {
        return Err("Nenhuma imagem carregada".to_string());
    }

    // Wrap-around: antes da primeira imagem, vai para a última
    st.current_index = if st.current_index == 0 {
        st.files.len() - 1
    } else {
        st.current_index - 1
    };

    let payload = build_payload(&st.files, st.current_index)?;

    if let Some(win) = app.get_webview_window("main") {
        let _ = win.set_title(&format!("{} — WinImgView", payload.filename));
    }

    Ok(payload)
}

/// Retorna o índice atual e o total de imagens na pasta.
/// Usado para atualizar o contador sem recarregar a imagem.
#[tauri::command]
pub fn get_position(state: State<'_, Mutex<ViewerState>>) -> Result<String, String> {
    let st = state.lock().map_err(|e| e.to_string())?;
    if st.files.is_empty() {
        return Ok(String::new());
    }
    Ok(format!("{} / {}", st.current_index + 1, st.files.len()))
}

// ── Entry point da biblioteca (chamado por main.rs) ───────────────────────

pub fn run() {
    // Coleta argumento de linha de comando: o caminho do arquivo a abrir.
    // O Explorer passa o arquivo como argv[1] no duplo-clique.
    let initial_path: Option<String> = std::env::args().nth(1);

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        // Estado compartilhado entre os comandos — protegido por Mutex
        .manage(Mutex::new(ViewerState::default()))
        .invoke_handler(tauri::generate_handler![
            open_file,
            next_image,
            prev_image,
            get_position,
        ])
        .setup(move |app| {
            // Se foi passado um arquivo na linha de comando, abre ao iniciar
            if let Some(path) = initial_path {
                let app_handle = app.handle().clone();
                let state = app.state::<Mutex<ViewerState>>();

                // Executa a abertura do arquivo no thread principal
                match open_file(path, state, app_handle) {
                    Ok(payload) => {
                        // Envia o payload para o frontend via evento
                        if let Some(win) = app.get_webview_window("main") {
                            let _ = win.emit("image-loaded", payload);
                        }
                    }
                    Err(e) => {
                        eprintln!("Erro ao abrir arquivo inicial: {e}");
                    }
                }
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Erro ao iniciar WinImgView");
}
