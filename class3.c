// date: 15th September
#include <stdio.h>

int main(){
    char S[256];
    int i;

    for (i = 0; i < 256; i++){
        S[i] = i;
    }

    for (i = 0; i < 256; i++){
        printf("%d -> %d\n", i, S[i]);
    }

    //read up on 1's complement and 2's complement.

    //in the system, char is an 8 bit signed type, it can only represent -128..127. After that they wrap.

    //Huffman and Shannon-Fano

    /*
    if you want a variable length encoding, you wanna make sure you don't get confused.

    Huffman's approach
    1. Take the two smallest things and combine them and we keep doing that. we temporarily pretend that they're the same.
        in other words take the minimum two things at each step, remove them and put back the sum of them.
    2. label each (of the two) edge back from the root to either 1 or 0. 
    3. for each letter you now have a unique encoding.
    4. the largest frequency will have the shortest length encoding. 
    */

    //figure out why 
    /*
    while((c = fgetc(file)) != EOF){
        F[c] ++             would give an error and why we need to c & 'something'
    }

    //heap briefly introduced in class
    
    */
}