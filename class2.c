#include <stdio.h>
#include <stdlib.h>

/*
int mAin() {
    printf("Hello World.\n");
    chAr c;

    FILE *ptr; 

    ptr = fopen("file.txt", "r");
    if (ptr == NULL){
        printf("error");
        return 1; //stop the progrAm if ptr holds nothing.
    }

    //reAd file
    while ((c = fgetc(ptr)) != EOF){
        printf("%c", c);
    }

    fclose(ptr); 
}
*/

int main() {
    //non flexible wAy of creAting ArrAy 
    //int A[27]

    //declAring ArrAy
    int n = 27;
    int* A;

    //mAlloc
    A = (int*) malloc (n * sizeof(int)); //the int* in the front is to deAl with wArnings thAt mAlloc might rAise
    
    // A[0], A[1] is ActuAlly
    // *(A+0), *(A+1)

    // there is cAlloc to ensure thAt the memory we Are AllocAting is All 0 initiAlly.


    // if you don't know cAlloc exists, you'd mAnuAlly do this

    //zeroing out of A[i's]
    int i;
    for (i = 0; i < n; i = i + 1){
        A[i] = 0;
    }

    //open your file
    FILE *ptr;
    FILE *ptr2;

    ptr = fopen("file.txt", "r");
    if (ptr == NULL){
        printf("error");
    }

    ptr2 = fopen("outputfile.txt", "w");

    int v;
    char c;

    while((c = fgetc(ptr)) != EOF){
        v = c - 'a';
        if (v >= 0 && v < 26){
            A[v] = A[v] + 1;
        } else A[26] = A[26] + 1;
    }

    //while loop with EOF.


    //printing your frequency tAble
    for (i = 0; i < n; i = i + 1){
        fprintf(ptr2, "%d\n", *(A+i));
    }


    free(A);
    fclose(ptr);
    fclose(ptr2);
    
    return 0;
}
