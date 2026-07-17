# ATAC-seq peak viewer

Made by: Lőrinc Kóródi (korodilorinc@gmail.com)

## Try with:
  Run peak calling pipeline and create database:
  > docker compose --profile pipeline run --rm --build pipeline

  Start back- and frontend servers:
  > docker compose up --build </br>
  This might take a while (even 10+ mins.) on the first time, because Rust has to bundle DuckDB .

  Open in browser: [frontend](http://localhost:5173)<br/>

  Remove Docker container:
  > docker compose down<br/>

## The data used
To make this project I used the publicly available ATAC-seq data for _Danio rerio_ pronephric duct cells from 4 samples (2 control, 2 actl6a knockdown)[^1]: [data](https://www.ncbi.nlm.nih.gov/geo/query/acc.cgi?acc=GSE274957)

These files were aligned to the [danRer11](https://genome.ucsc.edu/cgi-bin/hgGateway?db=danRer11) reference[^2]. I could not find an ENCODE blacklist for _D. rerio_, therefore I used UCSC's pre-computed [RepeatMasker](https://hgdownload.soe.ucsc.edu/goldenPath/danRer11/bigZips/danRer11.fa.out.gz) + [Tandem Repeats Finder](https://hgdownload.soe.ucsc.edu/goldenPath/danRer11/bigZips/danRer11.trf.bed.gz) annotations as an exclusion filter, which serves the same practical purpose of removing likely-artifactual signal in repetitive regions. These .BAM files are the input files in /resources/input_bam.

## Pipeline
First the aligned and filtered .BAM files are indexed and sorted via [samtools](https://www.htslib.org/). The peak calling is done with [Genrich](https://github.com/jsh58/genrich) which was made with ATAC-seq data in mind. This is followed by [Subread](https://subread.sourceforge.net/)'s `featureCounts` tool[^3] and my own script to calculate QC statistics about the peak calling. As a last step these statistics as well as the peaks are loaded into a [DuckDB](https://duckdb.org/) database.

## Application
The ATAC-seq peak viewer application has an [Axum (Rust)](https://docs.rs/axum/latest/axum/) backend which hosts the data from the DuckDB database. The frontend is a simple [Svelte 5](https://svelte.dev/) + [Vite](https://vite.dev/) stack which uses [igv.js](https://igv.org/doc/igvjs/)[^4] to visualize the peaks.

## References:
  [^1]: Cheng, X., Zhu, Q., Ma, S., Peng, X., Huang, G., Liu, G., Zhang, W., Zhang, Y., Jiang, C., Qiu, A., & Cao, Y. (2025). Epigenetic regulation of cilia stability and kidney development by the SWI/SNF chromatin remodeling complex in zebrafish. Journal of Genetics and Genomics/Journal of Genetics and Genomics, 53(4), 630–642. https://doi.org/10.1016/j.jgg.2025.11.001
 
  [^2]: Howe, K., Clark, M. D., Torroja, C. F., Torrance, J., Berthelot, C., Muffato, M., Collins, J. E., Humphray, S., McLaren, K., Matthews, L., McLaren, S., Sealy, I., Caccamo, M., Churcher, C., Scott, C., Barrett, J. C., Koch, R., Rauch, G., White, S., . . . Stemple, D. L. (2013). The zebrafish reference genome sequence and its relationship to the human genome. Nature, 496(7446), 498–503. https://doi.org/10.1038/nature12111

  [^3]: Liao, Y., Smyth, G. K., & Shi, W. (2013). featureCounts: an efficient general purpose program for assigning sequence reads to genomic features. Bioinformatics, 30(7), 923–930. https://doi.org/10.1093/bioinformatics/btt656

  [^4]: James T Robinson, Helga Thorvaldsdottir, Douglass Turner, Jill P Mesirov, igv.js: an embeddable JavaScript implementation of the Integrative Genomics Viewer (IGV), Bioinformatics, Volume 39, Issue 1, January 2023, btac830, https://doi.org/10.1093/bioinformatics/btac830
