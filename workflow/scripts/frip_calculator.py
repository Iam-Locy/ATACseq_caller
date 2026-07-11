import argparse
import os

parser = argparse.ArgumentParser(
    prog= "FRiP calculator", 
    description= "Calculates the Fraction of Reads in Peaks metric for the input file.")

parser.add_argument('--input-folder','-in')
parser.add_argument('--output-file','-out')

args = parser.parse_args()

out_txt = "sample;assigned_reads;peaks;frip\n"

for root, dirs, files in os.walk(args.input_folder):
    for file in files:
        peak_count = 0
        if not "summary" in file:
            with open(f"{root}/{file}", "r") as cnt_f:
                peak_count = len(cnt_f.readlines()) - 2  #Two line header.

            assigned = 0
            all = 0

            with open(f"{root}/{file}.summary", "r") as sum_f:
                sum_f.readline()
                for line in sum_f:
                    line = line.strip().split("\t")
                    
                    if line[0] == "Assigned":
                        assigned = int(line[1])
                    
                    all += int(line[1])

            out_txt += f"{file.replace(".counts", "")};{assigned};{peak_count};{(assigned/all):.4f}\n"

with open(args.output_file, "w") as out_f:
    out_f.write(out_txt)