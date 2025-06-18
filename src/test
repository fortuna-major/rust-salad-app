use pretty_assertions::assert_eq;
use rstest::{fixture, rstest};

use testing::attractions::{MovieTheater, Museum};
use testing::managment::VenueManagment;

#[fixture]
fn museum_with_three_paintings() -> Museum {
    let mut museum = Museum::new();
    museum.buy_paintings("Mona Lisa");
    museum.buy_paintings("Starry night");
    museum.buy_paintings("Bllerinas");
    museum
}

#[fixture]
fn museum_managment(museum_with_three_paintings: Museum) -> VenueManagment<Museum> {
    VenueManagment::new(museum_with_three_paintings)
}

#[fixture]
fn movie_theater_managment() -> VenueManagment<MovieTheater> {
    let mut movie_theater = MovieTheater::new();
    movie_theater.add_movie("Love, Eat, Pray");
    VenueManagment::new(movie_theater)
}

#[rstest]
fn venue_managment_interacts_with_museum_venue(museum_with_three_paintings: Museum) {
    let mut vn_mgmt = VenueManagment::new(museum_with_three_paintings);
    vn_mgmt.make_money();

    assert_eq!(vn_mgmt.venue.paintings.len(), 3);
    assert_eq!(vn_mgmt.venue.revenue, 35);
}
#[rstest]
fn venue_managment_interacts_with_movie_venue(
    mut movie_theater_managment: VenueManagment<MovieTheater>,
) {
    movie_theater_managment.make_money();
    assert_eq!(movie_theater_managment.venue.sales, 15);
}
