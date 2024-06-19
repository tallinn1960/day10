// C++ Program to split a string by a delimiter 

#include <iostream> 
#include <sstream> 
#include <vector> 

using namespace std; 

int main() 
{ 
	// Input string 
	string inputString = "geeks,for,geeks"; 

	// Create a stringstream object with the input string 
	stringstream ss(inputString); 

	// Tokenize the input string by comma delimiter 
	string token; 
	vector<string> tokens; 
	char delimiter = ','; 

	while (getline(ss, token, delimiter)) { 
		tokens.push_back(token); 
	} 
    inputString = "hello,world";
    
	// Output the string after splitting 
	cout << "String after splitting: " << endl; 
	for (const auto& part : tokens) { 
		cout << part << endl; 
	} 

	return 0; 
}
