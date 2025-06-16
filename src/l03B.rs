use std::env;
use std::time::Instant;

use csv::Writer;
use std::error::Error;
use std::fs::create_dir_all;

use serde::Serialize;

#[derive(Serialize)]
struct Fila {
    ejec: usize,
    pd_t: f64,
    pf_t: f64,
}

fn guardar_csv_individual(n: usize, datos: &[Fila]) -> Result<(), Box<dyn Error>> {
    create_dir_all("polinomios")?;
    let path = format!("polinomios/orden_n_{}.csv", n);
    let mut wtr = Writer::from_path(path)?;

    for fila in datos {
        wtr.serialize(fila)?;
    }

    wtr.flush()?;
    Ok(())
}

type Long = u128;

fn evaluar_polinomio(a: &Vec<usize>, n: usize, x: usize) -> Option<Long> {
    let mut px: Long = 0;
    for i in 0..=n {
        let mut potencia: Long = 1;
        for _ in 1..=i {
            potencia *= x as Long;
        }
        px += a[i] as Long * potencia;
    }
    Some(px)
}

fn evaluar_polinomio_factorizado(a: &Vec<usize>, n: usize, x: usize) -> Option<Long> {
    let mut resultado = a.last().copied().unwrap() as Long;
    for i in (0..n).rev() {
        resultado = resultado * x as Long + a[i] as Long;
    }
    Some(resultado)
}

fn generar_coeficientes(n: usize) -> Vec<usize> {
    let mut vec = Vec::with_capacity(n + 1);
    // let mut rng = rand::rng();
    for _ in 0..=n {
        let random_number = 1;
        vec.push(random_number as usize);
    }
    vec
}

fn control(orden_n: usize, m: usize) {
    let mut datos: Vec<Fila> = Vec::with_capacity(m);

    let variable_x = 1;

    println!(
        "Evaluando polinomio de orden n = {} con x = {}",
        orden_n, variable_x
    );
    for i in 1..=m {
        // Crear arreglo
        let coeficientes = generar_coeficientes(orden_n);

        let inicio_directa = Instant::now();
        evaluar_polinomio(&coeficientes, orden_n, variable_x);
        let duracion_directa = inicio_directa.elapsed();

        let inicio_factorizado = Instant::now();
        evaluar_polinomio_factorizado(&coeficientes, orden_n, variable_x);
        let duracion_factorizado = inicio_factorizado.elapsed();

        datos.push(Fila {
            ejec: i,
            pd_t: duracion_directa.as_secs_f64(),
            pf_t: duracion_factorizado.as_secs_f64(),
        });
    }

    if let Err(e) = guardar_csv_individual(orden_n, &datos) {
        eprintln!("error al guardar el csv: {}", e);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Uso: $ ./{} <orden_n> <repeticiones>", args[0]);
        return;
    }
    let _orden_n: usize = args[1].parse().expect("Numero invalido para n");
    let m: usize = args[2].parse().expect("Numero invalido para x");

    let n_vec: Vec<usize> = vec![
        10, 50, 100, 500, 1_000, 5_000, 10_000, 20_000, 30_000, 40_000, 50_000, 60_000, 70_000,
        80_000, 90_000, 100_000, 200_000, 300_000, 400_000, 500_000, 600_000, 700_000, 800_000, 900_000, 1_000_000,
    ];

    for &n in n_vec.iter() {
        control(n, m);
    }
}
