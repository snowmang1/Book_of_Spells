// Evan Drake
/* this program will create a genral use password of a given length.
all upper/lower case letters will be used along with a handfull of symbols.
speed of this program is not a goal. the goal is security.
this program will create a password of any given length so long as it fits in a long */
#include <iostream>
#include <ctime>
#include <cstdlib>

using std::cout;
using std::cin;
using std::endl;
using std::cerr;

int main(int argc, char *argv[]){
	char lower[26];
	char upper[26];
	char symbols[31] = {'!','#','$', '%', '&', '\\','~','(',')', '*', '+', ',', '-', '/', ':', ';', '<', '=', '>', '?', '@', '[', ']', '^', '_', '`', '{', '|', '}'};
	char num[10] = {'0','1','2','3','4','5','6','7','8','9'};
	///////////////////preset variabels
	if(argc <= 1){
		cerr << "ERROR: EXACTLY ONE PARAMETER MUST BE PASSED\n";
		return 0;
	}
	int len = std::atoi(argv[1]);
	char pWord[(unsigned int)len];

	srand(time(NULL));//getting current time
	////////////////////////////setting up letters with ascii conversion
	for(int i = 65;i <= 90;i++){
		upper[i-65] = i;
	}
	for(int i = 97;i <= 122;i++){
		lower[i-97] = i;
	}
	///////////////////////////////////////////////////////////////////////////done setting up
	int choice = 0;
	for(int i = 0;i < len;i++){
		choice = rand()%4;
		if(choice == 0){
			pWord[i] = lower[rand()%26];
		}else if(choice == 1){
			pWord[i] = upper[rand()%26];
		}else if(choice == 2){
			pWord[i] = symbols[rand()%31];
		}else if(choice <= 3){
			pWord[i] = num[rand()%10];
		}
		cout << pWord[i] << " ";
	}
	cout << endl;
	
	
	return 0;
}
