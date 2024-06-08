use std::{fs::File, io::Read, path::Path, str::FromStr};


pub fn read_file(path: &Path) -> Vec<Atom> {
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    let _ = file.read_to_string(&mut s);
    let mut atoms = Vec::new();
    s.lines()
        .enumerate()
        .filter(|(i, _)| i > &1)
        .for_each(|(_, line)| {
        let m = line.split_whitespace().collect::<Vec<_>>();
        let element = Elements::from_str(m[0]).unwrap();
        let position = [
            m[1].parse::<f32>().unwrap(),
            m[2].parse::<f32>().unwrap(),
            m[3].parse::<f32>().unwrap(),
        ];
        let atom = def_atom(element, position);
        atoms.push(atom);
    });
    atoms
}

#[derive(Default, Debug)]
pub struct Atom {
    pub element: Option<String>,
    pub atomic_number: Option<u32>,
    pub color: Color,
    pub x: Option<f32>,
    pub y: Option<f32>,
    pub z: Option<f32>,
}
impl Atom {
  //pub fn distances(&self, atoms: &Vec<Atom>) -> Vec<Vec3> {
  pub fn distances(&self, atoms: &[Atom]) -> Vec<Vec3> {
       let distances = atoms.iter()
           .map(|atom| 
               Vec3::new(atom.x.expect(""), atom.y.expect(""), atom.z.expect("")) - 
               Vec3::new(self.x.expect(""), self.y.expect(""), self.z.expect("")))
           .collect::<Vec<Vec3>>();
      println!("distances in fun: {:?}", distances);
      distances 
   } 
}

#[derive(Debug)]
enum Elements {
    H,
    C,
    O,
}
impl FromStr for Elements {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "H" => Ok(Elements::H),
            "C" => Ok(Elements::C),
            "O" => Ok(Elements::O),
            _ => Err(()),
        }
    }
    
}

fn def_atom(element: Elements, position: [f32; 3]) -> Atom {
    let mut atom = match element {
        Elements::H => Atom {
            element: Some("H".to_string()),
            atomic_number: Some(1),
            color: WHITE,
            ..Default::default()
        },
        Elements::C => Atom {
            element: Some("C".to_string()),
            atomic_number: Some(6),
            color: BLACK,
            ..Default::default()
        },
        Elements::O => Atom {
            element: Some("O".to_string()),
            atomic_number: Some(6),
            color: RED,
            ..Default::default()
        },
    };
    atom.x = Some(position[0]);
    atom.y = Some(position[1]);
    atom.z = Some(position[2]);
    atom
}
