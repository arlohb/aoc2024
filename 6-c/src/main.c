#include <stdio.h>
#include <stdlib.h>

int fileLen(FILE* fd) {
    fseek(fd, 0, SEEK_END);
    int size = ftell(fd);
    rewind(fd);

    return size;
}

#define OBJECT '#'
#define GUARD '^'
#define EMPTY '.'
#define VISITED 'X'

typedef enum {
    UP,
    DOWN,
    LEFT,
    RIGHT,
} Dir;

int dirOffsetX(Dir dir) {
    switch (dir) {
        case UP   : return 0;
        case DOWN : return 0;
        case LEFT : return -1;
        case RIGHT: return 1;
    }
}

int dirOffsetY(Dir dir) {
    switch (dir) {
        case UP   : return -1;
        case DOWN : return 1;
        case LEFT : return 0;
        case RIGHT: return 0;
    }
}

Dir dirTurnRight(Dir dir) {
    switch (dir) {
        case UP   : return RIGHT;
        case DOWN : return LEFT;
        case LEFT : return UP;
        case RIGHT: return DOWN;
    }
}

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

Map* parseInput(char* input, int len) {
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

typedef struct {
    Map* map;
    int guardX;
    int guardY;
    Dir dir;
    int count;
} Solver;

char solverGetPoint(Solver* solver) {
    return solver->map->data
        [solver->guardX]
        [solver->guardY];
}

void solverSetPoint(Solver* solver, char point) {
    solver->map->data
        [solver->guardX]
        [solver->guardY] = point;
}

void solverStep(Solver* solver) {
    solver->guardX += dirOffsetX(solver->dir);
    solver->guardY += dirOffsetY(solver->dir);
}

void solverUndoStep(Solver* solver) {
    solver->guardX -= dirOffsetX(solver->dir);
    solver->guardY -= dirOffsetY(solver->dir);
}

int solverIsInBounds(Solver* solver) {
    return solver->guardX >= 0
        && solver->guardX < solver->map->width
        && solver->guardY >= 0
        && solver->guardY < solver->map->height;
}

// Returns true if still in bounds
int solverWalk(Solver* solver) {
    while (solverGetPoint(solver) != OBJECT) {
        solverStep(solver);

        if (!solverIsInBounds(solver)) {
            return 0;
        }

        char point = solverGetPoint(solver);
        if (point == EMPTY) {
            solverSetPoint(solver, VISITED);
            solver->count++;
        }
    }

    solverUndoStep(solver);
    solver->dir = dirTurnRight(solver->dir);

    return 1;
}

int solve(Map* map) {
    Solver solver;
    solver.map = map;
    solver.dir = UP;
    // To include starting square
    solver.count = 1;
    solver.guardX = 0;
    solver.guardY = 0;

    mapFind(map, GUARD, &solver.guardX, &solver.guardY);

    while (solverWalk(&solver));

    mapPrint(solver.map);

    return solver.count;
}

int main() {
    FILE* inputFile = fopen("input.txt", "r");

    if (!inputFile) {
        printf("Failed to open input\n");
        return 1;
    }

    int len = fileLen(inputFile);

    char* input = malloc(sizeof(char) * len);
    if (fread(input, len, 1, inputFile) != 1) {
        printf("Failed to read input\n");
        return 1;
    }

    fclose(inputFile);

    Map* map = parseInput(input, len);

    int count = solve(map);
    printf("%d\n", count);

    mapFree(map);
    free(map);
}

