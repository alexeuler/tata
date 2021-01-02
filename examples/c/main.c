#include <stdio.h>
#include <unistd.h>
#include <stdlib.h>
#include <pthread.h>
#include "tata_core.h"

void callback(ByteArray bytes)
{
    for (int i = 0; i < bytes.len; i++)
    {
        printf("%c", bytes.data[i]);
    }
    printf("\n");
    fflush(stdout);
}

int main(int argc, char const *argv[])
{
    KeyPair key_pair = generate_keypair();
    unsigned char *p1 = malloc(4 * sizeof(char));
    p1[0] = 'T';
    p1[1] = 'a';
    p1[2] = 't';
    p1[3] = 'a';
    ByteArray name = {
        .data = p1,
        .len = 4};
    start_network(key_pair.secret, name, callback, true, Debug);
    pause();

    return 0;
}
