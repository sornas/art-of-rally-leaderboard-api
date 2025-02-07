use art_of_rally_leaderboard_api::{
    Area, Direction, Filter, Group, Leaderboard, Platform, Response, Weather,
};
use curl::easy::{Easy2 as Curl, Handler, WriteError};

#[derive(Default)]
struct Collector(Vec<u8>);

impl Handler for Collector {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        self.0.extend_from_slice(data);
        Ok(data.len())
    }
}

fn main() {
    let mut curl = Curl::new(Collector::default());
    let leaderboard = Leaderboard {
        area: Area::Finland,
        stage: 1,
        direction: Direction::Forward,
        weather: Weather::Dry,
        group: Group::Sixties,
        filter: Filter::Friends,
        platform: Platform::Steam,
    };
    let url = leaderboard.as_url(76561198230518420, &[76561198087789780, 76561198062269100]);
    curl.get(true).unwrap();
    curl.url(&url).unwrap();
    curl.perform().unwrap();
    assert_eq!(curl.response_code().unwrap(), 200);
    let body = String::from_utf8(curl.get_ref().0.clone()).unwrap();
    let resp: Response = serde_json::from_str(&body).unwrap();
    dbg!(&resp);
}
