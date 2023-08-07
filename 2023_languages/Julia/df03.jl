using DataFrames, CSV, Tables

df = DataFrame(CSV.File("tmp/df02_a01.tsv"; delim="\t"))

print(df)
