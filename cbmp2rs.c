#include <stdio.h>  // Fuer fopen, fscanf, fclose, printf
#include <stdlib.h> // Fuer exit
#include <unistd.h>
#include <ctype.h>
#include <string.h>

#define  INPUT_FILE   "bmp_hhu.c"
#include INPUT_FILE



int main() {
    char buff[256];
    char output_fname[256];
    sprintf(buff,"%s",INPUT_FILE);

    char *fname = strtok(buff,".");
    sprintf(output_fname,"%s.rs", fname);

    printf("Converting GIMP c source image '%s' to Rust array file '%s' ...\n", INPUT_FILE, output_fname);

    FILE *f = fopen(output_fname, "w+");
    if (f==NULL) {
        printf("error: could not create output file\n");
        exit(EXIT_FAILURE);
    }

    fprintf(f, "pub const WIDTH:u32  = %d;\n", hhu.width);
    fprintf(f, "pub const HEIGHT:u32 = %d;\n", hhu.height);
    fprintf(f, "pub const BPP:u32    = %d;\n", hhu.bytes_per_pixel);
    fprintf(f, "\n");
    fprintf(f, "pub const DATA: &[u8;%ld] = b\"", sizeof(hhu.pixel_data));

    for (int i=0; i< sizeof(hhu.pixel_data); i++)
        fprintf(f, "\\x%02x", hhu.pixel_data[i]);

    fprintf(f, "\";\n");

    fclose(f);
    exit(EXIT_SUCCESS);
}
