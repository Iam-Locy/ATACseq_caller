use axum::{Json, debug_handler, extract::Path};

#[derive(Debug, serde::Serialize)]
pub struct Sample {
    name: String,
    frip: f32,
}

#[derive(Debug, serde::Serialize)]
pub struct Peak {
    sample: String,
    chrom: String,
    start: i32,
    end: i32,
    name: String,
    score: i32, 
    strand: String,
    signalValue: f32,
    pValue: f32,
    qValue: f32,
    peak: i32
}

pub async fn get_names() -> Json<Vec<Sample>>{
    Json::from(vec![
        Sample{
        name: "SSR00001".to_string(),
        frip: 0.5,
        },
        Sample{
            name: "SSR00002".to_string(),
            frip: 0.33,
        }
    ])
}

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