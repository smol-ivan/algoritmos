use rand::prelude::*;
use std::env;
use std::time::Instant;

use csv::Writer;
use std::error::Error;
use std::fs::create_dir_all;

use serde::Serialize;

#[derive(Serialize)]
struct FilaTiempo {
    ejec: usize,
    is_t: f64,
    ss_t: f64,
    bs_t: f64,
}

fn guardar_csv_individual(n: usize, datos: &[FilaTiempo]) -> Result<(), Box<dyn Error>> {
    create_dir_all("resultados")?; // crea carpeta si no existe
    let path = format!("resultados/resultados_n_{}.csv", n);
    let mut wtr = Writer::from_path(path)?;

    // wtr.write_record(&["ejec", "is_t", "ss_t", "bs_t"])?;

    for fila in datos {
        wtr.serialize(fila)?;
    }

    wtr.flush()?;
    Ok(())
}

fn insertion_sort(array: &mut Vec<usize>, n: usize) {
    for j in 1..n {
        let k = array[j];
        let mut i = j - 1;

        while i > 0 && array[i - 1] > k {
            array[i+1] = array[i];
            i -= 1;
        }
        array[i + 1] = k
    }
}

fn intercambiar(array: &mut Vec<usize>, i: usize, j: usize) {
    array.swap(i, j);
}

fn selection_sort(array: &mut Vec<usize>, n: usize) {
    for j in 0..n - 1 {
        let mut menor = j;
        for i in j + 1..n {
            if array[i] < array[menor] {
                menor = i;
            }
        }
        if menor != j {
            intercambiar(array, j, menor);
        }
    }
}

fn bubble_sort(array: &mut Vec<usize>, n: usize) {
    for i in 0..n {
        for j in (i + 1..n).rev() {
            if array[j] < array[j - 1] {
                intercambiar(array, j, j - 1);
            }
        }
    }
}

fn generar_arreglo(n: usize) -> Vec<usize> {
    let mut array: Vec<usize> = Vec::with_capacity(n);
    let mut rng = rand::rng();
    for _ in 0..n {
        let random_number: usize = rng.random_range(1..=10 * n);
        array.push(random_number);
    }
    array
}

fn tiempo_ejecucion(n: usize, m: usize) {
    let mut datos: Vec<FilaTiempo> = Vec::with_capacity(m);

    for i in 1..=m {
        let arreglo: Vec<usize> = generar_arreglo(n);

        let mut copy_is = arreglo.clone();
        let mut copy_ss = arreglo.clone();
        let mut copy_bs = arreglo.clone();

        let inicio_is = Instant::now();
        insertion_sort(&mut copy_is, n);
        let duracion_is = inicio_is.elapsed();

        let inicio_ss = Instant::now();
        selection_sort(&mut copy_ss, n);
        let duracion_ss = inicio_ss.elapsed();

        let inicio_bs = Instant::now();
        bubble_sort(&mut copy_bs, n);
        let duracion_bs = inicio_bs.elapsed();

        datos.push(FilaTiempo {
            ejec: i,
            is_t: duracion_is.as_secs_f64(),
            ss_t: duracion_ss.as_secs_f64(),
            bs_t: duracion_bs.as_secs_f64(),
        });
    }
    if let Err(e) = guardar_csv_individual(n, &datos) {
        eprintln!("Error al guardar archivo csv: {}", e);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Uso: $ ./{} <array size> <iterations>", args[0]);
        return;
    }

    let _n: usize = args[1].parse().expect("Se espera un numero");
    let m: usize = args[2].parse().expect("Se espera un numero");

    let n_vec: Vec<usize> = vec![
        10, 50, 100, 500, 1_000, 5_000, 10_000, 20_000, 30_000, 40_000, 50_000, 60_000, 70_000,
        80_000, 90_000, 100_000,
    ];

    for n in n_vec.iter() {
        println!("Prueba: n -> {}", &n);
        tiempo_ejecucion(*n, m);
    }

    // tiempo_ejecucion(n, m);
}
