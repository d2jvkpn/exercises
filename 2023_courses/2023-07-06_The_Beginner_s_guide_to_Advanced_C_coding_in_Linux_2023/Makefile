# make

# include envfile
# export $(shell sed 's/=.*//' envfile)

_path = $(shell mkdir -p target && pwd)

# MESSAGE1=YES MESSAGE2=NG make hello MESSAGE2=OK
hello:
	echo "~~~ Hello, $(MESSAGE1), $(MESSAGE2), $(_path)"

# C_FILE=hello.c make run
run:
	bash ../C_Run.sh $(C_FILE)
