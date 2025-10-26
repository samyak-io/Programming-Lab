
#include <stdio.h>

int ssum(int *A, int i, int j){

}

int main(){
    // [-5, 11, 23, -2, -2, 5, ...]
    // maximum subset sum problem

    int n = 10;
    int A[10] = {-5, 11,23,-2,-2,5};

    //brute force approach
    for (int i=0; i<n; i++){
        for (int j=i; j<n; j++){
        }
    }
}

// write all the approaches for finding given some x, A[x] and A[y] such that A[x] + A[y] = X.
// nlogn + n
// nlogn + nlogn



//russian roulette. Josephus problem

//fertilizer problem. (kill plants on rule) keep deleting numbers from the seauence if at any point a number ot the left is greater than the number to the right, that number is killed. after how many iterations, do we reach a list where nothing is going to die. (you hava a monotonically increasing sequence).

//clearly define a set of rules and solve for that

//we're agnostic to the ordering. the only question is, after how many steps does the array become monotonically increasing?

