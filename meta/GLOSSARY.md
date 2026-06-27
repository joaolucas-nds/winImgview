# GLOSSARY.md — Termos do Projeto

> **Opcional.** Use quando o projeto tem vocabulário próprio (nomes de módulos, conceitos, identificadores) que o assistente reexplicaria a cada sessão sem isto.
> Mantenha curto: só o que não é óbvio para alguém de fora.

---

## Conceitos do projeto

- **SVG vetorial** — SVG renderizado como gráfico vetorial real pelo engine web (WebView2/Chromium). Escala a qualquer zoom sem pixel. Contraste com «SVG rasterizado».
- **SVG rasterizado** — SVG convertido a bitmap (imagem de pixels) pelo viewer *antes* de exibir. Funciona na resolução original, mas pixela qualquer zoom. O que IrfanView, XnView, FastStone e PicView fazem atualmente com SVG.
- **WebView2** — componente Chromium embutido da Microsoft, distribuído como parte do Windows (instalado por padrão no Win10/11 via Edge). Permite que apps nativas renderizem HTML/SVG/CSS com qualidade de browser, sem abrir o browser. É o motor que dá ao ImageGlass 9 sua qualidade de SVG.
- **Engine web** — termo genérico para qualquer motor de renderização de padrões web (WebView2, WebKitGTK, Gecko). Para SVG vetorial, é o único caminho sem escribir um renderer próprio do zero.
- **Portable** — versão de um programa distribuída como ZIP/pasta, sem instalador, sem entradas no Registro do Windows, executável de qualquer local (inclusive pendrive). Preferido neste projeto por zero rastro e atualização por substituição.
- **Gap** — funcionalidade necessária para o workflow do usuário que nenhuma das ferramentas existentes cobre satisfatoriamente. Só a confirmação de gap real (Fase 3) justifica partir para o build customizado (Fase 4).

## Arquiteturas / módulos

- **Tauri 2.x** — framework para apps desktop multiplataforma usando Rust como backend e WebView2 (Windows) / WebKitGTK (Linux) / WKWebView (macOS) como frontend. Resultado: executável leve (<10 MB), SVG vetorial nativo via engine web, performance de app nativo.
- **Magick.NET** — binding .NET da biblioteca ImageMagick. Usado por PicView e versões antigas do ImageGlass para decodificar formatos raster. Não renderiza SVG como vetor — converte para bitmap primeiro.
- **Shell extension** — DLL registrada no Windows que se integra ao Explorer para fornecer thumbnails, menus de contexto e pré-visualizações de formatos não suportados nativamente (ex: SVG, WebP em versões antigas do Windows).

## Comandos / artefatos

- **`github.com/d2phap/ImageGlass/releases`** — página oficial de releases do ImageGlass. Baixar sempre `ImageGlass_9.x.x.xxx_x64_portable.zip` (nunca o `.msi`).
- **`faststone.org/FSViewerDownload.htm`** — página oficial de download do FastStone Image Viewer.
- **`%AppData%\ImageGlass\`** — pasta onde o ImageGlass armazena configurações (mesmo no modo portable). Backup desta pasta preserva todas as preferências do usuário.

## Identificadores

- **F1–F4** — fases do ROADMAP: F1 Pesquisa (concluída), F2 Adoção, F3 Avaliação de Gap, F4 Build (condicional).
- **DEC-N** — decisão de arquitetura registrada em DECISIONS.md.
- **FIX-N** — bug grave registrado em DECISIONS.md (não há nenhum ainda; projeto em fase pré-código).
- **Classic (ImageGlass)** — tier gratuito do ImageGlass, sem limitações de funcionalidade. O tier «Kobe» é pago e adiciona temas e suporte prioritário; não é necessário para os fins deste projeto.
