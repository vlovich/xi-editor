#include <stdbool.h>
#include <stdio.h>

#if defined(_MSC_VER)
#define API __declspec(dllexport)
#elif defined(__GNUC__)
#define API __attribute__((visibility("default")))
#endif

struct xi_trace_block_t;
typedef struct xi_trace_block_t xi_trace_block_t;

extern bool xi_trace_is_enabled(void);
extern void xi_trace(const char *name, const char ** categories);
extern xi_trace_block_t* xi_trace_block_begin(const char *name, const char ** categories);
extern void xi_trace_block_end(xi_trace_block_t* trace_block);

API void example_main(void);

void something() {
}

void something_else() {
}

API void example_main(void) {
    fprintf(stderr, "C: trace enabled = %s\n", xi_trace_is_enabled() ? "yes" : "no");

    xi_trace_block_t* total_trace = xi_trace_block_begin("total", (const char*[]) {"c", "frontend"});
    xi_trace_block_t* trace = xi_trace_block_begin("something", (const char*[]) {"c", "frontend"});
    something();
    xi_trace_block_end(trace);
    xi_trace("something_else", (const char*[]) {"c", "frontend"});
    something_else();
    xi_trace_block_end(total_trace);
}
