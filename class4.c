#include <stdio.h>
#include <stdlib.h>
#include <time.h>

char buf;
int bufsize = 0;

void output(FILE *x, int b) {
    if (bufsize == 8){
        fputc(buf, x);
        bufsize = 0;
    } else if (bufsize == 0) {
        buf = 0;
    }
    buf = (buf << 1) | b;
    bufsize ++;
}


void siftdown(int *H, int i, int n) {
    int p = i; int lc = 2*i + 1; int rc = 2*i + 2;

    if (i >= n){
        return;
    }

    if ((H[p] <= H[lc]) && (H[p] <= H[rc])){
        return;
    } else if (H[lc] <= H[rc]){
        H[lc] = H[p];
        H[p] = H[lc];
        siftdown(H, lc, n);
    } else if (H[rc] <= H[lc]){
        H[rc] = H[p];
        H[p] = H[rc];
        siftdown(H, rc, n);
    }
}

int extract (int *H, int n){
    int t = H[0];
    n--;
    H[0] = H[n];
    siftdown(H, 0, n);
}

int main(){
    int arr[20] = {0};  
    int heap[13] = {5, 19, 15, 23, 81, 18, 20, 30, 40, 90, 82, 25, 42};

    int n = 10;
    //before calling siftdown   
    extract(heap, n);

    srand(time(0));
    // for (int i = 0; i < n; i++){
    //     int num = 50 + rand() % (69 - 50 + 1); 
    //     arr[i] = num;
    //     printf("%d ", arr[i]);
    // }
    for (int i = 0; i < n; i++){
        printf("%d ", heap[i]);
    }
    
    FILE *ptr;

    ptr = fopen("some.txt", "wb");

    if(ptr == NULL){
        printf("error");
    }

    fclose(ptr);
}



//discussing heap

/*
every child should be greater than it's parent. (min heap)

siftdown(int *H, int i, int n)

H[2i+1] <= H[2i+2] then swap H[2i+1] with the parent of the two children

we recursively call siftdown on i -> 2i+1 or 2i+2 whichever was the minimum.

*/

// write code for heapify. figure out the sift ups / sift downs in the clever method.

// write heap sort (maxheap is more natural for ascending order) 

//27th is midterm exam