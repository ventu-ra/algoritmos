def pesquisa_binaria(lista:, item:)
  baixo = 0

  alto = lista.size

  while baixo <= alto
    meio = (baixo + alto) / 2
    chute = lista[meio]

    return meio if chute == item

    if chute > item
      alto = meio - 1
    else
      baixo = meio + 1
    end
  end
  nil
end

lista = [1, 3, 5, 7, 9]
puts pesquisa_binaria(lista: lista, item: 3)
puts pesquisa_binaria(lista: lista, item: -1)
