use crate::get_state_populations::Pop;

pub fn merge_pops(pops1: Vec<Pop>, pops2: Vec<Pop>) -> Vec<Pop> {
    let mut new_pops = pops1.clone();
    for pop in pops2 {
        let existing_pop = new_pops.iter_mut().find(|new_pop| {
            new_pop.culture == pop.culture
                && new_pop.religion == pop.religion
                && new_pop.pop_type == pop.pop_type
        });
        match existing_pop {
            Some(existing_pop) => {
                existing_pop.size += pop.size;
            }
            None => {
                new_pops.push(pop);
            }
        }
    }

    new_pops
}
