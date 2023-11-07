```
#include <stdio.h>

int main() {
  FILE *fp;
  
  if(!(fp = fopen("README.md", "w"))) {
    printf("Woops - no README today");
    return 1;
  }

  fprintf(fp, "Hello, My name is %s", "Theo Lincke");
  fclose(fp);

  return 0;
}
```
