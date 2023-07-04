pub trait Day {
    fn part1(&self);
    fn part2(&self);
}

impl Day for fn() {
    fn part1(&self) {
        self();
    }

    fn part2(&self) {
        self();
    }
}