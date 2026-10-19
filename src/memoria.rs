use crate::println;

pub const TAM_BLOQUE: usize = 16;
pub const NUM_BLOQUES: usize = 64;

pub struct Memoria {
    datos: [u8; TAM_BLOQUE * NUM_BLOQUES],
    dueno: [u8; NUM_BLOQUES],
    largo: [usize; NUM_BLOQUES],
}

impl Memoria {
    pub fn new() -> Self {
        Self {
            datos: [0; TAM_BLOQUE * NUM_BLOQUES],
            dueno: [b'.'; NUM_BLOQUES],
            largo: [0; NUM_BLOQUES],
        }
    }

    pub fn reservar(&mut self, nombre: u8, bytes: usize) -> Option<usize> {
        if bytes == 0 {
            return None;
        }

        let n = (bytes + TAM_BLOQUE - 1) / TAM_BLOQUE;
        let mut inicio = 0;

        while inicio + n <= NUM_BLOQUES {
            if self.dueno[inicio..inicio + n]
                .iter()
                .all(|&d| d == b'.')
            {
                for i in inicio..inicio + n {
                    self.dueno[i] = nombre;
                }

                self.largo[inicio] = n;

                println!(
                    "{} pide {} bytes -> bloques {}..{} (desperdicio interno: {} bytes)",
                    nombre as char,
                    bytes,
                    inicio,
                    inicio + n - 1,
                    n * TAM_BLOQUE - bytes
                );

                return Some(inicio);
            }

            inicio += 1;
        }

        println!(
            "{} pide {} bytes -> NO HAY ESPACIO CONTIGUO",
            nombre as char,
            bytes
        );

        None
    }

    pub fn liberar(&mut self, inicio: usize) {
        if inicio >= NUM_BLOQUES {
            return;
        }

        let n = self.largo[inicio];

        if n == 0 || inicio + n > NUM_BLOQUES {
            return;
        }

        let nombre = self.dueno[inicio];

        for i in inicio..inicio + n {
            self.dueno[i] = b'.';
        }

        self.largo[inicio] = 0;

        println!("{} libera {} bloques", nombre as char, n);
    }

    pub fn escribir(
        &mut self,
        inicio: usize,
        desp: usize,
        datos: &[u8],
    ) -> Result<(), &'static str> {
        if inicio >= NUM_BLOQUES {
            return Err("inicio fuera de limites");
        }

        let bloques = self.largo[inicio];

        if bloques == 0 {
            return Err("region no asignada");
        }

        let limite = bloques * TAM_BLOQUE;

        if desp > limite || datos.len() > limite - desp {
            return Err("fuera de limites");
        }

        let base = inicio * TAM_BLOQUE + desp;

        self.datos[base..base + datos.len()].copy_from_slice(datos);

        Ok(())
    }

    pub fn leer(
        &self,
        inicio: usize,
        desp: usize,
        len: usize,
    ) -> Result<&[u8], &'static str> {
        if inicio >= NUM_BLOQUES {
            return Err("inicio fuera de limites");
        }

        let bloques = self.largo[inicio];

        if bloques == 0 {
            return Err("region no asignada");
        }

        let limite = bloques * TAM_BLOQUE;

        if desp > limite || len > limite - desp {
            return Err("fuera de limites");
        }

        let base = inicio * TAM_BLOQUE + desp;

        Ok(&self.datos[base..base + len])
    }

    pub fn mostrar(&self) {
        let mut libres = 0;
        let mut hueco = 0;
        let mut mayor = 0;

        for &d in self.dueno.iter() {
            if d == b'.' {
                libres += 1;
                hueco += 1;

                if hueco > mayor {
                    mayor = hueco;
                }
            } else {
                hueco = 0;
            }
        }

        println!("[{}]", core::str::from_utf8(&self.dueno).unwrap());

        println!(
            "  libres: {} bloques ({} bytes), hueco mayor: {} bloques",
            libres,
            libres * TAM_BLOQUE,
            mayor
        );
    }

    pub fn compactar(&mut self) {
        let mut nuevos_datos = [0u8; TAM_BLOQUE * NUM_BLOQUES];
        let mut nuevos_duenos = [b'.'; NUM_BLOQUES];
        let mut nuevos_largos = [0usize; NUM_BLOQUES];
        let mut destino = 0;
        let mut origen = 0;

        while origen < NUM_BLOQUES {
            let n = self.largo[origen];

            if n > 0 {
                let bytes = n * TAM_BLOQUE;
                let base_origen = origen * TAM_BLOQUE;
                let base_destino = destino * TAM_BLOQUE;

                nuevos_datos[base_destino..base_destino + bytes]
                    .copy_from_slice(
                        &self.datos[base_origen..base_origen + bytes],
                    );

                for i in destino..destino + n {
                    nuevos_duenos[i] = self.dueno[origen];
                }

                nuevos_largos[destino] = n;

                println!(
                    "Proceso {}: bloques {}..{} -> {}..{}",
                    self.dueno[origen] as char,
                    origen,
                    origen + n - 1,
                    destino,
                    destino + n - 1
                );

                destino += n;
                origen += n;
            } else {
                origen += 1;
            }
        }

        self.datos = nuevos_datos;
        self.dueno = nuevos_duenos;
        self.largo = nuevos_largos;

        println!("Memoria compactada");
        self.mostrar();
    }

    pub fn reservar_mejor_ajuste(
        &mut self,
        nombre: u8,
        bytes: usize,
    ) -> Option<usize> {
        if bytes == 0 {
            return None;
        }

        let n = (bytes + TAM_BLOQUE - 1) / TAM_BLOQUE;
        let mut inicio = 0;
        let mut mejor_inicio = None;
        let mut mejor_tamano = NUM_BLOQUES + 1;

        while inicio < NUM_BLOQUES {
            if self.dueno[inicio] != b'.' {
                inicio += 1;
                continue;
            }

            let comienzo = inicio;

            while inicio < NUM_BLOQUES && self.dueno[inicio] == b'.' {
                inicio += 1;
            }

            let tamano = inicio - comienzo;

            if tamano >= n && tamano < mejor_tamano {
                mejor_inicio = Some(comienzo);
                mejor_tamano = tamano;
            }
        }

        if let Some(pos) = mejor_inicio {
            for i in pos..pos + n {
                self.dueno[i] = nombre;
            }

            self.largo[pos] = n;

            println!(
                "{} pide {} bytes -> mejor ajuste, bloques {}..{} (desperdicio interno: {} bytes)",
                nombre as char,
                bytes,
                pos,
                pos + n - 1,
                n * TAM_BLOQUE - bytes
            );

            Some(pos)
        } else {
            println!(
                "{} pide {} bytes -> NO HAY ESPACIO CONTIGUO",
                nombre as char,
                bytes
            );

            None
        }
    }
}
