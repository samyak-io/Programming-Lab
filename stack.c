// creating an array

#include <stdio.h>
#include <errno.h>

// int add(int *stack, &top, int val){
//     top++;
//     stack[top] = val;

//     return top;
// }

void add(int *stack, int *top, int val){
    (*top)++;
    stack[*top] = val;
    
}

int pop(int *stack, int *top){
    if (*top < 0){
        printf("Empty Stack");
        return -1;
    };

    (*top)--;
    return *(stack + *top + 1);
}

int main(){
    int a[10] = {0};
    int top = -1;


    add(a, &top, 23);
    add(a, &top, 46);

    printf("%d\n", *(a + top));

    return 0;    
}
