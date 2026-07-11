use axum::{Json, debug_handler, extract::{State, Path}};
use duckdb::{Connection, Result};

use crate::state::AppState;
use crate::models::{Peak, File};


////###################### Handler for GET files #####################

#[debug_handler]
pub async fn get_files(State(state): State<AppState>) -> Json<Vec<File>>{
    let files = tokio::task::spawn_blocking(move || {
        let conn = Connection::open(&*state.db_path).unwrap();
        get_files_from_db(&conn).unwrap()
    }).await.unwrap();

    Json(files)
}

fn get_files_from_db(conn: &Connection) -> Result<Vec<File>>{
    let mut stmt = conn.prepare("SELECT sample, assigned_reads, peaks, frip FROM files;")?;

    stmt.query_map([], |row| {
        Ok(File {
            sample: row.get(0)?,
            assigned_reads: row.get(1)?,
            peaks: row.get(2)?,
            frip: row.get(3)?,
        })
    })?.collect::<Result<Vec<File>>>()
}

//###################### Handler for GET peaks #####################

#[debug_handler]
pub async fn get_peaks(Path(sample): Path<String>) -> Json<Vec<Peak>>{
    println!("Asking for peaks from sample: {}",sample);
    
    Json::from(vec![
        Peak{
            sample: "SSR00001".to_string(),
            chrom: "chr1".to_string(),
            start: 1,
            end: 200,
            name: "peak_1".to_string(),
            score: 400, 
            strand: ".".to_string(),
            signalValue: 31.1,
            pValue: 0.34,
            qValue: 33.3,
            peak: 100
        },
        Peak{
            sample: "SSR00001".to_string(),
            chrom: "chr1".to_string(),
            start: 200,
            end: 400,
            name: "peak_2".to_string(),
            score: 400, 
            strand: ".".to_string(),
            signalValue: 31.1,
            pValue: 0.34,
            qValue: 33.3,
            peak: 100
        }
    ])
}