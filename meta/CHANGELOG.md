# CHANGELOG

> Formato baseado em [Keep a Changelog](https://keepachangelog.com/pt-BR/) e versionamento [SemVer](https://semver.org/lang/pt-BR/).
> **Cresce**: entradas novas no topo. Registra só o que foi de fato concluído/entregue.

---

## [Não lançado]
### Em andamento
- Adoção e configuração do ImageGlass 9.5 portable como viewer padrão (Fase 2)

---

## [0.1.0] — 2026-06-26
### Adicionado
- Pesquisa aprofundada de viewers Windows para SVG e WebP: 8 ferramentas avaliadas (ImageGlass 9.5, ImageGlass 10 Beta 2, PicView 3.x, FastStone 8.5, IrfanView 4.73, XnView MP, Honeyview/BandiView, Smart SVG Viewer)
- Tabela comparativa de viewers com critérios: suporte SVG vetorial, WebP, gratuidade, open source, segurança (CONTEXT.md)
- Identificação da limitação fundamental: SVG vetorial exige engine web (WebView2/Chromium); viewers nativos sem engine web rasterizam e pixelam no zoom
- Seleção de combinação primária: ImageGlass 9.5 (SVG + WebP) + FastStone 8.5 (raster/RAW/batch)
- Armadilhas mapeadas: MSI com prompt Opera, Honeyview descontinuado, IrfanView inadequado para SVG, agregadores com adware (CONTEXT.md)
- Stack de build condicional identificada: Tauri 2.x + WebView2 (Fase 4 condicional)
- Documentação completa de contexto: CONTEXT.md, STATUS.md, DECISIONS.md, CHANGELOG.md, IDEAS.md, ROADMAP.md, GLOSSARY.md, HISTORICO.md, logs/2026-06-26.md
- Três decisões registradas: DEC-001 (pesquisar antes de construir), DEC-002 (ImageGlass + FastStone), DEC-003 (fonte oficial + portable)
