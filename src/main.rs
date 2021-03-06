extern crate clap;
extern crate glium;

use clap::{App, Arg};

use glium::glutin;
use glium::Surface;
use std::fs::File;
use std::io::prelude::*;

mod cpu;
mod memory;

fn main() {
    let matches = App::new("RustBoy")
        .version("0.1.0")
        .author("Guilherme Chichanoski <guilherme.chichanoski@gmail.com>")
        .about("A GameBoy Emulator.")
        .arg(
            Arg::with_name("rom")
                .short("r")
                .long("rom")
                .help("ROM file to load")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    // It's safe to unwrap here because this argument is required.
    let rom_path = matches.value_of("rom").unwrap();

    // Abre o arquivo
    let mut rom_file = match File::open(rom_path) {
        Ok(file) => file,
        Err(err) => panic!("Deu bosta na leitura do arquivo: {}", err),
    };

    // Lê todo o arquivo e coloca na memoria (rom)
    let mut rom = Vec::new();
    let rom_size = match rom_file.read_to_end(&mut rom) {
        Ok(size) => size,
        Err(_) => 0,
    };

    let mem = memory::Memory::new(rom);
    let mut cpu = cpu::CPU::new(mem);
    loop {
        cpu.step();
    }

    // Instancia os objetos necessarios para tela
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_dimensions(240, 160)
        .with_title("GBA Emulator");
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let mut closed = false;

    // Muda a cor do fundo para azul
    let mut target = display.draw();
    target.clear_color(0.0, 0.0, 1., 1.);
    target.finish().unwrap();

    // Loop de execução do programa
    while !closed {
        events_loop.poll_events(|ev| match ev {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::Closed => closed = true,
                _ => (),
            },
            _ => (),
        });
    }
}
