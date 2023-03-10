use crate::astra_net::activation::{Activation, LeakyReLU, Softmax};
use crate::astra_net::layer::LayerDense;
use crate::astra_net::Net;

use crate::tensor::Tensor;

mod astra_net;
mod tensor;
use ndarray_rand::rand_distr::{Distribution, Normal};
use rand::Rng;

pub struct MutatingNet {
    config: Vec<usize>,
    weights: Vec<Tensor>,
    biases: Vec<Tensor>,
    activation: Box<dyn Activation>,
    mutation_rate: f64,
}

impl MutatingNet {
    pub fn from_config(config: Vec<usize>) -> Self {
        let mut rng = rand::thread_rng();
        let normal = Normal::new(-1.0, 1.0).unwrap();
        Self {
            config: config.clone(),
            weights: config
                .clone()
                .into_iter()
                .zip(config.clone().into_iter().skip(1))
                .map(|(prev, cur)| {
                    Tensor::from_vec(
                        (0..cur * prev).map(|_| normal.sample(&mut rng)).collect(),
                        vec![cur, prev],
                    ) * (2.0 / (prev + cur) as f64).sqrt()
                })
                .collect(),
            biases: config
                .into_iter()
                .map(|sz| Tensor::from_element(0.0, vec![sz]))
                .collect(),
            activation: Box::new(LeakyReLU::new(0.3)),
            mutation_rate: 0.15,
        }
    }

    pub fn feed_forward(&self, input: &Tensor) -> Tensor {
        let mut output = input.to_owned().reshape(vec![1, input.len()]).unwrap();

        for (weight, bias) in self
            .weights
            .to_owned()
            .into_iter()
            .zip(self.biases.to_owned().into_iter())
        {
            output =
                output.dot(&weight).unwrap() + bias.clone().reshape(vec![1, bias.len()]).unwrap();

            output = self.activation.call(output)
        }
        output
    }

    pub fn crossover(mut self, right: &Self) -> Option<Self> {
        if self.config != right.config.to_owned() {
            return None;
        }

        let mut rng = rand::thread_rng();

        self.weights = self
            .weights
            .into_iter()
            .zip(right.weights.to_owned().into_iter())
            .map(|(weights_self, weights_right)| {
                Tensor::from_vec(
                    weights_self
                        .clone()
                        .into_iter()
                        .zip(weights_right.into_iter())
                        .map(|(w_l, w_r)| {
                            if rng.gen_range(0.0..1.0) < 0.5 {
                                w_l
                            } else {
                                w_r
                            }
                        })
                        .collect(),
                    weights_self.shape,
                )
            })
            .collect();
        self.biases = self
            .biases
            .into_iter()
            .zip(right.biases.to_owned().into_iter())
            .map(|(weights_self, weights_right)| {
                Tensor::from_vec(
                    weights_self
                        .clone()
                        .into_iter()
                        .zip(weights_right.into_iter())
                        .map(|(w_l, w_r)| {
                            if rng.gen_range(0.0..1.0) < 0.5 {
                                w_l
                            } else {
                                w_r
                            }
                        })
                        .collect(),
                    weights_self.shape,
                )
            })
            .collect();

        Some(self)
    }
}

fn main() {
    let config = vec![5, 3, 5];
    let mt = MutatingNet::from_config(config.clone());
    let mt2 = MutatingNet::from_config(config);

    let inp = Tensor::from_element(1.0, vec![5]);
    let ff = mt.feed_forward(&inp);

    let mt_co = mt.crossover(&mt2).unwrap();
    let ff_co = mt_co.feed_forward(&inp);

    println!("{:#?}", ff);
    println!("{:#?}", ff_co);

    test_astra_net_tensor();
}

// use bevy::prelude::*;

// fn main() {
//     App::new()
//     .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
//     .add_plugins(DefaultPlugins.set(WindowPlugin {
//         window: WindowDescriptor {
//             title: "Rust Invaders!".to_string(),
//             width: 598.0,
//             height: 676.0,
//             ..Default::default()
//         },
//         ..Default::default()
//     }))
//     .add_startup_system(setup)
//     .run();
// }

