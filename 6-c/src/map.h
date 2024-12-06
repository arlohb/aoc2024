#ifndef MAP_H
#define MAP_H

#include <stdlib.h>
#include <stdio.h>

typedef struct {
    char** data;
    int width;
    int height;
} Map;

void mapInit(Map* map, int width, int height) {
    map->width = width;
    map->height = height;

    map->data = malloc(sizeof(char*) * width);

    for (int x = 0; x < width; x++) {
        map->data[x] = malloc(sizeof(char) * height);
    }
}

void mapFree(Map* map) {
    for (int x = 0; x < map->width; x++) {
        free(map->data[x]);
    }

    free(map->data);
}

void mapPrint(Map* map) {
    for (int y = 0; y < map->height; y++) {
        for (int x = 0; x < map->width; x++) {
            char c = map->data[x][y];
            printf("%c ", c);
        }
        printf("\n");
    }
}

void mapFind(Map* map, char toFind, int* outX, int* outY) {
    for (int x = 0; x < map->width; x++) {
        for (int y = 0; y < map->height; y++) {
            char c = map->data[x][y];
            if (c == toFind) {
                *outX = x;
                *outY = y;
                return;
            }
        }
    }
}

Map* mapParse(char* input, int len) {
    int lineLen = 0;
    while (input[lineLen++] != '\n');
    // Remove the \n
    lineLen--;

    int lineNum = len / lineLen;

    Map* map = malloc(sizeof(Map));
    mapInit(map, lineLen, lineNum);

    for (int lineIndex = 0; lineIndex < lineNum; lineIndex++) {
        int lineOffset = lineIndex * (lineLen + 1);
        char* line = input + lineOffset;
        for (int x = 0; x < lineLen; x++) {
            map->data[x][lineIndex] = line[x];
        }
    }

    mapPrint(map);

    return map;
}

#endif
