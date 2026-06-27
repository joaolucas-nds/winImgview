/**
 * main.js — Frontend do WinImgView
 *
 * Responsabilidades:
 *  - Invocar comandos Tauri (open_file, next_image, prev_image)
 *  - Renderizar o payload recebido (SVG via iframe, raster via <img>)
 *  - Gerenciar zoom (roda do mouse, Ctrl++/-, Ctrl+0, Ctrl+1)
 *  - Gerenciar pan (arrastar com o mouse quando zoom > 1)
 *  - Drag & drop de arquivo sobre a janela
 *  - Atalhos de teclado (← →, F11, B, E, Ctrl+Shift+C)
 */

import { invoke } from "https://unpkg.com/@tauri-apps/api@2/core";
import { listen } from "https://unpkg.com/@tauri-apps/api@2/event";
import { convertFileSrc } from "https://unpkg.com/@tauri-apps/api@2/core";
import { getCurrentWindow } from "https://unpkg.com/@tauri-apps/api@2/window";

// ── Referências DOM ──────────────────────────────────────────────────────

const canvas     = document.getElementById("canvas");
const viewport   = document.getElementById("viewport");
const splash     = document.getElementById("splash");
const svgFrame   = document.getElementById("svg-frame");
const rasterImg  = document.getElementById("raster-img");
const statusPos  = document.getElementById("status-position");
const statusName = document.getElementById("status-filename");
const statusZoom = document.getElementById("status-zoom");

// ── Estado de zoom/pan ────────────────────────────────────────────────────

const zoom = {
  scale: 1.0,
  minScale: 0.05,
  maxScale: 32.0,
  /** Passo multiplicativo do zoom por scroll/tecla (+25% por passo). */
  step: 1.25,

  /** Translação acumulada para pan (em pixels no espaço transformado). */
  tx: 0,
  ty: 0,
};

/** Aplica a transformação CSS atual ao #viewport. */
function applyTransform() {
  viewport.style.transform =
    `translate(${zoom.tx}px, ${zoom.ty}px) scale(${zoom.scale})`;
  statusZoom.textContent = `${Math.round(zoom.scale * 100)}%`;
}

/**
 * Reseta zoom e pan para o estado "fit" (imagem cabe na janela, centrada).
 * Chamado sempre que uma nova imagem é aberta.
 */
function resetZoom() {
  zoom.scale = 1.0;
  zoom.tx = 0;
  zoom.ty = 0;
  applyTransform();
}

/**
 * Aplica zoom centrado no ponto (cx, cy) do canvas (coordenadas de tela).
 * @param {number} delta - Fator multiplicativo positivo (>1 aumenta, <1 diminui).
 * @param {number} cx    - X do ponto focal no canvas (pageX).
 * @param {number} cy    - Y do ponto focal no canvas (pageY).
 */
function zoomAt(delta, cx, cy) {
  const newScale = Math.min(
    zoom.maxScale,
    Math.max(zoom.minScale, zoom.scale * delta),
  );
  if (newScale === zoom.scale) return;

  // Ajusta translação para manter o ponto focal fixo na tela
  const rect   = canvas.getBoundingClientRect();
  const originX = cx - rect.left - rect.width  / 2;
  const originY = cy - rect.top  - rect.height / 2;

  zoom.tx = originX - (originX - zoom.tx) * (newScale / zoom.scale);
  zoom.ty = originY - (originY - zoom.ty) * (newScale / zoom.scale);
  zoom.scale = newScale;

  applyTransform();
}

// ── Renderização de payload ───────────────────────────────────────────────

/**
 * Recebe o ImagePayload do backend e renderiza na tela.
 * @param {object} payload - Estrutura ImagePayload serializada pelo Rust.
 */
