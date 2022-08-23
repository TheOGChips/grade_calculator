MAIN=main
GRADE=Grade
FUNCTIONS=functions
OBJ = $(MAIN).o $(GRADE).o $(FUNCTIONS).o
TGT = grade_calculator.exe

all:	$(OBJ)
	g++ -o $(TGT) $^

$(MAIN).o:	$(MAIN).cpp
		g++ -c $^

$(GRADE).o:	$(GRADE).cpp
		g++ -c $^

$(FUNCTIONS).o:	$(FUNCTIONS).cpp
		g++ -c $^

.PHONY: clean src_only
clean:	$(OBJ)
	rm $^

src_only:	$(TGT) $(OBJ)
		rm $^
