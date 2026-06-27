# IDEAS.md — Brainstorm e Visão

> **Segundo cérebro** do projeto. Captura TUDO que for mencionado, mesmo solto ou no meio de outro assunto.
> Nunca perde: ideia implementada vai para «Concluídas»; ideia recusada vai para «Descartadas» com o motivo.
> Separar por autor (você × assistente) ajuda a lembrar de onde veio cada coisa.

---

## 💡 Ideias Ativas — Usuário

### 2026-06 — Viewer que não abra no navegador
Não ser jogado no browser para ver SVG. Sem lentidão, sem travamento. Resolução perfeita. Navegar para próxima/anterior dentro da pasta. Zoom bom. Suporte a SVG e WebP.
*(Motivação original do projeto — parcialmente atendida pelo ImageGlass 9.5; avaliar se gap persiste após uso real.)*

---

## 🤖 Ideias Ativas — Assistente

### 2026-06-26 — Thumbnail SVG vetorial no painel de galeria
ImageGlass mostra thumbnails de SVG na galeria, mas a geração de thumb pode rasterizar em resoluções pequenas (perda de nitidez em ícones). Um viewer customizado poderia usar WebView2 para renderizar os thumbnails também — resultando em SVG perfeitamente nítido mesmo em 64×64.

### 2026-06-26 — Preview de fundo personalizável
SVGs e WebPs com transparência ficam ambíguos em fundo branco padrão. Adicionar toggle de fundo rápido: branco / preto / grid xadrez (padrão Photoshop) / cor customizada. Um único atalho de teclado alterna entre os modos.

### 2026-06-26 — Painel de metadados SVG
Ao abrir um SVG, exibir painel lateral colapsável com: dimensões (viewBox), presença de animações (`<animate>`, `<animateTransform>`), número de elementos, paleta de cores usadas (extrai `fill` e `stroke` do DOM). Útil para diagnóstico rápido sem abrir editor.

### 2026-06-26 — Comparison mode (dual-pane sincronizado)
Dois painéis lado a lado, zoom e pan sincronizados. Útil para comparar SVG original vs. otimizado (SVGO), ou WebP vs. PNG de mesma cena. Tira o peso de trocar entre janelas.

### 2026-06-26 — Quick Copy-to-Path
Hotkey (ex: `Ctrl+Shift+C`) copia o caminho completo do arquivo atual para a área de transferência. Economiza o ritual de ir ao Explorer → clicar com botão direito → Propriedades → copiar. Muito útil em workflow de dev.

### 2026-06-26 — Export rápido SVG→PNG
Direto no viewer: `E` abre dialog de export com resolução (1×, 2×, 4×, px custom) e salva PNG na mesma pasta. Evita abrir Inkscape ou ImageMagick só para uma conversão rápida.

### 2026-06-26 — Shell extension para thumbnails SVG no Explorer
O Windows não renderiza thumbnails de SVG no Explorer (aparece ícone genérico). Uma shell extension leve (DLL registrada) pode usar WebView2 para gerar thumbs vetoriais. Projeto separado, mas high-value para o workflow.

### 2026-06-26 — Modo "designer workflow": atalho para abrir no editor
`Shift+Enter` (ou configurável) abre o arquivo atual no editor associado (Inkscape para SVG, Figma via browser, VS Code). Evita ter que ir ao Explorer. Complementa o viewer, não substitui o editor.

### 2026-06-26 — Suporte a SVG animado com controles de playback
SVG pode ter animações SMIL (`<animate>`) ou CSS. Viewer mostra botão play/pause, velocidade (0.5×, 1×, 2×), e frame-by-frame. Hoje nenhum viewer gratuito faz isso bem.

---

## ✅ Concluídas
*(nenhuma ainda — projeto em fase inicial)*

---

## 🚫 Descartadas

- **Honeyview como alternativa** — descartado porque foi descontinuado (última versão jun/2024) e o substituto oficial (BandiView) é trial/ad-supported.
- **IrfanView como viewer SVG** — descartado porque rasteriza SVG (o fórum oficial sugere usar Chrome para SVG). Válido para outros formatos raster, mas não para este projeto.
- **Construir viewer imediatamente (sem pesquisa)** — descartado por DEC-001: pesquisa mostrou que ImageGlass 9.5 cobre o caso de uso primário. Build só se gap real for confirmado.
- **Electron como stack de build** — preterido em favor de Tauri: Electron é 3–5× mais pesado (Chromium + Node bundled, >100 MB), Tauri usa WebView2 do sistema (~<10 MB app). Para um viewer de imagens que precisa ser rápido, Tauri é mais adequado.

---

## 📝 Feedback para o Kit

### 2026-06-26 — Nenhum desvio estrutural registrado nesta sessão
O kit foi aplicado conforme o template. Próxima sessão: registrar aqui qualquer adaptação estrutural feita.
