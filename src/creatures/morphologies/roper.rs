pub fn roper_corpus() -> crate::creatures::Appendage {
    let mut roper = crate::prelude::MorphologyBuilder::new("Central Body");
    // The Roper's teeth are stone like and can crush bone with ease.
    roper.add_appendage("Central Body", "Mouth");
    roper.add_appendage("Central Body", "Eye");
    // Multiple long, flexible tentacles for grasping prey. Can sap the energy of anything they wrap around.
    roper.add_appendage("Central Body", "Tentacle 1");
    roper.add_appendage("Central Body", "Tentacle 2");
    roper.add_appendage("Central Body", "Tentacle 3");
    roper.add_appendage("Central Body", "Tentacle 4");
    roper.add_appendage("Central Body", "Tentacle 5");
    roper.add_appendage("Central Body", "Tentacle 6");
    // Tiny hair-like structures for movement at the base of the body.
    roper.add_appendage("Central Body", "Cilia");

    roper.build()
}
