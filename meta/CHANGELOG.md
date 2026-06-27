# CHANGELOG

> Formato baseado em [Keep a Changelog](https://keepachangelog.com/pt-BR/) e versionamento [SemVer](https://semver.org/lang/pt-BR/).
> **Cresce**: entradas novas no topo. Registra só o que foi de fato concluído/entregue.

---

## [Não lançado]
### Em andamento
- Primeiro build e testes na máquina do usuário

---

## [0.2.0] — 2026-06-27
### Adicionado
- Estrutura Tauri 2.x completa: `src-tauri/Cargo.toml`, `tauri.conf.json`, `capabilities/default.json`, `build.rs`
- Backend Rust `lib.rs`: comandos `open_file`, `next_image`, `prev_image`, `get_position`
- Navegação de pasta com ordenação alfabética case-insensitive e wrap-around
- Três estratégias de renderização: SVG (iframe asset://), raster nativo (img asset://), formatos não-nativos (base64 PNG via crate `image`)
- Frontend `index.html` + `css/style.css` + `js/main.js`
- Zoom por roda do mouse (centrado no cursor) e teclado (Ctrl+/-, Ctrl+0, Ctrl+1)
- Pan por arrastar com mouse
- Drag & drop de arquivo sobre a janela
- Atalhos: ← →, F11, B (ciclo de fundo), Ctrl+Shift+C (copy-to-path)
- Barra de status: posição na pasta, nome do arquivo, zoom atual
- Abertura por argv[1] para integração com duplo-clique no Explorer
- SETUP.md com guia completo de instalação dos pré-requisitos e primeiro build
- DEC-004: decisão de pular F2/F3 e ir direto ao build customizado

---

## [0.1.0] — 2026-06-26
### Adicionado
- Pesquisa aprofundada de viewers Windows para SVG e WebP: 8 ferramentas avaliadas
- Tabela comparativa de viewers com critérios: suporte SVG vetorial, WebP, gratuidade, open source, segurança
- Identificação da limitação fundamental: SVG vetorial exige engine web (WebView2/Chromium)
- Seleção de combinação primária de referência: ImageGlass 9.5 + FastStone 8.5
- Armadilhas mapeadas: MSI com prompt Opera, Honeyview descontinuado, IrfanView inadequado para SVG
- Stack de build identificada: Tauri 2.x + WebView2
- Documentação completa de contexto: CONTEXT.md, STATUS.md, DECISIONS.md, CHANGELOG.md, IDEAS.md, ROADMAP.md, GLOSSARY.md, HISTORICO.md
- Três decisões registradas: DEC-001, DEC-002, DEC-003
