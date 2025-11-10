#include <stdio.h>

//non robustly
void *insert(int *q, int val, int *end){
    q[*end] = val;
    *end = *end + 1;
}

int pop( int *q, int *start, int *end){
    if (*start <= *end){
        // --*(start);
        return q[*start - 1];
    }
    else {
        return -1;
    }
}

int main(){
    // int length = 10;
    int start = 0;
    int end = 0;

    int q[10] = {0};

    insert(q, 10, &end);
    insert(q, 20, &end);

    int ele = pop(q, &start, &end);

    printf("%d", ele);
    return 0;
}