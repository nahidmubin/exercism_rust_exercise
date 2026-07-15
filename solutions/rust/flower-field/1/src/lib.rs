
pub fn annotate(garden: &[&str]) -> Vec<String> {
    //Initialize the flower field. Data will added here subsequently from garden. 
    let mut flower_field: Vec<String> = Vec::new();

    //Get the height and width of the garden to iterate over the garden's cells
    let height = garden.len();
    let width = match garden.get(0){
        Some(val) => val.len(),
        None => 0
    };

    //iterate over each row of the garden
    for i in 0..height {
        //for each row of garden. Add a row in the flower field which a String with same width of the garden
        flower_field.push(String::with_capacity(width));
        
        //As flowers counts in depends on the surrounding cells. Need to get previous, current and next row to check
        //for the inexistence of previous or next row. Initialize a virtual empty row whic is Vec::new()
        let prev_row: Vec<&u8> = if i > 0 {garden[i-1].as_bytes().iter().collect()} else {Vec::new()}; //if i is zero (i-1) will have a overflow
        let current_row: Vec<&u8> = garden[i].as_bytes().iter().collect();
        let next_row = garden.get(i+1); // if i+1 is greater than the widht it will cause index error
        let next_row:Vec<&u8> = match next_row {
            Some(val) => val.as_bytes().iter().collect(),
            None => Vec::new()
        };
        
        //iterate over each cell of the garden
        for j in 0..width{
            //initialize a flower counter for each cell
            let mut flower_count: u8 = 0;

            // if a cell in garden have flower, so the flower field will have
            if *current_row[j] == '*' as u8{
                flower_field[i].push_str("*");
            }
            else {
                //check for each cells surrounding flowers. For each flower, increase the counter
                if j > 0 && prev_row.get(j-1).is_some() && *prev_row[j-1] == '*' as u8 {
                    flower_count += 1;
                }
                if prev_row.get(j).is_some() && *prev_row[j] == '*' as u8 {
                    flower_count += 1;
                }
                if prev_row.get(j+1).is_some() && *prev_row[j+1] == '*' as u8 {
                    flower_count += 1;
                }

                if j > 0 && current_row.get(j-1).is_some() && *current_row[j-1] == '*' as u8 {
                    flower_count += 1;
                }
                if current_row.get(j+1).is_some() && *current_row[j+1] == '*' as u8 {
                    flower_count += 1;
                }

                if j > 0 && next_row.get(j-1).is_some() && *next_row[j-1] == '*' as u8 {
                    flower_count += 1;
                }
                if next_row.get(j).is_some() && *next_row[j] == '*' as u8 {
                    flower_count += 1;
                }
                if next_row.get(j+1).is_some() && *next_row[j+1] == '*' as u8 {
                    flower_count += 1;
                }

                // If a cell have no surrounding flowers, Keep the cell empty in the flower field.
                //else add the counted flower number in the cell
                if flower_count == 0{
                    flower_field[i].push_str(" ");
                }
                else {
                    flower_field[i].push_str(&flower_count.to_string());
                }
            }
        }
    }


    flower_field
}
