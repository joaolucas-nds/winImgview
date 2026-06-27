// Oculta a janela de console no Windows em builds de release.
// Em debug, a janela é mantida para ver logs de erro.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    winimgview_lib::run();
}
