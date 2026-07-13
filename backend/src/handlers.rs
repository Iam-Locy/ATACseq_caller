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
    let mut stmt = conn.prepare("
        SELECT 
            sample,
            assigned_reads,
            peaks,
            frip 
        FROM files
        ORDER BY sample;")?;

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
pub async fn get_peaks(
    Path(sample): Path<String>,
    State(state): State<AppState>
) -> Json<Vec<Peak>>{
    let peaks = tokio::task::spawn_blocking(move || {
        let conn = Connection::open(&*state.db_path).unwrap();
        get_peaks_from_db(&conn, &sample).unwrap()
    }).await.unwrap();

    Json(peaks)
}


fn get_peaks_from_db(conn: &Connection, sample: &str) -> Result<Vec<Peak>>{
    let mut stmt = conn.prepare(
        "SELECT sample,
                chrom,
                start,
                \"end\",
                name,
                score,
                strand,
                signalValue,
                pValue,
                qValue,
                peak
        FROM peaks
        WHERE sample = ?;"
    )?;

    stmt.query_map([sample], |row| {
        Ok(Peak {
            sample: row.get(0)?,
            chrom: row.get(1)?,
            start: row.get(2)?,
            end: row.get(3)?,
            name: row.get(4)?,
            score: row.get(5)?,
            strand: row.get(6)?,
            signal_value: row.get(7)?,
            p_value: row.get(8)?,
            q_value: row.get(9)?,
            peak: row.get(10)?,
        })
    })?.collect::<Result<Vec<Peak>>>()
}