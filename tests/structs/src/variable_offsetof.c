#include <stddef.h>

typedef struct {
    int a;
    float mod[3];
} use;

size_t get_offset(size_t idx) {
    return offsetof(use, mod[idx]);
}

struct yield {
    int a;
    float mod[3];
};

size_t get_offset2(size_t idx) {
    return offsetof(struct yield, mod[idx]);
}

struct Nested_D{
    int e;
} ;

struct Nested_C{
    struct Nested_D d[6];
} ;

typedef struct {
    struct Nested_C c;
} Nested_B;

typedef struct {
    Nested_B b[3];
} Nested_A;

typedef struct {
    Nested_A a;
} Outer;

size_t get_offset_nested(size_t idx) {
    return offsetof(Outer, a.b[idx].c.d[idx * 2].e);
}
