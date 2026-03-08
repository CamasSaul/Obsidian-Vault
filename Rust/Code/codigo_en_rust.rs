use std::fmt;

// --- Configuraciones y Tipos ---

#[derive(Debug, Clone, Copy, PartialEq)]
enum Entidad {
    Planta,
    Herbivoro { energia: u32 },
    Vacio,
}

struct Mundo {
    ancho: usize,
    alto: usize,
    celdas: Vec<Entidad>,
}

impl Mundo {
    /// Crea un nuevo mundo lleno de espacios vacíos
    fn nuevo(ancho: usize, alto: usize) -> Self {
        let celdas = vec![Entidad::Vacio; ancho * alto];
        Mundo { ancho, alto, celdas }
    }

    /// Convierte coordenadas (x, y) en un índice del vector
    fn obtener_indice(&self, x: usize, y: usize) -> usize {
        y * self.ancho + x
    }

    /// Coloca una entidad en una posición específica
    fn spawn(&mut self, x: usize, y: usize, ent: Entidad) {
        let idx = self.obtener_indice(x, y);
        self.celdas[idx] = ent;
    }

    /// La "lógica" del ecosistema: un turno de simulación
    fn actualizar(&mut self) {
        let mut nuevas_celdas = self.celdas.clone();

        for y in 0..self.alto {
            for x in 0..self.ancho {
                let idx = self.obtener_indice(x, y);

                match self.celdas[idx] {
                    Entidad::Herbivoro { energia } => {
                        // Si no tiene energía, muere (se convierte en Vacío)
                        if energia == 0 {
                            nuevas_celdas[idx] = Entidad::Vacio;
                        } else {
                            // Pierde energía por existir y se queda quieto (por ahora)
                            nuevas_celdas[idx] = Entidad::Herbivoro { energia: energia - 1 };
                        }
                    }
                    Entidad::Planta => {
                        // Las plantas intentan expandirse a la derecha si está vacío
                        if x + 1 < self.ancho {
                            let der = self.obtener_indice(x + 1, y);
                            if self.celdas[der] == Entidad::Vacio {
                                nuevas_celdas[der] = Entidad::Planta;
                            }
                        }
                    }
                    Entidad::Vacio => {}
                }
            }
        }
        self.celdas = nuevas_celdas;
    }
}

// Implementación de Display para imprimir el mundo en la consola de forma bonita
impl fmt::Display for Mundo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.alto {
            for x in 0..self.ancho {
                let simbolo = match self.celdas[self.obtener_indice(x, y)] {
                    Entidad::Planta => "🌿",
                    Entidad::Herbivoro { .. } => "🐰",
                    Entidad::Vacio => "░░",
                };
                write!(f, "{}", simbolo)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

// --- Función Principal ---

fn main() {
    let mut mi_mundo = Mundo::nuevo(10, 5);

    // Poblar el mundo inicialmente
    mi_mundo.spawn(0, 0, Entidad::Planta);
    mi_mundo.spawn(2, 2, Entidad::Herbivoro { energia: 5 });
    mi_mundo.spawn(5, 4, Entidad::Planta);

    println!("--- Inicio del Ecosistema ---");
    println!("{}", mi_mundo);

    // Simular 3 turnos
    for i in 1..=3 {
        mi_mundo.actualizar();
        println!("--- Turno {} ---", i);
        println!("{}", mi_mundo);
    }
}