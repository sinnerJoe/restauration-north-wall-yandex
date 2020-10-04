use std::io;
use std::vec;
use std::cmp::Ordering;
use std::collections::hash_set::HashSet;
use std::option;
use std::borrow;
struct MatrixSize {
    width: usize,
    xStart: usize,
    height: usize,
    yStart: usize
}


type Line = Vec<u8>;

type Matrix = Vec<Line>; 

struct SolutionStep {
    parentId: Option<usize>,
    x: usize,
    y: usize
} 

impl SolutionStep {
    fn get_id(&self) -> usize {
        self.x + self.y * 2001
    }
}



fn main() {
    
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("SOMETHING WENT WRONG");
    let n: usize = buffer.trim()
        .parse()
        .expect("Parsing didn't work");
    
    let mut matrix: Matrix = vec![];
    matrix.reserve(n as usize);
    for _ in 0..n as usize {
        buffer.clear();
        io::stdin()
        .read_line(&mut buffer)
        .expect("array couldn't be read");
        
        let line: Line = buffer.trim().as_bytes().into_iter().map(|c| c - ('0' as u8)).collect();
        matrix.push(line);
    }

    let wholeSize = MatrixSize {
        width: n,
        height: n,
        yStart: 0,
        xStart: 0,
    };

    let steps = recursive_restoration(None, &wholeSize, &mut matrix);

    // for line in &matrix {
    //     for el in line {
    //         print!("{} ", el);
    //     }
    //     println!("");
    // }

    if(!is_matrix_done(&wholeSize, &matrix)) {
        println!("-1");
        return;
    }

    if let Ok(steps) = steps {
        println!("{}", steps.len());
        let mut displayedSteps = HashSet::new();
        loop {
            let mut availableSteps = vec![];
            for stepId in 0..(&steps).len() {
                if displayedSteps.contains(&steps[stepId].get_id()) {
                    continue;
                }
                match steps[stepId].parentId {
                    None => availableSteps.push(stepId),
                    Some(parentId) => {
                        if displayedSteps.contains(&parentId) {
                            availableSteps.push(stepId);
                        }
                    }
                }
            }

            // for s in &availableSteps {
            //     print!("{} ", s);
            // }
            // println!();

            availableSteps.sort_by(|id1, id2| {
                let step1 = &steps[*id1];
                let step2 = &steps[*id2];
                let comp_x =  step1.y.to_string().cmp(&step2.y.to_string());
                if Ordering::Equal != comp_x {
                    return comp_x;
                }
                step1.x.to_string().cmp(&step2.x.to_string())
            });

            for stepId in availableSteps {
                displayedSteps.insert(steps[stepId].get_id());
                println!("{}\n{}", steps[stepId].y + 1, steps[stepId].x + 1);
            }

            if(displayedSteps.len() == steps.len()) {
                break;
            }
        }
    } else {
        println!("-1");
    }

}

fn is_matrix_done(size: &MatrixSize, matrix: &Matrix) -> bool {
    for row in size.yStart..size.yStart+size.height {
        for col in size.xStart..size.xStart+size.width {
            if matrix[row][col] == 1 {
                return false;
            }
        }
    }
    return true;
}

fn undo_changes(size: &MatrixSize, matrix: &mut Matrix) {
    for row in size.yStart..size.yStart+size.height {
        for col in size.xStart..size.xStart+size.width {
            if matrix[row][col] == 2 {
                matrix[row][col] = 1;
            }
        }
    }
}

