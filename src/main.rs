pub mod risk {
    use rand::{distributions::Uniform, prelude::Distribution};


#[derive(Debug)]
pub enum RegionId {
  // North America
  Alaska,
  NorthwestTerritory,
  Greenland,
  Alberta,
  Ontario,
  Quebec,
  WesternUnitedStates,
  EasternUnitedStates,
  CentralAmerica,

  // South America
  Venezuela,
  Peru,
  Brazil,
  Argentina,

  // Europe
  Iceland,
  Scandinavia,
  Russia,
  GreatBritain,
  NorthernEurope,
  WesternEurope,
  SouthernEurope,

  // Africa
  NorthAfrica,
  Egypt,
  EastAfrica,
  CentralAfrica,
  SouthAfrica,
  Madagascar,

  // Asia
  Ural,
  Siberia,
  Yakutsk,
  Kamchatka,
  Irkutsk,
  Mongolia,
  Japan,
  Afghanistan,
  China,
  MiddleEast,
  India,
  SoutheastAsia,

  // Australia
  Indonesia,
  NewGuinea,
  WesternAustralia,
  EasternAustralia,
}

#[derive(Debug)]
pub struct Region{
  pub id: RegionId,
  pub owner: i8,
  pub num_troops: i16,
  pub neighbors: Vec<[i8; 2]>,
}

#[derive(Debug)]
pub struct Continent {
  pub regions: Vec<Region>,
  pub bonus: i8
}

pub struct Game {
  pub continents: Vec<Continent>,
  pub current_player: i8,
  pub num_players: i8,
}

impl Game {
  pub fn num_draft_troops(self, player_id: i8) -> i16 {
    let mut sum: f32 = 0.0;
    for continent in self.continents.iter() {
      let mut owns_continent = true;
      for region in continent.regions.iter() {
        if region.owner == player_id {
          sum += 1.0/3.0;
        }
        else{
          owns_continent = false;
        }
      }
      if owns_continent {
        sum += continent.bonus as f32;
      }
    }
    return sum.floor() as i16;
  }
  pub fn battle(mut self, num_attackers: i16, num_defenders: i16, from: [i8; 2], to: [i8; 2], num_moving_attackers: i16) -> i8{
    assert!(num_attackers > 0);
    assert!(num_attackers <= 3);
    assert!(num_attackers <= self.continents[from[0] as usize].regions[from[1] as usize].num_troops-1);

    assert!(num_defenders > 0);
    assert!(num_defenders <= 2);
    assert!(num_defenders <= self.continents[to[0] as usize].regions[to[1] as usize].num_troops);

    assert!(self.continents[from[0] as usize].regions[from[1] as usize].neighbors.contains(&to));

    let mut rng = rand::thread_rng();
    let distribution = Uniform::from(1..7);

    let mut attack_dice = vec![0; num_attackers as usize];
    for i in 0..num_attackers {
      attack_dice[i as usize] = distribution.sample(&mut rng);
    }
    attack_dice.sort_by(|a, b| b.cmp(a));

    let mut defend_dice = vec![0; num_defenders as usize];
    for i in 0..num_defenders {
      defend_dice[i as usize] = distribution.sample(&mut rng);
    }
    defend_dice.sort_by(|a, b| b.cmp(a));

    for i in 0..num_attackers.min(num_defenders) {
      if attack_dice[i as usize] > defend_dice[i as usize] {
        self.continents[to[0] as usize].regions[to[1] as usize].num_troops -= 1;
      }
      else {
        self.continents[from[0] as usize].regions[from[1] as usize].num_troops -= 1;
      }
    }

    if self.continents[to[0] as usize].regions[to[1] as usize].num_troops == 0 {
      self.continents[to[0] as usize].regions[to[1] as usize].owner = self.continents[from[0] as usize].regions[from[1] as usize].owner;
      self.continents[to[0] as usize].regions[to[1] as usize].num_troops = num_moving_attackers;
      self.continents[from[0] as usize].regions[from[1] as usize].num_troops -= num_moving_attackers;
    }
    return 0;
  }
}

pub fn init(num_players: i8) -> Game {
  let mut continents: Vec<Continent> = Vec::new();
  continents.push(Continent{regions: 
    vec![
      Region { id: RegionId::Alaska, owner: -1, num_troops: 0, neighbors: Vec::new() },
      Region { id: RegionId::NorthwestTerritory, owner: -1, num_troops: 0, neighbors: Vec::new() },
      Region { id: RegionId::Greenland, owner: -1, num_troops: 0, neighbors: Vec::new() },
      Region { id: RegionId::Alberta, owner: -1, num_troops: 0, neighbors: Vec::new() },
      Region { id: RegionId::Ontario, owner: -1, num_troops: 0, neighbors: Vec::new() },
      Region { id: RegionId::Quebec, owner: -1, num_troops: 0, neighbors: Vec::new() },
      Region { id: RegionId::WesternUnitedStates, owner: -1, num_troops: 0, neighbors: Vec::new() },
      Region { id: RegionId::EasternUnitedStates, owner: -1, num_troops: 0, neighbors: Vec::new() },
      Region { id: RegionId::CentralAmerica, owner: -1, num_troops: 0, neighbors: Vec::new() },
    ],
  bonus: 5});

  continents.push(Continent{regions: 
    vec![
      Region { id: RegionId::Venezuela, owner: -1, num_troops: 0, neighbors: Vec::new() },
      Region { id: RegionId::Peru, owner: -1, num_troops: 0, neighbors: Vec::new() },
      Region { id: RegionId::Brazil, owner: -1, num_troops: 0, neighbors: Vec::new() },
      Region { id: RegionId::Argentina, owner: -1, num_troops: 0, neighbors: Vec::new() },
    ],
  bonus: 2});

  continents.push(Continent{regions:
    vec![
      Region { id: RegionId::Iceland, owner: -1, num_troops: 0, neighbors: Vec::new() },
      Region { id: RegionId::Scandinavia, owner: -1, num_troops: 0, neighbors: Vec::new() },
      Region { id: RegionId::Russia, owner: -1, num_troops: 0, neighbors: Vec::new() },
      Region { id: RegionId::GreatBritain, owner: -1, num_troops: 0, neighbors: Vec::new() },
      Region { id: RegionId::NorthernEurope, owner: -1, num_troops: 0, neighbors: Vec::new() },
      Region { id: RegionId::WesternEurope, owner: -1, num_troops: 0, neighbors: Vec::new() },
      Region { id: RegionId::SouthernEurope, owner: -1, num_troops: 0, neighbors: Vec::new() },
    ],
  bonus: 5});

  continents.push(Continent{regions:
    vec![
      Region { id: RegionId::NorthAfrica, owner: -1, num_troops: 0, neighbors: Vec::new() },
      Region { id: RegionId::Egypt, owner: -1, num_troops: 0, neighbors: Vec::new() },
      Region { id: RegionId::EastAfrica, owner: -1, num_troops: 0, neighbors: Vec::new() },
      Region { id: RegionId::CentralAfrica, owner: -1, num_troops: 0, neighbors: Vec::new() },
      Region { id: RegionId::SouthAfrica, owner: -1, num_troops: 0, neighbors: Vec::new() },
      Region { id: RegionId::Madagascar, owner: -1, num_troops: 0, neighbors: Vec::new() },
    ],
  bonus: 3});

  continents.push(Continent{regions:
    vec![
      Region { id: RegionId::Ural, owner: -1, num_troops: 0, neighbors: Vec::new() },
      Region { id: RegionId::Siberia, owner: -1, num_troops: 0, neighbors: Vec::new() },
      Region { id: RegionId::Yakutsk, owner: -1, num_troops: 0, neighbors: Vec::new() },
      Region { id: RegionId::Kamchatka, owner: -1, num_troops: 0, neighbors: Vec::new() },
      Region { id: RegionId::Irkutsk, owner: -1, num_troops: 0, neighbors: Vec::new() },
      Region { id: RegionId::Mongolia, owner: -1, num_troops: 0, neighbors: Vec::new() },
      Region { id: RegionId::Japan, owner: -1, num_troops: 0, neighbors: Vec::new() },
      Region { id: RegionId::Afghanistan, owner: -1, num_troops: 0, neighbors: Vec::new() },
      Region { id: RegionId::China, owner: -1, num_troops: 0, neighbors: Vec::new() },
      Region { id: RegionId::MiddleEast, owner: -1, num_troops: 0, neighbors: Vec::new() },
      Region { id: RegionId::India, owner: -1, num_troops: 0, neighbors: Vec::new() },
      Region { id: RegionId::SoutheastAsia, owner: -1, num_troops: 0, neighbors: Vec::new() },
    ],
  bonus: 7});

  continents.push(Continent{regions:
    vec![
      Region { id: RegionId::Indonesia, owner: -1, num_troops: 0, neighbors: Vec::new() },
      Region { id: RegionId::NewGuinea, owner: -1, num_troops: 0, neighbors: Vec::new() },
      Region { id: RegionId::WesternAustralia, owner: -1, num_troops: 0, neighbors: Vec::new() },
      Region { id: RegionId::EasternAustralia, owner: -1, num_troops: 0, neighbors: Vec::new() },
    ],
  bonus: 2});

  continents[0].regions[0].neighbors = vec![
    [0, 1],
    [0, 3],
    [4, 3]];

  continents[0].regions[1].neighbors = vec![
    [0, 0],
    [0, 2],
    [0, 3],
    [0, 4],];

  continents[0].regions[2].neighbors = vec![
    [0, 1],
    [0, 4],
    [0, 5],
    [2, 0],];

  continents[0].regions[3].neighbors = vec![
    [0, 0],
    [0, 1],
    [0, 4],
    [0, 6],];

  continents[0].regions[4].neighbors = vec![
    [0, 1],
    [0, 2],
    [0, 3],
    [0, 5],
    [0, 6],
    [0, 7],];

  continents[0].regions[5].neighbors = vec![
    [0, 2],
    [0, 4],
    [0, 7],];

  continents[0].regions[6].neighbors = vec![
    [0, 3],
    [0, 4],
    [0, 7],
    [0, 8],];

  continents[0].regions[7].neighbors = vec![
    [0, 4],
    [0, 5],
    [0, 6],
    [0, 8],];

  continents[0].regions[8].neighbors = vec![
    [0, 6],
    [0, 7],
    [1, 0],];

  continents[1].regions[0].neighbors = vec![
    [0, 8],
    [1, 1],
    [1, 2],];
  
  continents[1].regions[1].neighbors = vec![
    [1, 0],
    [1, 2],
    [1, 3],];
  
  continents[1].regions[2].neighbors = vec![
    [1, 0],
    [1, 1],
    [1, 3],
    [3, 0],];

  continents[1].regions[3].neighbors = vec![
    [1, 1],
    [1, 2],];
  
  continents[2].regions[0].neighbors = vec![
    [0, 8],
    [2, 1],
    [2, 3],];

  continents[2].regions[1].neighbors = vec![
    [2, 0],
    [2, 2],
    [2, 3],
    [2, 4],];
  
  continents[2].regions[2].neighbors = vec![
    [2, 1],
    [2, 4],
    [2, 6],
    [4, 0],
    [4, 7],
    [4, 9],];

  continents[2].regions[3].neighbors = vec![
    [2, 0],
    [2, 1],
    [2, 4],
    [2, 5],];

  continents[2].regions[4].neighbors = vec![
    [2, 1],
    [2, 2],
    [2, 3],
    [2, 5],
    [2, 6],];

  continents[2].regions[5].neighbors = vec![
    [2, 3],
    [2, 4],
    [2, 6],
    [3, 0],];

  continents[2].regions[6].neighbors = vec![
    [2, 2],
    [2, 4],
    [2, 5],
    [3, 0],
    [3, 1],
    [4, 9],];

  continents[3].regions[0].neighbors = vec![
    [1, 2],
    [2, 5],
    [2, 6],
    [3, 1],
    [3, 2],
    [3, 3],];

  continents[3].regions[1].neighbors = vec![
    [2, 6],
    [3, 0],
    [3, 2],
    [4, 9],];

  continents[3].regions[2].neighbors = vec![
    [3, 0],
    [3, 1],
    [3, 3],
    [3, 4],
    [3, 5],
    [4, 9],];

  continents[3].regions[3].neighbors = vec![
    [3, 0],
    [3, 2],
    [3, 4],];

  continents[3].regions[4].neighbors = vec![
    [3, 2],
    [3, 3],
    [3, 5],];

  continents[3].regions[5].neighbors = vec![
    [3, 2],
    [3, 4],];
  
  continents[4].regions[0].neighbors = vec![
    [2, 2],
    [4, 1],
    [4, 7],
    [4, 8],];

  continents[4].regions[1].neighbors = vec![
    [4, 0],
    [4, 2],
    [4, 4],
    [4, 5],
    [4, 8],];
  
  continents[4].regions[2].neighbors = vec![
    [4, 1],
    [4, 3],
    [4, 4],];

  continents[4].regions[3].neighbors = vec![
    [0, 0],
    [4, 2],
    [4, 4],
    [4, 5],
    [4, 6],];

  continents[4].regions[4].neighbors = vec![
    [4, 1],
    [4, 2],
    [4, 3],
    [4, 5],];

  continents[4].regions[5].neighbors = vec![
    [4, 1],
    [4, 3],
    [4, 4],
    [4, 6],
    [4, 8],];

  continents[4].regions[6].neighbors = vec![
    [4, 3],
    [4, 5],];

  continents[4].regions[7].neighbors = vec![
    [2, 2],
    [4, 0],
    [4, 8],
    [4, 9],
    [4, 10],];

  continents[4].regions[8].neighbors = vec![
    [4, 0],
    [4, 1],
    [4, 5],
    [4, 7],
    [4, 10],
    [4, 11],];

  continents[4].regions[9].neighbors = vec![
    [2, 2],
    [2, 6],
    [3, 1],
    [3, 2],
    [4, 7],
    [4, 10],];

  continents[4].regions[10].neighbors = vec![
    [4, 7],
    [4, 8],
    [4, 9],
    [4, 11],];

  continents[4].regions[11].neighbors = vec![
    [4, 8],
    [4, 10],
    [5, 0],];
  
  continents[5].regions[0].neighbors = vec![
    [4, 11],
    [5, 1],
    [5, 2],];

  continents[5].regions[1].neighbors = vec![
    [5, 0],
    [5, 2],
    [5, 3],];

  continents[5].regions[2].neighbors = vec![
    [5, 0],
    [5, 1],
    [5, 3],];

  continents[5].regions[3].neighbors = vec![
    [5, 1],
    [5, 2],];

  return Game{continents: continents, current_player: 0, num_players: num_players};
}
}

fn main() {
  let mut game = risk::init(4);
  for i in 0..100 {
    // print!("{:#?}",game.battle(0, 0));
  }
  // let neighbor = game.continents[5].regions[3].neighbors[0];
  // print!("{:#?}", game.continents[neighbor[0] as usize].regions[neighbor[1] as usize]);
}
