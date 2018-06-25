# Aprenent Rust

Aquest és el meu segon programa en Rust. Fins ara no n'havia fet mai cap i per tant segurament hi faig algunes coses que poden ser fetes d'una forma més òptima.

## Compilar

1.  Primer s'ha d'instal·lar Rust. La forma més senzilla sol ser seguir les instruccions de [la seva web](https://www.rust-lang.org/es-ES/) ;-). també estan en castellà però en resum consisteixen en executar:

    curl https://sh.rustup.rs -sSf | sh

2.  clonar el repositori

3.  compilar i executar el programa (per defecte agafa in.txt com a paràmetre)

    cargo run --release

Un cop compilat l'executable es trobarà disponible a **target/release/** i es pot executar sense fer servir cargo.

A més se li pot passar un fitxer amb les dades a provar com a paràmetre:

    target/release/rebenton in.txt

# Exercici: Montando semáforos

El programa és un dels exercicis del ProgramaMe de l'any 2018, Programa de la Nacional del año 2018, fet a la Universidad Complutense de Madrid.

> Tiempo máximo: 1,000 s Memoria máxima: 4096 KiB

## Semáforo con los colores descolocados

Shan Cho Kao es un empresario chino famoso por sus fábricas de semáforos. Todas ellas tienen una línea de producción que comienza con un operario que va colocando por orden las bombillas de los tres famosos colores: rojo, amarillo, verde; rojo, amarillo, verde… Más adelante, otros trabajadores van cogiendo de la línea las bombillas, y las ensamblan en los semáforos, que acabarán en las ciudades de medio mundo.

Todas las fábricas han funcionado perfectamente durante años, hasta que hace unos días, en la sede de Lan Li Hao cometieron el error de contratar a Confun Dio para colocar las luces al principio de la línea, sin que éste se atreviera a reconocer que es daltónico y no distingue los colores.

El resultado es que las luces están llegando a los ensambladores completamente desordenadas. Éstos, entrenados durante años a coger las bombillas consecutivas, no son capaces de reaccionar y adaptarse, por lo que se montan unos atascos en la línea de producción que no dicen nada positivo sobre el producto que fabrican.

En ocasiones, el azar hace que las bombillas puedan retirarse correctamente aprovechando que la retirada de tres luces hace que otras dos queden consecutivas aunque no lo estuvieran en un principio. Por ejemplo si Confun Dio coloca las luces en la secuencia rojo, amarillo, rojo, amarillo, verde, verde, se pueden construir dos semáforos extrayendo las luces de la línea de producción. Si representamos los colores por sus iniciales:

    RARAVV ⇒ RAV ⇒ ∅

Los operarios cogen siempre luces consecutivas, que deben estar en el orden rojo, amarillo, verde. Sabiendo esto, dada una secuencia de luces, ¿cuántos semáforos como máximo se pueden construir?

## Entrada

La entrada consiste en múltiples líneas, cada una de ellas con la secuencia de colores de las bombillas tal y como las ha colocado Confun Dio en un día de trabajo. Siempre se especifican como una cadena con las letras R, A o V, para cada uno de los colores rojo, amarillo y verde. El número máximo de bombillas por día que es capaz de poner es 500.000.

## Salida

Por cada caso de prueba el programa escribirá una única línea con el número máximo de semáforos completos y correctos que pueden construirse con los operarios actuales.

## Entrada de ejemplo

    RARAVV
    VAR
    RAVV

## Salida de ejemplo

    2
    0
    1
