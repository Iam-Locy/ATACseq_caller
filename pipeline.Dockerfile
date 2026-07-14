FROM condaforge/mambaforge:latest

WORKDIR /pipeline

RUN mamba install -y -c bioconda -c conda-forge snakemake-minimal && \
    mamba clean -afy

COPY workflow ./workflow

ENTRYPOINT ["snakemake", "--snakefile", "workflow/Snakefile", "--use-conda", "--conda-frontend", "mamba", "--cores", "10"]
