#include <stdio.h>
#include <stdlib.h>
#include <ctype.h> //can use for isalpha(c)

//create a lookup table
//replace the characters with a permuted version of the original array
//how to create a permutation of the array first


//study Fischer - Yates

/*
for (i = n-1; i > 0; i = i - 1){
    j = rand() % (i+1);
    swap(A,i,j);
}*/

void swap(int* A, int* B){

}

void print_arr(int* array, int size){
    for (int i=0; i<size; i++){
        printf("%d ", *(array + i));
    }

}

void shuffleArray(int *A, int n){
    for (i = n-1; i > 0; i = i - 1){
        j = rand() % (i+1);
        swap(A,i,j);
    }
}

/*
int i;
for (i = rot; i < n; i++){
    A[i-rot] = A[i];
}
for (i = rot; i < n; i = i + 1){A[i-rot] = A[i];}

for (i = 0; i < rot; i = i + 1){A[ n-rot +i ] = buf [i];}
*/

void rotateArray(int *A, int n, int rot){
    int* buf = (int*) malloc(rot * sizeof(int));

    int i;

    for (i = rot; i < n; i++){
        A[i-rot] = A[i];
    }
    for (i = rot; i < n; i++){
        A[i-rot] = A[i];
    }

    memcpy((A + n - rot), buf, rot * sizeof(int)); //when memories are overlapping do not use memcpy
}

int main(){
    srand(12345);

    int S[26] = {0}; 

    for(int i=0; i<26; i++){
        S[i] = i;
    }
    print_arr(S, 26);

    rotateArray(S, 26, 3);

    while (c = fgetc(infile) != EOF){
        if (c >= 'a' && c <= 'z'){
            
        }
    }
}