fn apply_restoration(x: usize, y: usize, parentStep: Option<usize>, size: &MatrixSize, matrix: &mut Matrix) -> Result<Vec<SolutionStep>, ()> {
    for row in size.yStart..size.yStart+size.height {
        matrix[row][x] = 2;
    }
    for col in size.xStart..size.xStart+size.width {
        matrix[y][col] = 2;
    }
    let missingLeft = x == size.xStart;
    let missingRight = x == size.xStart+size.width-1;
    let missingTop = y == size.yStart;
    let missingBottom = y == size.yStart + size.height - 1;
    let mut result: Vec<SolutionStep> = vec![SolutionStep{parentId: parentStep, x, y}];
    let currentStep = result[0].get_id();
    let mut firstSteps: Vec<SolutionStep> = vec![];
    let mut separateHead = |quadrantData: Result<Vec<SolutionStep>, ()> | -> bool {
        if let Ok(quadrantData) = quadrantData {
            if quadrantData.len() > 0 {
                // firstSteps.push(quadrantData[0]);
                let mut iter = quadrantData.into_iter();
                // iter.next();
                result.extend(iter);
            }
            return true;
        }
        return false;
    };
    if !missingLeft && !missingTop {
        let leftTop = recursive_restoration(Some(currentStep), &MatrixSize {
            height: y - size.yStart,
            width: x - size.xStart,
            ..*size
        }, matrix);

        if !separateHead(leftTop) {
            return Err(());
        }
    }
    if !missingLeft && !missingBottom {
        let data = recursive_restoration(Some(currentStep), &MatrixSize{
            height: size.height - (y - size.yStart) - 1,
            width: x - size.xStart,
            yStart: y+1,
            ..*size
        }, matrix);
        if !separateHead(data) {
            return Err(());
        }
    }
    if !missingRight && !missingTop {
        let data = recursive_restoration(Some(currentStep),&MatrixSize {
            height: y - size.yStart,
            width: size.width - (x - size.xStart) - 1,
            xStart: x + 1,
            ..*size
        }, matrix);
        if !separateHead(data) {
            return Err(());
        }
    }

    if !missingRight && !missingBottom {
        let data = recursive_restoration(Some(currentStep), &MatrixSize {
            height: size.height - (y-size.yStart) - 1,
            width: size.width - (x - size.xStart) - 1,
            xStart: x+1,
            yStart: y+1,
        }, matrix);
        if !separateHead(data) {
            return Err(());
        }
    }

    if !is_matrix_done(size, matrix) {
        undo_changes(size, matrix);
        return Err(());
    }

    // firstSteps.sort_by(|(row, col), (row2, col2)| {
    //     match row.cmp(row2) {
    //         Ordering::Equal => col.cmp(col2),
    //         rest => rest
    //     }
    // });

    // firstSteps.extend(result);
    // firstSteps.insert(0, (y, x));
    // Ok(firstSteps)
    Ok(result)
}

type Path = Vec<SolutionStep>;

fn compare_steps(step1: &SolutionStep, step2: &SolutionStep) -> Ordering {
    let comp_x =  step1.y.to_string().cmp(&step2.y.to_string());
    if Ordering::Equal != comp_x {
        return comp_x;
    }
    step1.x.to_string().cmp(&step2.x.to_string())
}

fn compare_paths(path1: &Path, path2: &Path) -> Ordering {
    match path1.len().cmp(&path2.len()) {
        Ordering::Equal => {
            for i in 0..path1.len() {
                let cmp_result = compare_steps(&path1[i], &path2[i]);
                if let Ordering::Equal = cmp_result {
                    continue;
                } else {
                    return cmp_result;
                }
            }
            Ordering::Equal
        },
        rest => rest
    }
}

fn recursive_restoration(parentId: Option<usize>, size: &MatrixSize, matrix: &mut Matrix) -> Result<Vec<SolutionStep>, ()> {
    match find_cross(size, matrix) {
        Ok((rows , columns)) => {
            let mut shortest: Result<Vec<SolutionStep>, ()> = Err(());
            for y in &rows {
                for x in &columns {
                    if let Ok(result) = apply_restoration(*x, *y, parentId, size, matrix) {
                        let chosen = match shortest {
                            Err(_) => result,
                            Ok(path) => {
                                match compare_paths(&path, &result) {
                                    Ordering::Greater => result,
                                    _ => path
                                }
                            }
                        };
                        shortest = Ok(chosen);
                    }
                }
            }
            shortest
        }
        Err(_) => Ok(vec![])
    }
}


fn find_cross(size: &MatrixSize, matrix: &Matrix) -> Result<(Vec<usize>, Vec<usize>), ()> {
    let mut crossRows: Vec<usize> = vec![];
    let mut crossColumns: Vec<usize> = vec![];
    for col in size.xStart..size.xStart+size.width {
        if matrix[size.yStart][col] == 1 {
            let mut allOnes = true;
            for row in size.yStart..size.yStart + size.height {
                if matrix[row][col] != 1 {
                    allOnes = false;
                    break;
                }
            }
            if allOnes {
                crossColumns.push(col);
            }
        }
    }

    for row in size.yStart..size.yStart+size.height {
        if matrix[row][size.xStart] == 1 {
            let mut allOnes = true;
            for col in size.xStart..size.xStart + size.width {
                if matrix[row][col] != 1 {
                    allOnes = false;
                    break;
                }
            }
            if allOnes {
                crossRows.push(row)
            }
        }
    }

    if crossColumns.len() > 0 && crossRows.len() > 0 {
        return Ok((crossRows, crossColumns))
    } else {
        Err(())   
    }
}