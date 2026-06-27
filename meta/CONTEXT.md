# CONTEXT.md — WinImgView

> Arquivo **estável**. O assistente lê no início de cada sessão para se ambientar.
> Muda pouco: só em alteração estrutural (stack, arquitetura, escopo, nova armadilha descoberta).
> Mantém enxuto — descreve o que o projeto É, não o que está acontecendo agora (isso é o STATUS).

---

## Visão Geral

Encontrar e/ou construir o viewer de imagens ideal para Windows com foco em **SVG vetorial** e **WebP**, que não abra no navegador, não trave, não pixele no zoom, e permita navegar entre imagens (próxima/anterior), zoom fluido e qualidade perfeita de exibição. O visualizador padrão da Microsoft (Photos) e o Windows Photo Viewer legado são descartados. O projeto passa por três fases: pesquisa (concluída), adoção de ferramenta existente, e — se gap real for identificado — desenvolvimento de viewer customizado.

**Usuário-alvo:** desenvolvedor/designer no Windows que trabalha com SVG, WebP, PNG e formatos modernos no dia a dia.

---

## Stack Tecnológica (Fase 4 — condicional, se build for necessário)

- **Linguagem:** Rust (backend) + HTML/CSS/JS (UI via WebView2)
- **Framework:** Tauri 2.x — combina Rust nativo + WebView2 para renderização SVG perfeita sem abrir browser
- **Alternativa avaliada:** Electron (mais pesado, mas mesma qualidade SVG)
- **Renderização SVG:** WebView2 (Chromium embutido via Tauri/Electron) — único caminho para SVG vetorial sem browser em apps nativas Windows
- **Formatos raster:** image-rs (Rust) ou bindings para ImageMagick
- **Deploy:** executável portable .exe; sem instalador; sem dependências externas

> Nota: a stack acima só entra em jogo se a Fase 3 confirmar que ImageGlass 9/10 não satisfaz.

---

## Ferramentas Avaliadas na Pesquisa (Fase 1)

| Viewer | SVG | WebP | Grátis | Open Source | Obs. |
|---|---|---|---|---|---|
| **ImageGlass 9.5** | ✅ Vetorial (WebView2) | ✅ | ✅ Classic | ✅ | **Recomendado primário** |
| **ImageGlass 10 Beta 2** | ✅ Vetorial nativo | ✅ | ✅ Classic | ✅ | Beta — aguardar GA |
| **PicView 3.x** | ⚠️ Via Magick.NET (pode pixelar) | ✅ animado | ✅ | ✅ | Alternativa leve |
| **FastStone 8.5** | ❌ Não suporta | ✅ estático | ✅ pessoal | ❌ | Melhor para raster/RAW/batch |
| **IrfanView 4.73** | ❌ Rasteriza/pixela | ✅ | ✅ pessoal | ❌ | Forum próprio sugere usar Chrome para SVG |
| **XnView MP 1.x** | ⚠️ Rasteriza | ✅ | ✅ pessoal | ❌ | SVG sem transparência/gallery |
| **Honeyview 5.53** | ❌ | ⚠️ | ✅ | ❌ | **DESCONTINUADO** → BandiView (ad-supported) |
| **Smart SVG Viewer** | ✅ (thumbnail!) | ❌ | ✅ | ❌ | **Alpha** — só SVG, instável |

**Escolha adotada:** ImageGlass 9.5 (SVG + WebP + 90+ formatos) + FastStone 8.5 (backup para raster avançado, RAW, batch).

---

## Como o ImageGlass renderiza SVG (CRÍTICO)

ImageGlass v9+ usa um componente chamado **WebView2** (Chromium da Microsoft embutido) especificamente para exibir SVG. Isso significa que o SVG é renderizado pelo mesmo motor do Edge/Chrome — vetor real, escala infinita, sem pixel. O arquivo *não abre no navegador*; a renderização ocorre dentro da janela do ImageGlass. ImageGlass v10 (beta) vai além: renderiza SVG nativamente sem WebView2, com qualidade equivalente.

**Limitação do modo WebView2 (v9):** quando exibindo um SVG, as funções de Rotação, Flip, Crop, Color Picker, Print, Set as Desktop/Lock screen e Export Frames ficam desabilitadas. Para essas operações, trocar temporariamente para o engine nativo (Settings → desmarcar "Use WebView2 for viewing SVG format") — mas aí pixela no zoom.

---

## Arquitetura — pontos-chave

- ImageGlass 9.5 em modo **portable** (ZIP do GitHub): zero instalação, zero Opera prompt, zero rastro no sistema — ver DEC-003.
- FastStone 8.5 em modo **portable**: mesma vantagem.
- SVG só roda bem como vetor com engine web (WebView2/Chromium) — qualquer viewer que "suporta SVG" mas não usa um engine web vai rasterizar — ver DEC-002.
- ImageGlass 10 está em beta (jun/2026); o roadmap do projeto prevê migrar quando GA — ver ROADMAP.md.

---

## Armadilhas Conhecidas (o que NÃO fazer)

1. **Baixar ImageGlass pelo MSI do site oficial** → o instalador .msi tem patrocínio da Opera e exibe prompt para instalar o navegador durante setup. Usar sempre o **portable .zip** do GitHub: `github.com/d2phap/ImageGlass/releases` — a versão portable não tem o prompt.

2. **Usar IrfanView, XnView Classic, ou FastStone para SVG** → eles rasterizam o SVG internamente (convertem para bitmap antes de exibir). Resultado: funciona na resolução original, mas pixela qualquer zoom. O fórum oficial do IrfanView chega a sugerir abrir SVG no Chrome. Não usar para SVG.

3. **Instalar Honeyview ou BandiView** → Honeyview foi descontinuado em 2024 (última versão: 5.53, jun/2024). O substituto oficial, BandiView, é atualmente trial/ad-supported segundo Softpedia. Evitar ambos.

4. **Baixar qualquer viewer de agregadores** (Softonic, CNET, UptoDown, etc.) → risco de bundle com adware. Sempre usar o site oficial ou o repositório GitHub do projeto.

5. **Usar ImageGlass 10 Beta em produção** → é beta; pode travar, perder configurações, ter bugs visuais. Aguardar GA (sem data confirmada, set/2026 provável). Para uso diário: v9.5.

6. **Esperar que PicView renderize SVG como vetor** → PicView usa Magick.NET como backend, que rasteriza SVG antes de exibir. Para WebP e raster, PicView é excelente e open source. Para SVG vetorial, preferir ImageGlass.

---

## Contexto de Produto

- **Usuário-alvo:** dev/designer no Windows que abre SVG e WebP com frequência no workflow diário (assets, ícones, exportações de design)
- **Dor que resolve:** viewers nativos do Windows são ruins (Photos lento, Photo Viewer legado sem WebP/SVG), browsers abrem mas tiram do contexto, viewers tradicionais pixelam SVG
- **O que é sucesso:** abrir SVG com zoom infinito sem pixel, navegar entre imagens de uma pasta com setas, abrir WebP animado, não travar, arrancar em menos de 1 segundo
- **O que o projeto deliberadamente NÃO é:** editor de imagem (para isso: GIMP, Photoshop, Inkscape), organizador de galeria com banco de dados (para isso: FastStone organizer, digiKam), conversor batch dedicado (FastStone ou XnConvert)
