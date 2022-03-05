#include "AVL.hpp"

Avl::Avl()
{
	root = nullptr;
}
Avl::~Avl()
{
	root = nullptr;
	delete root;
}

void Avl::Insert(Node* cur,int key)
{
	if( cur == nullptr )
	{
		cur = new Node( key, nullptr, nullptr );
		root = cur;
	}
	else if( cur->key > key )
	{
		// go left
		if( cur->lchild == nullptr )
			{ cur->lchild = new Node( key, nullptr, nullptr ); }
		else
			{ Insert(cur->lchild, key); }
	}
	else
	{
		// go right
		if( cur->rchild == nullptr )
			{ cur->rchild = new Node( key, nullptr, nullptr ); }
		else
			{ Insert(cur->rchild, key); }
	}
}

void Avl::Print(Node* cur)
{
	if( cur != nullptr )
	{
		Print(cur->lchild);
		std::printf("%d ", cur->key);
		Print(cur->rchild);
	}
}
