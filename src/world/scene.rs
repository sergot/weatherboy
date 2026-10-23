use crate::point::Point;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Material {
    Meadow,
    Hill,
}

struct Ridge {
    base: f32,
    waves: Vec<(f32, f32, f32)>,
}

impl Ridge {
    fn top(&self, x: f32) -> f32 {
        self.base
            - self
                .waves
                .iter()
                .map(|&(a, f, phase)| a * (f * x + phase).sin())
                .sum::<f32>()
    }
}

pub struct Layer {
    pub material: Material,
    pub depth: f32, // the bigger the further
    ridge: Ridge,
}

pub struct Scene {
    layers: Vec<Layer>,
}

impl Scene {
    pub fn hills() -> Self {
        let layer = |material, depth, base, waves| Layer {
            material,
            depth,
            ridge: Ridge { base, waves },
        };
        Self {
            layers: vec![
                layer(Material::Meadow, 0.0, 0.85, vec![(0.01, 3.0, 0.0)]),
                layer(
                    Material::Hill,
                    1.0,
                    0.68,
                    vec![(0.04, 4.0, 0.0), (0.015, 11.0, 1.3)],
                ),
                layer(
                    Material::Hill,
                    3.0,
                    0.58,
                    vec![(0.05, 2.5, 2.0), (0.02, 7.0, 0.4)],
                ),
            ],
        }
    }

    pub fn layer_at(&self, p: Point) -> Option<&Layer> {
        self.layers.iter().find(|layer| p.y >= layer.ridge.top(p.x))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn flat(material: Material, base: f32) -> Layer {
        Layer {
            material,
            depth: 0.0,
            ridge: Ridge {
                base,
                waves: vec![],
            },
        }
    }

    #[test]
    fn nearest_covering_layer_wins() {
        let scene = Scene {
            layers: vec![flat(Material::Meadow, 0.8), flat(Material::Hill, 0.5)],
        };
        let at = |y| scene.layer_at(Point { x: 0.0, y }).map(|l| l.material);

        assert_eq!(at(0.3), None); // above all is sky
        assert_eq!(at(0.6), Some(Material::Hill));
        assert_eq!(at(0.9), Some(Material::Meadow));
    }

    #[test]
    fn wave_peak_raises_ridge() {
        let ridge = Ridge {
            base: 0.5,
            waves: vec![(0.1, 1.0, 0.0)],
        };
        assert!((ridge.top(std::f32::consts::FRAC_PI_2) - 0.4).abs() < 1e-6);
    }
}