// #[derive(Resource)]
// pub struct WinSize {
// 	pub w: f32,
// 	pub h: f32,
// }

// #[derive(Resource)]
// struct GameTextures {
// 	player: Handle<Image>,
// 	player_laser: Handle<Image>,
// 	enemy: Handle<Image>,
// 	enemy_laser: Handle<Image>,
// 	explosion: Handle<TextureAtlas>,
// }

// #[derive(Resource)]
// struct EnemyCount(u32);

// fn setup(
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
//     mut texture_atlases: ResMut<Assets<TextureAtlas>>,
//     mut windows: ResMut<Windows>,
// ) {
//     // 2D Camera Bundle
//     commands.spawn(Camera2dBundle::default());

//     let window = windows.get_primary_mut().unwrap();
//     let (win_w, win_h) = (window.width(), window.height());

//     let win_size = WinSize {w: win_w, h: win_h};
//     commands.insert_resource(win_size);

//     let texture_handle = asset_server.load("explo_a_sheet.png");
//     let texture_atlas =
// 		TextureAtlas::from_grid(texture_handle, Vec2::new(64., 64.), 4, 4, None, None);
//     let explosion = texture_atlases.add(texture_atlas);

//     let game_textures = GameTextures {
// 		player: asset_server.load("player_a_01.png"),
// 		player_laser: asset_server.load("laser_a_01.png"),
// 		enemy: asset_server.load("enemy_a_01.png"),
// 		enemy_laser: asset_server.load("laser_b_01.png"),
// 		explosion,
// 	};
// 	commands.insert_resource(game_textures);
// 	commands.insert_resource(EnemyCount(0));

// }

fn test_astra_net_tensor() {
    let l1 = Box::new(LayerDense::new(6, 2, Box::new(LeakyReLU::new(0.1))));
    let l2 = Box::new(LayerDense::new(6, 6, Box::new(LeakyReLU::new(0.1))));
    let l3 = Box::new(LayerDense::new(12, 6, Box::new(LeakyReLU::new(0.1))));
    let l4 = Box::new(LayerDense::new(6, 12, Box::new(LeakyReLU::new(0.1))));
    let lend = Box::new(LayerDense::new(2, 6, Box::new(Softmax::new())));

    let mut my_net = Net::new();
    my_net.set_learning_rate(0.001);

    my_net.add_layer(l1);
    my_net.add_layer(l2);
    my_net.add_layer(l3);
    my_net.add_layer(l4);
    my_net.add_layer(lend);

    let input_data = generate_2d_cluster_dataset(50000);

    let target_data: Vec<Tensor> = (0..input_data.len())
        .map(|x| {
            if x < input_data.len() / 2 {
                Tensor::from_vec(vec![1.0, 0.0], vec![2])
            } else {
                Tensor::from_vec(vec![0.0, 1.0], vec![2])
            }
        })
        .collect();

    for (input, target) in input_data.into_iter().zip(target_data.into_iter()) {
        my_net.back_propagation(&input, &target);
    }

    let test1 = my_net.feed_forward(&Tensor::from_vec(vec![-3.0, -3.0], vec![2]));

    println!("test1, should be 1 , 0  {:#?}", test1);

    let test2 = my_net.feed_forward(&Tensor::from_vec(vec![8.0, 8.0], vec![2]));

    println!("test2, should be 0 , 1  {:#?}", test2);
}

fn generate_2d_cluster_dataset(num_samples: usize) -> Vec<Tensor> {
    let mut rng = rand::thread_rng();
    let mut data: Vec<Tensor> = Vec::with_capacity(num_samples);
    for _ in 0..num_samples / 2 {
        let x1 = rng.gen_range(-5.0..5.0);
        let x2 = rng.gen_range(-5.0..5.0);
        data.push(Tensor::from_vec(vec![x1, x2], vec![2]));
    }
    for _ in num_samples / 2..num_samples {
        let x1 = rng.gen_range(5.0..10.0);
        let x2 = rng.gen_range(5.0..10.0);
        data.push(Tensor::from_vec(vec![x1, x2], vec![2]));
    }
    data
}
