use serde::Deserialize;

const MAX_X: u32 = 320;
const MAX_Z: u32 = 120;

pub struct Topology {
    pub vertices: Vec<[f32; 3]>,
    pub indices: Vec<u32>,
}

pub fn get_topology() -> Topology {
    let cords = calc_cords();
    let heights = fetch_heights(&cords);

    let vertices = cords
        .iter()
        .zip(heights)
        .map(|(cord, height)| [cord.x as f32, height / 200.0, cord.z as f32])
        .collect();

    let mut indices = Vec::new();
    for i in 0..(MAX_X * MAX_Z - MAX_X) {
        if i % MAX_X == MAX_X - 1 {
            continue;
        }

        indices.push(i);
        indices.push(i + MAX_X);
        indices.push(i + 1);

        indices.push(i + 1);
        indices.push(i + MAX_X);
        indices.push(i + MAX_X + 1);
    }

    Topology { vertices, indices }
}

struct Cord {
    x: u32,
    z: u32,
    longitude: f32,
    latitude: f32,
}

fn calc_cords() -> Vec<Cord> {
    let mut cords = vec![];

    const MIN_LATITUDE: f32 = 46.5;
    const MAX_LATITUDE: f32 = 49.0;

    const MIN_LONGITUDE: f32 = 9.5;
    const MAX_LONGITUDE: f32 = 17.5;

    for z in 0..MAX_Z {
        for x in 0..MAX_X {
            let longitude =
                MIN_LONGITUDE + (MAX_LONGITUDE - MIN_LONGITUDE) * (x as f32 / MAX_X as f32);
            let latitude =
                MIN_LATITUDE + (MAX_LATITUDE - MIN_LATITUDE) * (1. - (z as f32 / MAX_Z as f32));

            cords.push(Cord {
                x,
                z,
                latitude,
                longitude,
            });
        }
    }

    cords
}

#[derive(Deserialize)]
struct PointsResponse {
    results: Vec<PointResponse>,
}

#[derive(Deserialize)]
struct PointResponse {
    elevation: f32,
}

fn fetch_heights(cords: &Vec<Cord>) -> Vec<f32> {
    cords
        .chunks(100)
        .flat_map(|cords| {
            let search = cords
                .iter()
                .map(|cord| format!("{},{}", cord.latitude, cord.longitude))
                .collect::<Vec<String>>()
                .join("|");

            let response = ureq::get(&format!(
                "http://localhost:5000/v1/eudem25m?locations={}",
                search
            ))
            .call()
            .unwrap();

            let response =
                serde_json::from_str::<PointsResponse>(&response.into_string().unwrap()).unwrap();

            response
                .results
                .iter()
                .map(|response| response.elevation)
                .collect::<Vec<f32>>()
        })
        .collect()
}
