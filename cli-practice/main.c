#include <stdio.h>
#include <string.h>

int main(int argc, char **argv)
{
    /*
    printf("Hello: %d\n", argc);
    for (int i = 0; i < argc; ++i)
    {
        printf("Arg[%d] = %s\n", i, argv[i]);
    }
    */
    if (argc != 2)
    {
        puts("Error: Expected two arguments");
    }
    if (strcmp(argv[1], "--help") == 0)
    {
        puts("Help: main --help");
    }
    else
    {
        puts("Unknown command");
    }
    return 0;
}
