using DataFrames
using CSV

df1 = CSV.read("d1.tsv", DataFrame; header=1, delim="\t")

println(df1)
