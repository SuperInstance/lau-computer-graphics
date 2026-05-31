//! Agent visualization: rendering agent state spaces and interactions.

use nalgebra::Vector2;
use crate::color::Rgb;
use crate::rasterize::Framebuffer;

/// An agent in a 2D state space.
#[derive(Debug, Clone)]
pub struct Agent {
    pub id: usize,
    pub position: Vector2<f64>,
    pub state: Vec<f64>,
    pub color: Rgb,
    pub label: String,
}

/// An interaction (edge) between two agents.
#[derive(Debug, Clone)]
pub struct Interaction {
    pub from: usize,
    pub to: usize,
    pub strength: f64,
    pub color: Rgb,
}

/// A scene of agents and their interactions.
#[derive(Debug, Clone)]
pub struct AgentScene {
    pub agents: Vec<Agent>,
    pub interactions: Vec<Interaction>,
    pub bounds: (Vector2<f64>, Vector2<f64>),
}

impl AgentScene {
    pub fn new(min_bound: Vector2<f64>, max_bound: Vector2<f64>) -> Self {
        Self {
            agents: Vec::new(),
            interactions: Vec::new(),
            bounds: (min_bound, max_bound),
        }
    }

    pub fn add_agent(&mut self, agent: Agent) {
        self.agents.push(agent);
    }

    pub fn add_interaction(&mut self, interaction: Interaction) {
        self.interactions.push(interaction);
    }

    /// Render the scene to a framebuffer.
    pub fn render(&self, width: usize, height: usize) -> Framebuffer {
        let mut fb = Framebuffer::new(width, height);
        let (min_b, max_b) = &self.bounds;
        let range_x = max_b.x - min_b.x;
        let range_y = max_b.y - min_b.y;

        let to_screen = |p: &Vector2<f64>| -> (i32, i32) {
            let sx = ((p.x - min_b.x) / range_x * (width as f64 - 1.0)) as i32;
            let sy = ((max_b.y - p.y) / range_y * (height as f64 - 1.0)) as i32; // flip y
            (sx, sy)
        };

        let to_u32 = |c: &Rgb| -> u32 {
            let clamped = c.clamp();
            let r = (clamped.r * 255.0) as u32;
            let g = (clamped.g * 255.0) as u32;
            let b = (clamped.b * 255.0) as u32;
            (r << 16) | (g << 8) | b
        };

        // Draw interactions (edges)
        for inter in &self.interactions {
            if inter.from < self.agents.len() && inter.to < self.agents.len() {
                let from = to_screen(&self.agents[inter.from].position);
                let to = to_screen(&self.agents[inter.to].position);
                fb.draw_line(from.0, from.1, to.0, to.1, to_u32(&inter.color));
            }
        }

        // Draw agents (as 3x3 squares for visibility)
        for agent in &self.agents {
            let (sx, sy) = to_screen(&agent.position);
            let color = to_u32(&agent.color);
            for dx in -2..=2 {
                for dy in -2..=2 {
                    fb.set_pixel(sx + dx, sy + dy, color);
                }
            }
        }

        fb
    }
}

/// Compute state-space distance between two agents.
pub fn state_distance(a: &Agent, b: &Agent) -> f64 {
    if a.state.len() != b.state.len() {
        return f64::INFINITY;
    }
    a.state
        .iter()
        .zip(&b.state)
        .map(|(x, y)| (x - y).powi(2))
        .sum::<f64>()
        .sqrt()
}

/// Find all agents within a radius in state space.
pub fn agents_within_radius(agents: &[Agent], center: &Agent, radius: f64) -> Vec<usize> {
    agents
        .iter()
        .enumerate()
        .filter(|(i, a)| {
            if *i == center.id { return false; }
            state_distance(a, center) <= radius
        })
        .map(|(i, _)| i)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_scene_render() {
        let mut scene = AgentScene::new(Vector2::new(0.0, 0.0), Vector2::new(10.0, 10.0));
        scene.add_agent(Agent {
            id: 0,
            position: Vector2::new(2.0, 2.0),
            state: vec![1.0, 0.0],
            color: Rgb::new(1.0, 0.0, 0.0),
            label: "A".into(),
        });
        scene.add_agent(Agent {
            id: 1,
            position: Vector2::new(8.0, 8.0),
            state: vec![0.0, 1.0],
            color: Rgb::new(0.0, 0.0, 1.0),
            label: "B".into(),
        });
        scene.add_interaction(Interaction {
            from: 0,
            to: 1,
            strength: 1.0,
            color: Rgb::new(0.5, 0.5, 0.5),
        });

        let fb = scene.render(100, 100);
        // Agents and line should have drawn pixels
        let mut non_zero = 0;
        for row in &fb.pixels {
            for &px in row {
                if px != 0 {
                    non_zero += 1;
                }
            }
        }
        assert!(non_zero > 10); // at least some pixels drawn
    }

    #[test]
    fn test_state_distance_same() {
        let a = Agent {
            id: 0, position: Vector2::new(0.0, 0.0),
            state: vec![1.0, 2.0, 3.0], color: Rgb::new(1.0, 1.0, 1.0), label: "A".into(),
        };
        let b = Agent {
            id: 1, position: Vector2::new(0.0, 0.0),
            state: vec![1.0, 2.0, 3.0], color: Rgb::new(1.0, 1.0, 1.0), label: "B".into(),
        };
        assert_eq!(state_distance(&a, &b), 0.0);
    }

    #[test]
    fn test_state_distance_different() {
        let a = Agent {
            id: 0, position: Vector2::new(0.0, 0.0),
            state: vec![0.0, 0.0], color: Rgb::new(1.0, 1.0, 1.0), label: "A".into(),
        };
        let b = Agent {
            id: 1, position: Vector2::new(0.0, 0.0),
            state: vec![3.0, 4.0], color: Rgb::new(1.0, 1.0, 1.0), label: "B".into(),
        };
        assert_eq!(state_distance(&a, &b), 5.0);
    }

    #[test]
    fn test_agents_within_radius() {
        let agents = vec![
            Agent { id: 0, position: Vector2::new(0.0, 0.0), state: vec![0.0], color: Rgb::new(1.0, 0.0, 0.0), label: "A".into() },
            Agent { id: 1, position: Vector2::new(0.0, 0.0), state: vec![1.0], color: Rgb::new(0.0, 1.0, 0.0), label: "B".into() },
            Agent { id: 2, position: Vector2::new(0.0, 0.0), state: vec![10.0], color: Rgb::new(0.0, 0.0, 1.0), label: "C".into() },
        ];
        let nearby = agents_within_radius(&agents, &agents[0], 2.0);
        assert_eq!(nearby, vec![1]);
    }
}
