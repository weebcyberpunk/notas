package notas;

public class JVMeJDK {
    public static void main(String[] args) {
        /* Java - JVM
         * 
         * A Java Virtual Machine pega o bytecode compilado pela JavaC e faz ele funcionar
         *
         * Ela é separada em algumas áreas:
         *    
         * - Loader/verificador: carrega o bytecode na memória da JVM e verifica se o código pode ser executado
         * - Interpretador/gerenciador: transforma o bytecode em código de máquina para cada máquina. O gerenciador trata sobre como a memória será gerenciada na JVM
         * - Compilador JIT: compilador Just In Time, compila o código em tempo real, para diminuir perdas de desempenho causadas pelo fato do programa precisar ser compilado antes de ser executado
         * 
         * 
         * Java - JDK
         *
         * O Java Development Kit é o kit para desenvolvimento em Java, que possui um JRE, JavaLang e JavaTools
         * IDEs podem ser anexadas ao JDK, mas não vem juntas dele
         *
         * JavaTools:
         *
         * JavaC: compilador Java, transforma Java em bytecode
         * Debugger: verifique a execução do programa em tempo real
         * APIs: apis ué kk
         *
         *
         * Edições do Java
         *
         * Existem algumas versões do JDK. Resumindo, 3:
         *
         * SE (Standard Edition): edição padrão
         * EE (Enterprise Edition): edição com acesso a banco de dados grandes e outras coisas que gigaempresas fazem
         * ME (Micro Edition): coisas pequenas como aplicativos de telefone, wearables, controles de arduino, etc
         */
    }    
}
