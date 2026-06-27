# ROADMAP.md — Plano Intencional de Evolução

> **Opcional.** Use quando o projeto tem um plano em fases — não para tarefas soltas (isso é o Backlog do STATUS) nem para brainstorm (isso é o IDEAS).
> Cada fase tem um objetivo e um critério de conclusão. Marque o estado: 🟢 concluída · 🟡 em curso/próxima · 🔵 futura · 🚫 descartada.
> Médio e longo prazo vivem AQUI, não no STATUS.

---

## 🟢 F1 — Pesquisa de Mercado *(concluída — 2026-06-26)*

**Objetivo:** Identificar se já existe um viewer gratuito, seguro e sem anúncios que resolva o problema (SVG vetorial + WebP + navegação fluida no Windows) antes de escrever uma linha de código.

**Critério de conclusão:** Tabela comparativa com ≥6 candidatos avaliados; recomendação clara emitida.

- Pesquisados: ImageGlass (v9.5 e v10 beta), PicView, FastStone, IrfanView, XnView MP, Honeyview/BandiView, Smart SVG Viewer
- Resultado: ImageGlass 9.5 (SVG vetorial via WebView2) + FastStone 8.5 (raster/batch) eleitos
- Armadilhas mapeadas e documentadas (CONTEXT.md)
- Decisões DEC-001 a DEC-003 registradas (DECISIONS.md)

---

## 🟡 F2 — Adoção e Configuração *(próxima)*

**Objetivo:** Instalar ImageGlass 9.5 portable + FastStone 8.5 portable na máquina, configurar como defaults e validar no uso real.

**Critério de conclusão:** Abrir SVG e WebP de uma pasta real com navegação, zoom e animação funcionando sem nenhuma abertura de browser, sem travamento.

- [ ] Baixar ImageGlass portable ZIP de `github.com/d2phap/ImageGlass/releases`
- [ ] Extrair para `C:\Ferramentas\ImageGlass\` (ou pasta de preferência)
- [ ] Associar extensões: `.svg`, `.webp`, `.png`, `.jpg`, `.gif`, `.avif`, `.heic`
- [ ] Testar SVG: zoom 400% → sem pixel
- [ ] Testar WebP animado: animação roda dentro do viewer
- [ ] Testar navegação de pasta: setas avançam para próxima imagem
- [ ] Opcional: baixar FastStone portable de `faststone.org` e instalar lado a lado

---

## 🔵 F3 — Avaliação de Gap *(futura — após 1–2 semanas de uso real)*

**Objetivo:** Após uso real do ImageGlass, identificar se alguma necessidade persiste sem cobertura. Decidir: encerrar o projeto (ImageGlass resolve tudo) ou avançar para Fase 4 (build).

**Critério de conclusão:** Decisão binária documentada em DECISIONS.md: «adotar e encerrar» ou «build justificado».

Perguntas a responder:
- Thumbnails SVG na galeria estão nítidos?
- Fundo personalizável (transparência/xadrez) faz falta?
- Comparison mode seria usado regularmente?
- Quick copy-to-path e export rápido fazem diferença no workflow?
- Algum formato crítico não abre bem?

---

## 🔵 F4 — Build Customizado *(condicional — só se F3 confirmar gap)*

**Objetivo:** Construir um viewer mínimo e portable que cubra os gaps identificados na F3, com stack Tauri + WebView2.

**Critério de conclusão:** Executável portable que abre SVG (vetorial), WebP (animado), PNG; navega pasta; tem fundo personalizável; export SVG→PNG; copy-to-path. Sem instalador, sem dependências externas, < 15 MB.

Funcionalidades mínimas (MVP):
- [ ] Janela única com barra de título minimalista
- [ ] Renderização SVG via WebView2 (vetorial, zoom infinito sem pixel)
- [ ] WebP estático e animado
- [ ] Formatos raster padrão: PNG, JPG, GIF, BMP, TIFF
- [ ] Navegação por pasta (setas ← →)
- [ ] Zoom: `+`, `-`, `Ctrl+0` (fit), `Ctrl+1` (100%)
- [ ] Fundo: `B` alterna branco / preto / xadrez
- [ ] Copy-to-path: `Ctrl+Shift+C`
- [ ] Export SVG→PNG: `E`
- [ ] Fullscreen: `F11`

Funcionalidades da v2 (se MVP validado):
- [ ] Painel de metadados SVG
- [ ] Comparison mode (dual-pane)
- [ ] Thumbnails SVG na galeria lateral
- [ ] Shell extension para thumbnails no Explorer

---

## 🚫 Itens descartados desta visão

- **Electron como stack** — descartado em favor de Tauri (mais leve, WebView2 do sistema). Ver IDEAS § Descartadas.
- **Honeyview/BandiView como ferramenta adotada** — descontinuado / ad-supported. Ver CONTEXT § Armadilhas.
