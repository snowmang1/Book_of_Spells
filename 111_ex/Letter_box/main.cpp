#include <iostream>

using std::cout;
using std::endl;
using std::cin;

int main()
{
	// create vars
	int row = 0;
	int col = 0;
	// ask for row
	cout << "give me a row (1-25)" << endl;
	cin >> row;
	// convert row to capital ASCII letter
	char letter = row + 64;
	// ask for column
	cout << "give me a col" << endl;
	cin >> col;
	// make our box
	for(int i=0;i<row;i++) {
		// col
		for(int j=0;j<col;j++) {
		// rows
			cout << letter;
		}
		cout << endl;
	}
}
