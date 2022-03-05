
#include "main.hpp"

using std::printf;

int main()
{
	//test1
	Avl tree;
	tree.Insert(5);
	tree.Insert(4);
	tree.Insert(6);
	tree.Print();
	//test2
	Avl tree1;
	tree1.Insert(20);
	tree1.Insert(10);
	tree1.Insert(30);
	tree1.Insert(0);
	tree1.Insert(9);
	tree1.Insert(1);
	tree1.Insert(3);
	tree1.Print();
}
