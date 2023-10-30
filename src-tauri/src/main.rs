// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::battleship_engine::BattleshipEngine;
use battleship_engine::ship_status::ShipStatus;

mod battleship_engine;

#[derive(serde::Serialize)]
struct ShotResult {
    battleship_engine: BattleshipEngine,
    ship_status: Option<ShipStatus>,
}

#[tauri::command]
fn battleship_engine(rows: Option<usize>, columns: Option<usize>) -> BattleshipEngine {
    BattleshipEngine::new(rows, columns)
}

#[tauri::command]
fn take_shot(mut battleship_engine: BattleshipEngine, row: usize, column: usize) -> ShotResult {
    let ship_status = battleship_engine.take_shot(row, column).cloned();
    ShotResult {
        battleship_engine,
        ship_status,
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![battleship_engine, take_shot])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
