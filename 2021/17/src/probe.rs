use nalgebra::Vector2;

pub type Position = Vector2<isize>;
pub type Velocity = Vector2<isize>;

const DRAG: Velocity = Velocity::new(1, 0);
const GRAV: Velocity = Velocity::new(0, 1);

#[derive(Default, Debug)]
pub struct Probe;

impl Probe {
    pub fn launch(vel: &Velocity, target: &(Position, Position)) -> bool {
        for (p, _) in Probe::trajectory(Position::new(0, 0), *vel) {
            // if we hit at any point return success
            if Probe::is_within(&p, target) {
                return true;
            // if we have overshot then bail out
            } else if Probe::no_chance(&p, target) {
                return false
            }
        }
        false
    }

    fn trajectory(pos: Position, vel: Velocity) -> impl Iterator<Item = (Position, Velocity)> {
        std::iter::successors(Some((pos, vel)), |(pos, vel)| {
            // first update position using current velocity
            let newpos = pos + vel;

            // next re-calc velocity
            let mut newvel = *vel;

            // subtract gravity 
            newvel -= GRAV;
            
            // subtract drag 
            // signum returns either -1, 0, 1 when negative, 0, positive respectively
            // this allows us to add/subtract DRAG towards x=0
            newvel -= vel.x.signum() * DRAG;

            Some((newpos, newvel))
        })
    }

    fn is_within(pos: &Position, (min, max): &(Position, Position)) -> bool {
        pos.x >= min.x && pos.x <= max.x && pos.y >= min.y && pos.y <= max.y
    }

    fn no_chance(pos: &Position, (min, max): &(Position, Position)) -> bool {
        pos.x > max.x || pos.y < min.y
    }
}
