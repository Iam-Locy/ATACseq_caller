use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct File {
    pub sample: String,
    pub assigned_reads: i32,
    pub peaks: i32,
    pub frip: f32,
}

#[derive(Debug, Serialize)]
pub struct Peak {
    pub sample: String,
    pub chrom: String,
    pub start: i32,
    pub end: i32,
    pub name: String,
    pub score: i32, 
    pub strand: String,
    pub signalValue: f32,
    pub pValue: f32,
    pub qValue: f32,
    pub peak: i32
}