#include <stdio.h>

// interface

int incpp(int i);
void dumppp(char* message);

extern "C" int inc(int i) { 
	return incpp(i); 
}

extern "C" void dump(char* message) { 
	return dumppp(message); 
}


// implementation

static int counter;

int incpp(int i) {
	counter += i;
	return counter;
}

void dumppp(char* message) {
	printf("%s: %d\n", message, counter);
}