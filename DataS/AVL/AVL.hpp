#include <iostream>

class Avl {
	private:
		struct Node
		{
			// node constructor
			Node(int k, Node* r, Node* l)
			{
				key = k;
				rchild = r;
				lchild = l;
			}
			~Node()
			{
				rchild = nullptr;
				delete rchild;
				lchild = nullptr;
				delete lchild;
			}
			// data that is being stored in node
			int key;
			Node* rchild;
			Node* lchild;
		};
		Node* root;
		void Insert(Node* , int );
		void Print(Node* );
	public:
		Avl();
		~Avl();
		void Insert(int k){ Insert(root, k); }
		void Print(){ Print(root); std::printf("\n"); }
};
