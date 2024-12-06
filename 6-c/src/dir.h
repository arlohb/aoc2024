#ifndef DIR_H
#define DIR_H

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

#endif
