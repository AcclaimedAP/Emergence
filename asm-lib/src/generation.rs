use bevy::prelude::*;
use itertools::Itertools as _;
use rand::prelude::{IteratorRandom, SliceRandom};
use std::convert::TryInto;

use crate::graphics::make_sprite_components;
use crate::structures::{Fungi, Plant, Structure};
use crate::terrain::Tile;
use crate::units::{Ant, Unit};
use crate::utils::Position;

use crate::config::MAP_SIZE;
use crate::config::{N_ANT, N_FUNGI, N_PLANT};

pub struct GenerationPlugin;
impl Plugin for GenerationPlugin {
	fn build(&self, app: &mut AppBuilder) {
		app.add_resource(GenerationConfig::new())
			.add_startup_system(generate_terrain.system())
			.add_startup_system(generate_entities.system());
	}
}

#[derive(Copy, Clone)]
pub struct GenerationConfig {
	pub map_size: isize,
	n_ant: usize,
	n_plant: usize,
	n_fungi: usize,
}

impl GenerationConfig {
	fn new() -> GenerationConfig {
		assert!((N_ANT + N_PLANT + N_FUNGI) <= (MAP_SIZE * MAP_SIZE).try_into().unwrap());
		GenerationConfig {
			map_size: MAP_SIZE,
			n_ant: N_ANT,
			n_plant: N_PLANT,
			n_fungi: N_FUNGI,
		}
	}
}

// TODO: make reusable terrain and entity generators
fn generate_terrain(
	mut commands: Commands,
	config: Res<GenerationConfig>,
	asset_server: Res<AssetServer>,
	mut materials: ResMut<Assets<ColorMaterial>>,
) {
	println!("Generating terrain.");

	let map_size = config.map_size;

	assert!(map_size > 0);

	let positions = (0..map_size).cartesian_product(0..map_size);

	let handle = asset_server.get_handle("tile.png");
	let my_material = materials.add(handle.into());

	let center = Position { alpha: 0, beta: 0 };
	let positions = Position::hexagon(center, config.map_size);
	for position in positions {
		commands
			.spawn(make_sprite_components(&position, my_material.clone()))
			.with(Tile {})
			.with(position);
	}

	println!("Terrain generated.");
}

fn generate_entities(
	mut commands: Commands,
	config: Res<GenerationConfig>,
	asset_server: Res<AssetServer>,
	mut materials: ResMut<Assets<ColorMaterial>>,
) {
	println!("Generating entities.");

	let n_ant = config.n_ant;
	let n_plant = config.n_plant;
	let n_fungi = config.n_fungi;

	let n_entities = n_ant + n_plant + n_fungi;

	let center = Position { alpha: 0, beta: 0 };
	let possible_positions = Position::hexagon(center, config.map_size);
	let mut rng = &mut rand::thread_rng();
	let mut entity_positions: Vec<_> = possible_positions
		.choose_multiple(&mut rng, n_entities)
		.cloned()
		.collect();

	// TODO: Figure out how to swap this to spawn_batch
	// The main challenge is figuring out how to add the extra components

	// Ant
	let handle = asset_server.get_handle("ant.png");
	let my_material = materials.add(handle.into());
	let positions = entity_positions.split_off(entity_positions.len() - n_ant);

	for position in positions {
		commands
			.spawn(make_sprite_components(&position, my_material.clone()))
			.with(Unit {})
			.with(Ant {})
			.with(position);
	}
	// Plant
	let handle = asset_server.get_handle("plant.png");
	let my_material = materials.add(handle.into());
	let positions = entity_positions.split_off(entity_positions.len() - n_plant);

	for position in positions {
		commands
			.spawn(make_sprite_components(&position, my_material.clone()))
			.with(Structure {})
			.with(Plant {})
			.with(position);
	}
	// Fungi
	let handle = asset_server.get_handle("fungi.png");
	let my_material = materials.add(handle.into());
	let positions = entity_positions.split_off(entity_positions.len() - n_fungi);

	for position in positions {
		commands
			.spawn(make_sprite_components(&position, my_material.clone()))
			.with(Structure {})
			.with(Fungi {})
			.with(position);
	}

	println!("Entities generated.");
}
