fn move_cups(index: usize, mut cup_list: Vec<i32>) -> (usize,Vec<i32>) {
    let current_cup = cup_list[index];

    println!("cups: {:?}, at {}",cup_list, current_cup); 
    // logic to get the next 3 items
    let mut removed_cups = Vec::new();
    
    let next_three = index + 3; 
    if next_three < cup_list.len() {
        // slice as normal
        removed_cups = cup_list.drain(index+1 ..=next_three).collect();
    } else if index < (cup_list.len() -1 )  {
        // loop around
        let mut top : Vec<i32> = cup_list.drain(index+1..).collect();
        let mut bot : Vec<i32> = cup_list.drain(..3 -top.len()).collect();
        removed_cups.append(&mut top);
        removed_cups.append(&mut bot);
    } else {
        // index is last item, get first three
        removed_cups = cup_list.drain(..3).collect();
    } 
    println!("pick up: {:?}",removed_cups);     
    
    let mut dest_cup = current_cup  -1 ;
    while removed_cups.contains(&dest_cup) || dest_cup == 0 {
        dest_cup -=1;
        if dest_cup < 1  {
            dest_cup = 9;
        }
    } 
    let dest_pos = cup_list.iter().position(|&x| x == dest_cup).unwrap();
    println!("destination : {} (index: {})",dest_cup,dest_pos);
    cup_list.splice(dest_pos+1..dest_pos+1, removed_cups);
    
    let next_index = cup_list.iter().position(|&x| x == current_cup).unwrap() + 1 ;
    (next_index % (cup_list.len()), cup_list)
}


pub fn run() {
    //let mut cups = vec![3,8,9,1,2,5,4,6,7];
    let mut cups = vec![1,5,7,6,2,3,9,8,4];
    let mut index = 0;
    for i in 0 .. 100 {
        println!("\n-- move {} --",i+1);
        let (a,b) = move_cups(index, cups);
        index = a;
        cups = b;
    }
    println!("\n-- final --\ncups: {:?} at {}",cups,cups[index]);
}
