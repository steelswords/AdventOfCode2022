#include <stdio.h>
#include <stdlib.h>

const char fileName[] = "../day1/elf_inventories.txt";
#define MAX_ELVES 4096
const ssize_t GETLINE_FAILURE = -1;

int main()
{
    char *line = NULL;
    size_t length = 0;
    ssize_t bytesRead;
    size_t numElves = 0;
    size_t indexOfBiggestInventory = 0;
    int biggestInventory = 0;

    size_t currentIndex = 0;
    int elfInventories[MAX_ELVES] = {[0 ... MAX_ELVES - 1] = 0};

    FILE* file = fopen(fileName, "r");

    if(!file)
    {
        fprintf(stderr, "ERROR: Could not open file \"%s\".", fileName);
        return 1;
    }

    printf("Contents of file:\n");
    while ((bytesRead = getline(&line, &length, file)) != GETLINE_FAILURE)
    {
        int calories = 0;
        int scanResult = sscanf(line, "%d", &calories);
        if (scanResult == EOF)
        {
            if (elfInventories[currentIndex] > biggestInventory)
            {
                indexOfBiggestInventory = currentIndex;
                biggestInventory = elfInventories[currentIndex];
            }
            currentIndex++;
            if (currentIndex > MAX_ELVES - 1)
            {
                fprintf(stderr, "Too many elves! Get Tristan to increase MAX_ELVES.\n");
                return 2;
            }
            printf("-- New elf\n");
        }
        else
        {
            elfInventories[currentIndex] += calories;
            printf("Calories: %d\n", calories);
        }
        free(line);
        line = NULL;
    }

    for (size_t i = 0; i <= currentIndex; i++)
    {
        printf("Elf %ld has %d calories in their pack.\n", i + 1, elfInventories[i]);
    }
    printf("\nThe elves should ask elf %ld. They have %d\n", indexOfBiggestInventory + 1, elfInventories[indexOfBiggestInventory]);
    return 0;
}
