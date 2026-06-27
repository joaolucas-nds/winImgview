# ROADMAP.md — Plano Intencional de Evolução

> Cada fase tem um objetivo e um critério de conclusão.
> 🟢 concluída · 🟡 em curso/próxima · 🔵 futura · 🚫 descartada

---

## 🟢 F1 — Pesquisa de Mercado *(concluída — 2026-06-26)*

**Objetivo:** Identificar viewers existentes gratuitos, seguros, sem anúncios, com SVG vetorial + WebP + Windows.

- Pesquisados: ImageGlass (v9.5 e v10 beta), PicView, FastStone, IrfanView, XnView MP, Honeyview/BandiView, Smart SVG Viewer
- Resultado: nenhum viewer existente cobre 100% dos casos — build justificado por DEC-004
- Decisões DEC-001 a DEC-003 registradas

---

## 🚫 F2 — Adoção do ImageGlass *(pulada — DEC-004)*

**Motivo:** usuário optou por ir direto ao build customizado. ImageGlass serve como referência de qualidade.

---

## 🚫 F3 — Avaliação de Gap *(pulada — DEC-004)*

**Motivo:** idem F2.

---

## 🟡 F4 — Build Customizado *(em curso — iniciado 2026-06-27)*

**Objetivo:** Viewer mínimo e portable que cobre os casos de uso com qualidade igual ou superior ao ImageGlass 9.5.

**Critério de conclusão do MVP:** abrir SVG (vetorial, zoom infinito), WebP (animado), PNG/JPG; navegar pasta; fundo personalizável; copy-to-path; fullscreen. Sem instalador, < 15 MB, arranca em < 1 s.

### MVP (v0.2.x — scaffolding pronto, aguarda build)
- [x] Estrutura Tauri 2.x + Rust
- [x] Backend: open_file, next_image, prev_image
- [x] Três estratégias de renderização (SVG/iframe, raster/img, base64)
- [x] Frontend: zoom, pan, drag&drop, atalhos, barra de status
- [x] Abertura por argv[1] (duplo-clique no Explorer)
- [ ] Primeiro build na máquina real + correção de bugs
- [ ] Ícone .ico real
- [ ] Associação de extensões no Windows

### v0.3.x — Polimento pós-MVP
- [ ] Export SVG→PNG (`E` abre dialog de resolução)
- [ ] Painel de metadados SVG (viewBox, animações, paleta)
- [ ] Tecla `Delete` move arquivo para lixeira (com confirmação)
- [ ] Suporte a AVIF nativo quando WebView2 atualizar

### v0.4.x — Diferenciais
- [ ] Comparison mode (dual-pane sincronizado)
- [ ] Shell extension para thumbnails SVG no Explorer
- [ ] Modo "abrir no editor" (`Shift+Enter` → abre no programa associado)
- [ ] Controles de playback para SVG animado (SMIL/CSS)
