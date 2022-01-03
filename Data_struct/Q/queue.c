#include "queue.h"

// do everything in main or things get messy

int main()
{
	int i = 0;						// genral use iterator
	struct node* root = NULL;		// this points to the back of the structure
	struct node* cur = NULL;		// helper pointer so that tail is not moved
	
	root = (struct node*) malloc( sizeof(struct node) ); // initial node of the structure
	cur = root;
	//loading the queue
	for( i=0;i<10;i+=1) 
	{
		cur->value = i;
		cur->next = (struct node*) malloc( sizeof(struct node) );
		cur = cur->next;
	}
	//front node values
	cur->value = 10;
	cur->next = NULL;
	//printing queue
	cur = root;	//root is the oldest thing in the queue
	while( cur )
	{
		printf(" %d", cur->value);
		cur = cur->next;
	}
	
	//unloading the queue
	while(root)
	{
		cur = root;
		root = root->next;
		free(cur);
	}
	
	
	return 0;
}
