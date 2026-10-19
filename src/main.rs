#![no_std]
#![no_main]

mod memoria;
mod vga_buffer;

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("so1_kernel UMG - Asignacion de memoria");

    let mut m = memoria::Memoria::new();

    println!("\n1. MEMORIA INICIAL");
    m.mostrar();

    println!("\n2. RESERVA DE MEMORIA");

    let a = m.reservar(b'A', 40).unwrap();
    let b = m.reservar(b'B', 100).unwrap();
    let c = m.reservar(b'C', 20).unwrap();

    m.mostrar();

    println!("\n3. ESCRITURA Y LECTURA");

    m.escribir(a, 0, b"Hola UMG").unwrap();

    match m.leer(a, 0, 8) {
        Ok(datos) => {
            println!(
                "A contiene: {}",
                core::str::from_utf8(datos).unwrap()
            );
        }
        Err(e) => println!("Error de lectura: {}", e),
    }

    println!("\n4. PROTECCION DE MEMORIA");

    match m.escribir(c, 30, b"desborde!") {
        Ok(_) => println!("Escritura aceptada"),
        Err(e) => println!("Escritura rechazada: {}", e),
    }

    match m.leer(a, 0, 49) {
        Ok(_) => println!("Lectura aceptada"),
        Err(e) => println!("Lectura rechazada: {}", e),
    }

    println!("\n5. LIBERACION Y PRIMER AJUSTE");

    m.liberar(b);
    m.mostrar();

    let _d = m.reservar(b'D', 50);
    let _e = m.reservar(b'E', 700);

    m.mostrar();

    println!("\n6. FRAGMENTACION EXTERNA");

    let _f = m.reservar(b'F', 150);

    println!("\n7. RETO B: MEJOR AJUSTE");

    let mut mejor = memoria::Memoria::new();

    let _a = mejor.reservar(b'A', 40).unwrap();
    let bb = mejor.reservar(b'B', 100).unwrap();
    let _c = mejor.reservar(b'C', 20).unwrap();

    mejor.liberar(bb);

    let _d = mejor.reservar_mejor_ajuste(b'D', 50);

    mejor.mostrar();

    println!("\n8. RETO C: COMPACTACION");

    m.compactar();

    let _f2 = m.reservar(b'F', 150);

    m.mostrar();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("PANIC: {}", info);
    loop {}
}