function renderPayload(payload) {
  splash.classList.add("hidden");
  resetZoom();

  // Atualiza barra de status
  statusPos.textContent  = payload.position;
  statusName.textContent = payload.filename;

  if (payload.kind === "svg") {
    // SVG: carrega no iframe para renderização vetorial via WebView2.
    // convertFileSrc converte o caminho Windows para o protocolo asset://
    // que o Tauri expõe com as devidas permissões de leitura.
    rasterImg.classList.add("hidden");
    svgFrame.classList.remove("hidden");
    svgFrame.src = convertFileSrc(payload.path);

    // Ajusta tamanho do iframe para ocupar o canvas disponível
    svgFrame.style.width  = "100%";
    svgFrame.style.height = `calc(100vh - 28px)`;

  } else if (payload.kind === "nativeRaster") {
    // PNG, JPG, WebP, GIF: WebView2 suporta nativamente via asset://
    svgFrame.classList.add("hidden");
    rasterImg.classList.remove("hidden");
    rasterImg.src = convertFileSrc(payload.path);

  } else if (payload.kind === "base64Raster") {
    // TIFF, BMP, AVIF: backend já converteu para PNG base64
    svgFrame.classList.add("hidden");
    rasterImg.classList.remove("hidden");
    rasterImg.src = payload.dataUrl;
  }
}

// ── Comandos Tauri ────────────────────────────────────────────────────────

/** Abre um arquivo pelo caminho absoluto. */
async function openFile(path) {
  try {
    const payload = await invoke("open_file", { path });
    renderPayload(payload);
  } catch (err) {
    console.error("Erro ao abrir arquivo:", err);
    statusName.textContent = `Erro: ${err}`;
  }
}

/** Navega para a próxima imagem da pasta. */
async function nextImage() {
  try {
    const payload = await invoke("next_image");
    renderPayload(payload);
  } catch (err) {
    console.error("next_image:", err);
  }
}

/** Navega para a imagem anterior da pasta. */
async function prevImage() {
  try {
    const payload = await invoke("prev_image");
    renderPayload(payload);
  } catch (err) {
    console.error("prev_image:", err);
  }
}

// ── Evento enviado pelo backend ao abrir via argv[1] ─────────────────────

// O backend emite "image-loaded" no setup() quando argv[1] existe
listen("image-loaded", (event) => {
  renderPayload(event.payload);
});

// ── Atalhos de teclado ────────────────────────────────────────────────────

