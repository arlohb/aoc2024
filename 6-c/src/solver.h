#ifndef SOLVER_H
#define SOLVER_H

#include "dir.h"
#include "map.h"

#define OBJECT '#'
#define GUARD '^'
#define EMPTY '.'
#define VISITED 'X'

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

#endif
