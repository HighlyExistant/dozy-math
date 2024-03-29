use drowsed_math::FVec3;

pub fn create_cube(ofst: FVec3) -> (Vec<FVec3>, Vec<u32>) {
    (vec![
        FVec3 {x: -0.5, y: -0.5, z: -0.5    } + ofst,
        FVec3 {x: -0.5, y: 0.5,  z: 0.5     } + ofst,
        FVec3 {x: -0.5, y: -0.5, z: 0.5     } + ofst,
        FVec3 {x: -0.5, y: 0.5,  z: -0.5    } + ofst,
   
        FVec3 {x: 0.5,  y: -0.5, z: -0.5    } + ofst,
        FVec3 {x: 0.5,  y: 0.5,  z: 0.5     } + ofst,
        FVec3 {x: 0.5,  y: -0.5, z: 0.5     } + ofst,
        FVec3 {x: 0.5,  y: 0.5,  z: -0.5    } + ofst,
   
        FVec3 {x: -0.5, y: -0.5, z: -0.5    } + ofst,
        FVec3 {x: 0.5,  y: -0.5, z: 0.5     } + ofst,
        FVec3 {x: -0.5, y: -0.5, z: 0.5     } + ofst,
        FVec3 {x: 0.5,  y: -0.5, z: -0.5    } + ofst,
   
        FVec3 {x: -0.5, y: 0.5,  z: -0.5    } + ofst,
        FVec3 {x: 0.5,  y: 0.5,  z: 0.5     } + ofst,
        FVec3 {x: -0.5, y: 0.5,  z: 0.5     } + ofst,
        FVec3 {x: 0.5,  y: 0.5,  z: -0.5    } + ofst,
   
        FVec3 {x: -0.5, y: -0.5,  z: 0.5    } + ofst,
        FVec3 {x: 0.5,  y: 0.5,   z: 0.5    } + ofst,
        FVec3 {x: -0.5, y: 0.5,   z: 0.5    } + ofst,
        FVec3 {x: 0.5,  y: -0.5,  z: 0.5    } + ofst,
   
        FVec3 {x: -0.5, y: -0.5,  z: -0.5   } + ofst,
        FVec3 {x: 0.5,  y: 0.5,   z: -0.5   } + ofst,
        FVec3 {x: -0.5, y: 0.5,   z: -0.5   } + ofst,
        FVec3 {x: 0.5,  y: -0.5,  z: -0.5   } + ofst
    ], vec![0u32,  1,  2,  0,  3,  1,  4,  5,  6,  4,  7,  5,  8,  9,  10, 8,  11, 9,
    12, 13, 14, 12, 15, 13, 16, 17, 18, 16, 19, 17, 20, 21, 22, 20, 23, 21] )
}