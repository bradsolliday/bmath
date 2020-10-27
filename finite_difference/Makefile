CC = g++
CFLAGS = -I /usr/include/python2.7 -lpython2.7 -I ~/.local/lib/python2.7/site-packages/numpy/core/include/

# Specifying the compiler flags (note, variables are supposed to be in call
# caps for some reason):
release: finite_difference_1.cpp
	$(CC) finite_difference_1.cpp -O2 $(CFLAGS) -o bin/finite_difference_1

debug: finite_difference_1.cpp
	$(CC) finite_difference_1.cpp -g $(CFLAGS) -o bin/finite_difference_1

clean:
	$(RM) bin/finite_difference_1

