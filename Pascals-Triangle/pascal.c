/* A simple attempt at implementing a Pascal's triangle calculating program */
#include <stdlib.h>
#include <stdio.h>

#define MAX_DISPLAYABLE 10


int triangle[100][100];

int loop() {
	int maxRows=1;
	
	printf("Please enter the number of rows you would like to calculate (or 0 to exit): ");
	scanf("%d",&maxRows);
	
	if (maxRows==0) return 0;
	
	if (maxRows<=MAX_DISPLAYABLE) {

	int row;
	for (row=1;row<maxRows+1;row++) {
			printf("%d ",row);
			int space;
				for(space=0; space<(maxRows-row)/2+1; space++) {
						printf("\t");
						
				}
				
				
			int col;
			for (col=0; col<row+1; col++)     {
				
				int out=1;
				
				if (row>0 && col>0) out = triangle[row-1][col-1]+triangle[row-1][col];
				
				triangle[row][col] = out;
				printf("%d\t",out);
			}
			
			
			//form triangle
			printf("\n");
		}
	
	}

	else printf("Sorry, I can only neatly display %d rows.. \n", MAX_DISPLAYABLE);
	
	
	printf("\n\nDone! \n");
	loop();
}

int main() {
	printf("Hai \n");
	printf("--PASCALS TRIANGLE CALCULATOR--\n");
	
	loop();
	
	return 0;
}
