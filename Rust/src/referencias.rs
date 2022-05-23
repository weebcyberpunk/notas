fn main() {

    /*
     * Referências são como ponteiros que tem a garantia de sempre apontarem para algo válido.
     * Usá-los impede que váriaveis tenham de necessariamente ser retornadas de uma função para que
     * ela devolva a posse.
     *
     * Assim como ponteiros de C, a referência é feita com & e a dereferência é feita com *.
     *
     * Como referências não são donas dos valores a qual elas referenciam, quando elas saem de
     * escôpo os valores não são dropados.
     *
     * Assim como as próprias variáveis, as referências são imutáveis por padrão. Não podemos
     * simplesmente modificar algo que emprestamos. Para modificá-las, deve-se assinalar a
     * váriavel, a referência criada na função chamadora e o argumento da função chamada para serem
     * mutáveis.
     *
     * Só é possível ter uma referência mutável de algo por vez. Imutáveis, quantas quiser, pois
     * elas são read-only. Porém, não se pode ter uma referência mutável e uma imutável ao mesmo
     * tempo, pois quem usa uma referência para leitura não espera que o valor dela mude de
     * repente.
     *
     * O escôpo de referências imutáveis existe desde que elas são criadas até o seu último uso,
     * de maneira que é possível ter referências mutáveis e imutáveis dentro de uma mesma função,
     * por exemplo, caso a referência mutável só seja criada após o último uso da imutável.
     */

}
