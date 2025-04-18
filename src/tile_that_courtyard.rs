type Tile = Vec<(usize, usize)>;

type CourtYard = Vec<Vec<usize>>;

fn show_tiling(tiles: &Vec<Tile>, missing_loc: &(usize, usize), size: usize) {
    let mut yard: CourtYard = vec![vec![0; size]; size];

    for (i, tile) in tiles.iter().enumerate() {
        let t1 = tile[0];
        let t2 = tile[1];
        let t3 = tile[2];

        yard[t1.1][t1.0] = i + 1;
        yard[t2.1][t2.0] = i + 1;
        yard[t3.1][t3.0] = i + 1;
    }

    yard.iter().for_each(|row| println!("{:?}", row));
}

fn please_tile_that_courtyard_please(size: usize, missing_loc: (usize, usize), use_given: bool) {
    let width = 1usize << size;

    let missing_loc = if use_given {
        missing_loc
    } else {
        (
            rand::random::<usize>() % width,
            rand::random::<usize>() % width,
        )
    };

    let section = (0, width - 1, 0, width - 1);
    let mut tiles = Vec::<Vec<(usize, usize)>>::new();
    let can_tile = tile_court_yard(section, missing_loc, &mut tiles);

    if !can_tile {
        println!("Can not tile the court yard");
    } else {
        println!("Can tile the court yard. Bellow are the tillings");
        show_tiling(&tiles, &missing_loc, width);
    }
}

// size is size of the court yard
fn show_section(section: (usize, usize, usize, usize), size: usize) {
    let mut yard: CourtYard = vec![vec![0; size]; size];

    for i in section.0..=section.1 {
        for j in section.2..=section.3 {
            yard[j][i] = 1;
        }
    }

    yard.iter().for_each(|row| println!("{:?}", row));
    println!()
}

fn tile_court_yard(
    section: (usize, usize, usize, usize),
    missing_loc: (usize, usize),
    tiles: &mut Vec<Tile>,
) -> bool {
    // determin sector of missing tile
    let (x_start, x_end, y_start, y_end) = section;

    let size = x_end - x_start + 1;
    let mid_x = x_start + size / 2;
    let mid_y = y_start + size / 2;

    let (m_x, m_y) = missing_loc;

    let sector: usize;

    // clockwise sector labeling
    if m_x < mid_x && m_y < mid_y {
        sector = 0;
    } else if m_x >= mid_x && m_y < mid_y {
        sector = 1;
    } else if m_x >= mid_x && m_y >= mid_y {
        sector = 2;
    } else if m_x < mid_x && m_y >= mid_y {
        sector = 3;
    } else {
        panic!("Unknown sector for missiong location {:?}", missing_loc);
    }

    if size == 2 {
        // place tile in correct orientation based on missing tile
        let mut tile: Tile = vec![
            (x_start, y_start),
            (x_end, y_start),
            (x_end, y_end),
            (x_start, y_end),
        ];
        tile.swap_remove(sector);
        tiles.push(tile);

        return true;
    }

    // place center tile in correct orientation based on missing tile
    let mut tile = vec![
        (mid_x - 1, mid_y - 1),
        (mid_x, mid_y - 1),
        (mid_x, mid_y),
        (mid_x - 1, mid_y),
    ];
    let sections = vec![
        (x_start, mid_x - 1, y_start, mid_y - 1),
        (mid_x, x_end, y_start, mid_y - 1),
        (mid_x, x_end, mid_y, y_end),
        (x_start, mid_x - 1, mid_y, y_end),
    ];

    // remove one tile from other sectors
    // upper left remove bottom right
    // upper right remove bottom left tile
    // lower right remove top left tile
    // lower left remove top right tile
    for i in 0..4 {
        let removed_tile = if sector == i { missing_loc } else { tile[i] };

        if !tile_court_yard(sections[i], removed_tile, tiles) {
            return false;
        }
    }

    tile.swap_remove(sector);
    tiles.push(tile);

    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_please_tile_that_courtyard_please() {
        please_tile_that_courtyard_please(2, (1, 1), false);
    }
}
