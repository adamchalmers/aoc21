const POS: Dir = Dir::Pos;
const NEG: Dir = Dir::Neg;
const X: (Axis, Dir) = (Axis::X, POS);
const Y: (Axis, Dir) = (Axis::Y, POS);
const Z: (Axis, Dir) = (Axis::Z, POS);
const X_: (Axis, Dir) = (Axis::X, NEG);
const Y_: (Axis, Dir) = (Axis::Y, NEG);
const Z_: (Axis, Dir) = (Axis::Z, NEG);
const FACE_A: Rotation = Rotation { x: X, y: Y, z: Z };
const FACE_B: Rotation = Rotation { x: X, y: Z, z: Y_ };
const FACE_C: Rotation = Rotation { x: Z_, y: Y, z: X };
const FACE_D: Rotation = Rotation { x: X, y: Z_, z: Y };
const FACE_E: Rotation = Rotation { x: Z, y: Y, z: X_ };
const FACE_F: Rotation = Rotation { x: X, y: Y_, z: Z_ };

/// Find all four rotations from rotation this around the Z axis.
const fn rotations(face: Rotation) -> [Rotation; 4] {
    const fn flip_direction((axis, dir): (Axis, Dir)) -> (Axis, Dir) {
        (
            axis,
            match dir {
                Dir::Pos => Dir::Neg,
                Dir::Neg => Dir::Pos,
            },
        )
    }
    [
        Rotation {
            x: flip_direction(face.x),
            y: flip_direction(face.y),
            z: face.z,
        },
        Rotation {
            x: flip_direction(face.x),
            y: face.y,
            z: face.z,
        },
        Rotation {
            x: face.x,
            y: flip_direction(face.y),
            z: face.z,
        },
        Rotation {
            x: face.x,
            y: face.y,
            z: face.z,
        },
    ]
}

const ALL_ROTATIONS: [Rotation; 24] = flatten(
    FACE_A,
    [
        rotations(FACE_A),
        rotations(FACE_B),
        rotations(FACE_C),
        rotations(FACE_D),
        rotations(FACE_E),
        rotations(FACE_F),
    ],
);

/// Flattens M arrays of N items into an array of N*M items.
/// Due to limitations around uninitialized memory and const evaluation,
/// requires a stub "default" value, but its choice does not affect
/// the return value at all.
const fn flatten<T: Copy, const N: usize, const M: usize>(
    dflt: T,
    items: [[T; N]; M],
) -> [T; N * M] {
    let mut out = [dflt; N * M];
    let mut m = 0;
    while m < M {
        let mut i = 0;
        while i < N {
            out[i + (m * N)] = items[m][i];
            i += 1;
        }
        m += 1;
    }
    out
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
enum Axis {
    X,
    Y,
    Z,
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
enum Dir {
    Pos,
    Neg,
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
struct Rotation {
    x: (Axis, Dir),
    y: (Axis, Dir),
    z: (Axis, Dir),
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_faces() {
        let distinct_rotations = HashSet::from(ALL_ROTATIONS);
        assert_eq!(distinct_rotations.len(), ALL_ROTATIONS.len(),);
    }
}
