using DataFrames, CSV, Tables

df = DataFrame(CSV.File("tmp_data/dataframe.tsv"; delim="\t"))

print(df)
