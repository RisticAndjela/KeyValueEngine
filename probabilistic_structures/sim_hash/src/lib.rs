pub mod sim_hash;
pub mod serialization;

#[cfg(test)]
mod tests {
    use sim_hash::{sim_hash,hamming_distance};
    use crate::sim_hash;

    #[test]
    fn equals() {
        let long_text: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit.
        Vestibulum vehicula ex eu justo malesuada, at sollicitudin erat elementum.
        Vivamus volutpat, erat id tempus facilisis, nisl justo facilisis orci, et cursus
        quam sem sit amet sapien. Donec non metus et nisl dictum facilisis. Duis luctus
        eleifend elit, eu blandit ipsum fermentum at.";

        let sim=sim_hash(long_text);
        assert!(hamming_distance(sim,sim)==0);
    }
    #[test]
    fn longer(){
        let long_text: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit.
        Vestibulum vehicula ex eu justo malesuada, at sollicitudin erat elementum.
        Vivamus volutpat, erat id tempus facilisis, nisl justo facilisis orci, et cursus
        quam sem sit amet sapien. Donec non metus et nisl dictum facilisis.";
        let longer_text: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit.
        Vestibulum vehicula ex eu justo malesuada, at sollicitudin erat elementum.
        Vivamus volutpat, erat id tempus facilisis, nisl justo facilisis orci, et cursus
        quam sem sit amet sapien. Donec non metus et nisl dictum facilisis. Duis luctus
        eleifend elit, eu blandit ipsum fermentum at.";
        let sim1=sim_hash(long_text);
        let sim2=sim_hash(longer_text);
        assert!(hamming_distance(sim1,sim2)>0);
    }
}
