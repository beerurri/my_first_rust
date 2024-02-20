use std::time::{Duration, Instant};
use std::path::Path;
use std::fs;
use plotters::prelude::*;

const Nx: u64 = 10;
const Nt: u64 = 1e7 as u64;

fn main() {
    println!("Hello, world!");

    let L = 0.01;
    let T = 0.01;

    let k: f64 = 50.;
    let T_room = 293.15;
    let T_melt = 1535. + T_room;

    let dx = L / Nx as f64;
    let dx2 = dx.powi(2);
    let dt = T as f64 / Nt as f64;
    let kdt = k * dt;
    println!("dx = {}, dt = {}", dx, dt);

    if (dt) > (dx2 / (2. * k)) {
        panic!("dt > dx**2 / (2 * k), {} > {}", dt, dx2 / (2. * k));
    } else {
        println!("dt <= dx**2 / (2 * k), {} <= {}", dt, dx2 / (2. * k))
    }

    let mut time = Instant::now();

    let mut mesh = vec![[0 as f64; Nx as usize]; Nt as usize];

    // начальные условия
    for i in 0..mesh[0].len() {
        mesh[0][i] = T_room;
    }

    // граничные условия
    for i in 0..mesh.len() {
        let l = mesh[i].len();
        mesh[i][l-1] = T_room;
        mesh[i][0] = T_melt / 2.;
    }

    // распечатать сетку
    println!("\ninitial mesh:");
    print_array(mesh.clone());

    // посчитать сетку
    for n in 0..(mesh.len() - 1) { // n for time
        for i in 1..(mesh[0].len() - 1) { // i for coordinate
            mesh[n+1][i] = mesh[n][i] + kdt * ((mesh[n][i+1] - 2 as f64 * mesh[n][i] + mesh[n][i-1]) / dx2);
        }
    }

    // распечатать обновлённую сетку
    println!("\nupdated mesh:");
    print_array(mesh.clone());

    println!("elapsed: {}", time.elapsed().as_secs_f32());

    time = Instant::now();

    let to_dump = &mesh[0..1000000];
    dump_csv(to_dump);

    println!("dumping elapsed: {}", time.elapsed().as_secs_f32());
}

fn dump_csv(mesh: &[[f64; Nx as usize]]) {
    let mut str = String::new();

    for i in 0..mesh.len() {
        for j in 0..mesh[i].len() {
            str += &mesh[i][j].to_string();
            if j != mesh[i].len() - 1 {
                str += ",";
            }
        }
        str += "\n";
    }

    write("dump.csv", str);
}

fn write<P>(filename: P, text: String)
where P: AsRef<Path>, {
    fs::write(filename, text).expect("Unable to write file");
}

fn print_array(mesh: Vec<[f64; Nx as usize]>) {
    for i in 0..3 {
        let l = mesh[i].len();
        println!("{} {} {} ... {} {} {}", mesh[i][0], mesh[i][1], mesh[i][2], mesh[i][l-3], mesh[i][l-2], mesh[i][l-1]);
    }
    println!("...");
    for i in (mesh.len()-3)..mesh.len() {
        let l = mesh[i].len();
        println!("{} {} {} ... {} {} {}", mesh[i][0], mesh[i][1], mesh[i][2], mesh[i][l-3], mesh[i][l-2], mesh[i][l-1]);
    }
}