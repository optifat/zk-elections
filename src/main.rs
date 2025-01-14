use election::election::Election;

mod election;

fn main() {
    let mut election = Election::new(2);

    let peter = "Peter".to_string();
    let veronica = "Veronica".to_string();

    election.add_vote(peter.clone(), 0).unwrap();
    election.add_vote(veronica.clone(), 1).unwrap();

    let peter_vote = election.votes.get(&peter).unwrap();
    println!("Verifying Peter's vote");
    election.circuit.verify(peter_vote.proof.clone()).unwrap();
    println!("{:?}", peter_vote.proof.get_public_inputs_hash());
    println!("Peter's vote is correct");

    let veronica_vote = election.votes.get(&veronica).unwrap();
    println!("Verifying Veronica's vote");
    election
        .circuit
        .verify(veronica_vote.proof.clone())
        .unwrap();
    println!("{:?}", veronica_vote.proof.get_public_inputs_hash());
    println!("Veronica's vote is correct");
}
