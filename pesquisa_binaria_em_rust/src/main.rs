fn pesquisa_binaria(lista: &[i32], item: i32) -> Option<usize> {
    let mut baixo = 0;
    let mut alto = lista.len() - 1;

    while baixo <= alto {
        let meio = (baixo + alto) / 2;
        let chute = lista[meio];

        if chute == item {
            return Some(meio);
        }
        if chute > item {
            alto = meio - 1;
        } else {
            baixo = meio + 1;
        }
    }
    return None;
}

fn main() {
    let lista = [1, 3, 5, 7, 9];

    println!("{:?}", pesquisa_binaria(&lista, 5)); // Saída: Some(1)
                                                   // println!("{:?}", pesquisa_binaria(&lista, -1)); // Saída: None
}