canvas.addEventListener("keydown", async (e) => {
  const ctrl = e.ctrlKey || e.metaKey;

  switch (true) {
    // Navegação
    case e.key === "ArrowRight":
      e.preventDefault();
      await nextImage();
      break;

    case e.key === "ArrowLeft":
      e.preventDefault();
      await prevImage();
      break;

    // Zoom com teclado
    case ctrl && (e.key === "+" || e.key === "="):
      e.preventDefault();
      zoomAt(zoom.step, canvas.clientWidth / 2, canvas.clientHeight / 2);
      break;

    case ctrl && e.key === "-":
      e.preventDefault();
      zoomAt(1 / zoom.step, canvas.clientWidth / 2, canvas.clientHeight / 2);
      break;

    case ctrl && e.key === "0":
      // Ctrl+0: fit (reset para 100%)
      e.preventDefault();
      resetZoom();
      break;

    case ctrl && e.key === "1":
      // Ctrl+1: zoom 100% centrado
      e.preventDefault();
      zoom.scale = 1.0;
      zoom.tx = 0;
      zoom.ty = 0;
      applyTransform();
      break;

    // F11: fullscreen toggle
    case e.key === "F11": {
      e.preventDefault();
      const win = getCurrentWindow();
      const isFullscreen = await win.isFullscreen();
      await win.setFullscreen(!isFullscreen);
      break;
    }

    // B: alterna cor de fundo (branco → preto → xadrez → cinza escuro)
    case e.key === "b" || e.key === "B":
      cycleBg();
      break;

    // Ctrl+Shift+C: copia caminho do arquivo atual para a área de transferência
    case ctrl && e.shiftKey && e.key === "C": {
      e.preventDefault();
      const path = rasterImg.src || svgFrame.src;
      // Remove o prefixo asset:// e decodifica para obter o caminho real
      const realPath = decodeURIComponent(
        path.replace(/^(asset:|https?:)\/\/localhost\//, "").replace(/\?.*$/, ""),
      );
      try {
        await navigator.clipboard.writeText(realPath);
        flashStatus("Caminho copiado!");
      } catch {
        flashStatus("Falha ao copiar");
      }
      break;
    }

    default:
      break;
  }
});

// Garante que o canvas receba foco de teclado ao clicar
canvas.addEventListener("click", () => canvas.focus());
canvas.focus();

// ── Zoom com roda do mouse ────────────────────────────────────────────────

canvas.addEventListener("wheel", (e) => {
  e.preventDefault();

  // Fator de zoom por tick de scroll; negativo = zoom in (scroll para cima)
  const delta = e.deltaY < 0 ? zoom.step : 1 / zoom.step;
  zoomAt(delta, e.clientX, e.clientY);
}, { passive: false });

// ── Pan com arrastar o mouse ──────────────────────────────────────────────

let isPanning = false;
let panStart  = { x: 0, y: 0 };

canvas.addEventListener("mousedown", (e) => {
  if (e.button !== 0) return; // só botão esquerdo
  isPanning  = true;
  panStart.x = e.clientX - zoom.tx;
  panStart.y = e.clientY - zoom.ty;
  canvas.style.cursor = "grabbing";
});

window.addEventListener("mousemove", (e) => {
  if (!isPanning) return;
  zoom.tx = e.clientX - panStart.x;
  zoom.ty = e.clientY - panStart.y;
  applyTransform();
});

window.addEventListener("mouseup", () => {
  if (!isPanning) return;
  isPanning = false;
  canvas.style.cursor = "default";
});

// ── Drag & drop de arquivo ────────────────────────────────────────────────

canvas.addEventListener("dragover", (e) => {
  e.preventDefault();
  canvas.classList.add("drag-over");
});

canvas.addEventListener("dragleave", () => {
  canvas.classList.remove("drag-over");
});

canvas.addEventListener("drop", async (e) => {
  e.preventDefault();
  canvas.classList.remove("drag-over");

  const file = e.dataTransfer?.files?.[0];
  if (!file) return;

  // `file.path` é propriedade do Electron/Tauri (não existe no browser padrão)
  // No Tauri, o evento de drop expõe o caminho real do arquivo
  const path = file.path ?? e.dataTransfer.getData("text/plain");
  if (path) {
    await openFile(path);
  }
});

// ── Ciclo de fundo (tecla B) ──────────────────────────────────────────────

const BG_CYCLE = [
  "#111111",          // escuro (padrão)
  "#ffffff",          // branco
  "#000000",          // preto
  "var(--checker)",   // xadrez (transparência)
];
let bgIndex = 0;

// Injeta o padrão xadrez via CSS custom property
document.documentElement.style.setProperty(
  "--checker",
  "repeating-conic-gradient(#2a2a2a 0% 25%, #1a1a1a 0% 50%) 0 0 / 16px 16px",
);

function cycleBg() {
  bgIndex = (bgIndex + 1) % BG_CYCLE.length;
  canvas.style.background = BG_CYCLE[bgIndex];
}

// ── Flash de mensagem na barra de status ─────────────────────────────────

let flashTimer = null;

/**
 * Exibe uma mensagem temporária no campo de nome da barra de status.
 * Restaura o conteúdo anterior após 1.5 s.
 */
function flashStatus(msg) {
  const prev = statusName.textContent;
  statusName.textContent = msg;
  if (flashTimer) clearTimeout(flashTimer);
  flashTimer = setTimeout(() => {
    statusName.textContent = prev;
  }, 1500);
}
