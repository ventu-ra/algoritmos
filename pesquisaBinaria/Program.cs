
static int pesquisaBinaria(int[] lista, int item)
{
  int baixo = 0;

  int alto = lista.Length - 1;

  while (baixo <= alto)
  {
    int meio = (baixo + alto) / 2;
    int chute = lista[meio];

    if (chute == item) return meio;

    if (chute > item) alto = meio - 1;

    else baixo = meio + 1;

  }

  return -1;
}

int[] lista = [1, 3, 5, 7, 9];

Console.WriteLine(pesquisaBinaria(lista, 3));
Console.WriteLine(pesquisaBinaria(lista, -1));
