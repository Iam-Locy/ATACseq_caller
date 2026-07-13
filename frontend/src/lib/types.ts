export type File = {
    sample: string;
    assigned_reads: number;
    peaks: number;
    frip: number;
};

export type Peak = {
    sample: string;
    chrom: string;
    start: number;
    end: number;
    name: string;
    score: number;
    strand: string;
    signal_value: number;
    p_value: number;
    q_value: number;
    peak: number;
};
