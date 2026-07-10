import argparse

parser = argparse.ArgumentParser(
    prog= "FRiP calculator", 
    description= "Calculates the Fraction of Reads in Peaks metric for the input file.")

parser.add_argument('--input-file','-in')
parser.add_argument('--output-file','-out')

args = parser.parse_args()

assigned = 0
all = 0

with open(args.input_file, "r") as in_f:
    in_f.readline()
    for line in in_f:
        line = line.strip().split("\t")
        
        if line[0] == "Assigned":
            assigned = int(line[1])
        
        all += int(line[1])

with open(args.output_file, "w") as out_f:
    out_f.write(f"fraction;{(assigned/all):.4f}\npercentage;{(assigned/all)*100:.2f}\n")