/*
Auther: Evan Drake
C queue
discription:
singly linked list that pops the "front" or "head" and adds to the "back" or "tail".
*/
#include <stdio.h>
#include <stdlib.h>

struct node
{
	struct node* next;			// this is the next node in the list containing the next "key" or "value"
	int value;				// this is the key or data actually being stored
};

