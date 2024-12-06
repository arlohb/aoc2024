#include <stdio.h>
#include <stdlib.h>

#include "map.h"
#include "solver.h"

int fileLen(FILE* fd) {
    fseek(fd, 0, SEEK_END);
    int size = ftell(fd);
    rewind(fd);

    return size;
}

char* readFile(char* path, int* len) {
    FILE* file = fopen(path, "r");
    if (!file) return 0;

    *len = fileLen(file);

    char* data = malloc(sizeof(char) * *len);
    if (fread(data, *len, 1, file) != 1) return 0;

    fclose(file);

    return data;
}

int main() {
    int len = 0;
    char* input = readFile("input.txt", &len);

    if (!input) {
        printf("Failed to read input\n");
        return 1;
    }

    Map* map = mapParse(input, len);
    free(input);

    int count = solve(map);
    printf("%d\n", count);

    mapFree(map);
    free(map);
}

